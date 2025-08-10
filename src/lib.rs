pub mod config;

use warp::Filter;
use std::collections::HashMap;
use std::sync::Arc;
use reqwest;
use serde_json;

use crate::config::Config;

#[derive(Clone)]
struct EndpointData {
    config: Config,
    client: reqwest::Client,
}

pub fn routes(config: Config) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    // Pass in this struct into each endpoint handler
    let data = Arc::new(EndpointData {
        config,
        client: reqwest::Client::new(),
    });
    let with_data = warp::any().map(move || data.clone());

    // GET /voices
    let get_voices = warp::path("voices")
        .and(warp::get())
        .and(warp::query::<HashMap<String, String>>())
        .and(with_data)
        .and_then(handle_get_voices);

    // POST /speak
    let post_speak = warp::path("speak")
        .and(warp::post())
        .map(|| warp::reply::html("Speak!")); // TODO implement this

    get_voices.or(post_speak)
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
                        Ok(warp::reply::with_status(warp::reply::json(&json_body), status))
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
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to request ElevenLabs API: {}", e)
            )
        }
    }
}

fn get_error(status: warp::http::StatusCode, message: impl Into<String>) -> Result<warp::reply::WithStatus<warp::reply::Json>, warp::Rejection> {
    let error_body = serde_json::json!({
        "status": status.as_u16(),
        "error": message.into(),
    });
    Ok(warp::reply::with_status(warp::reply::json(&error_body), status))
}