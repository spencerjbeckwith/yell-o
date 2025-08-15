pub mod config;

use warp::Filter;
use warp::hyper::Method;
use std::collections::HashMap;
use std::sync::Arc;
use std::path::Path;
use reqwest;
use serde_json;
use rodio;
use rodio::{Decoder, OutputStreamBuilder, OutputStream, Sink};
use std::io::Cursor;
use warp::reply::{with_status, json};
use warp::http::StatusCode;
use log::info;

use crate::config::Config;

const PROD_FILES_PATH: &str = "/etc/yell-o/ui";
const DEV_STATIC_PATH: &str = "./ui/dist";

#[derive(Clone)]
struct EndpointData {
    config: Config,
    client: reqwest::Client,
}

// To be received
#[derive(serde::Deserialize)]
struct SpeakRequest {
    text: String,
    voice_id: String,
}

// To send to ElevenLabs
#[derive(serde::Serialize)]
struct TextToSpeechRequestBody {
    text: String,
}

struct WithAudio {
    _stream: OutputStream, // Stream has to be passed with sink, so it isn't dropped
    sink: Sink,
}

pub fn routes(config: Config) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Pass in this struct into each endpoint handler
    let data = Arc::new(EndpointData {
        config,
        client: reqwest::Client::new(),
    });

    let output_stream = OutputStreamBuilder::open_default_stream().expect("unable to open default stream");
    let sink = Sink::connect_new(&output_stream.mixer());
    let audio = Arc::new(WithAudio {
        _stream: output_stream,
        sink,
    });

    let with_data = warp::any().map(move || data.clone());
    let with_data_2 = with_data.clone();
    let with_audio = warp::any().map(move || Arc::clone(&audio));

    let cors = warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["Access-Control-Request-Headers", "Access-Control-Request-Method", "Origin", "Accept", "Content-Type", "User-Agent"])
        .allow_methods(&[Method::GET, Method::POST, Method::OPTIONS]);

    // Serve the UI as static files
    let mut static_path = Path::new(PROD_FILES_PATH);
    if !static_path.is_dir() {
        static_path = Path::new(DEV_STATIC_PATH);
        if !static_path.is_dir() {
            panic!("unable to locate either {} or {}, one of which must contain yell-o's built ui files", PROD_FILES_PATH, DEV_STATIC_PATH);
        }
    }
    let assets_path = static_path.join(Path::new("assets"));
    info!("Static directory: {}", static_path.to_str().unwrap_or(""));
    let get_static = warp::path::end()
        .and(warp::fs::dir(static_path));
    let get_assets = warp::path("assets")
        .and(warp::fs::dir(assets_path));

    // GET /voices
    let get_voices = warp::path("voices")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_data)
        .and_then(handle_get_voices);

    // POST /speak
    let post_speak = warp::path("speak")
        .and(warp::post())
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .and(with_data_2)
        .and(with_audio)
        .and_then(handle_post_speak);

    get_voices.or(post_speak).or(get_assets).or(get_static).with(&cors)
}

async fn handle_get_voices(
    query: HashMap<String, String>,
    data: Arc<EndpointData>,
) -> Result<impl warp::Reply, warp::Rejection> {

    // Set up our query parameters for our request
    let mut request_query = HashMap::new();
    request_query.insert("page_size", String::from("100"));
    if query.contains_key("category") {
        request_query.insert("category", query.get("category").unwrap().to_string());
    }
    if query.contains_key("search") {
        request_query.insert("search", query.get("search").unwrap().to_string());
    }

    // Make request to list voices
    let result = data.client
        .get("https://api.elevenlabs.io/v2/voices")
        .query(&request_query)
        .header("xi-api-key", &data.config.elevenlabs_api_key)
        .send()
        .await;

    match result {
        Ok(response) => {
            let status = response.status();

            // Try to parse the body as JSON
            match response.json::<serde_json::Value>().await {
                Ok(json_body) => {
                    if status.is_success() {
                        // Success! Forward our response directly to the client.
                        Ok(with_status(json(&json_body), status))
                    } else {
                        // API request went through, but wasn't successful.
                        let message = json_body
                            .get("detail")
                            .and_then(|detail| detail.get("message"))
                            .and_then(|message| message.as_str())
                            .unwrap_or("Unknown error!");
                        get_error(
                            status,
                            format!("ElevenLabs API error: {}", message),
                        )
                    }
                }
                Err(e) => {
                    // Failed to parse the response
                    get_error(
                        status,
                        format!("Failed to parse ElevenLabs API response: {}", e)
                    )
                }
            }
        }
        Err(e) => {
            // Failed to make the request altogether
            get_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to request ElevenLabs API: {}", e)
            )
        }
    }
}

async fn handle_post_speak(
    request: SpeakRequest,
    data: Arc<EndpointData>,
    audio: Arc<WithAudio>,
) -> Result<impl warp::Reply, warp::Rejection> {

    info!("Voice ID {} to say '{}'", request.voice_id, request.text);

    // Make the request to stream the audio
    let result = data.client
        .post(format!("https://api.elevenlabs.io/v1/text-to-speech/{}", request.voice_id))
        .json(&TextToSpeechRequestBody {
            text: request.text,
        })
        .header("xi-api-key", &data.config.elevenlabs_api_key)
        .send()
        .await;

    match result {
        Ok(response) => {
            let status = response.status();
            if status.is_success() {
                // Play our audio
                let bytes = response.bytes().await;
                match bytes {
                    Ok(bytes) => {
                        let source = Decoder::new(Cursor::new(bytes));
                        match source {
                            Ok(source) => {
                                audio.sink.append(source);

                                // Send our success response
                                let confirmed = serde_json::json!({
                                    "message": "done!",
                                });
                                Ok(with_status(json(&confirmed), status))
                            }
                            Err(e) => {
                                get_error(
                                    StatusCode::INTERNAL_SERVER_ERROR,
                                    format!("Failed to play returned audio bytes: {}", e),
                                )
                            }
                        }
                    }
                    Err(e) => {
                        get_error(
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to process voice response bytes: {}", e),
                        )
                    }
                }
            } else {
                // API request went through, but wasn't successful.
                let response_json = response
                    .json::<serde_json::Value>()
                    .await
                    .unwrap_or(serde_json::json!({"detail": { "message": "Unable to process JSON response!" }}));
                let message = response_json
                    .get("detail")
                    .and_then(|detail| detail.get("message"))
                    .and_then(|message| message.as_str())
                    .unwrap_or("Unknown error!");
                get_error(
                    status,
                    format!("ElevenLabs API error: {}", message),
                )
            }
        }
        Err(e) => {
            // Failed to make the request altogether
            get_error(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to request ElevenLabs API: {}", e),
            )
        }
    }
}

fn get_error(status: StatusCode, message: impl Into<String>) -> Result<warp::reply::WithStatus<warp::reply::Json>, warp::Rejection> {
    let error_body = serde_json::json!({
        "status": status.as_u16(),
        "error": message.into(),
    });
    Ok(with_status(json(&error_body), status))
}