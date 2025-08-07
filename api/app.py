from flask import Flask, request
from elevenlabs import play
from elevenlabs.client import ElevenLabs
from elevenlabs.core import ApiError
from dotenv import load_dotenv
from werkzeug.exceptions import HTTPException, BadRequest
import os
import traceback

def create_app():
    load_dotenv()

    # Validate config
    elevenlabs_api_key = os.getenv("ELEVENLABS_API_KEY")
    if elevenlabs_api_key is None:
        raise Exception("ELEVENLABS_API_KEY environment variable must be set!")

    # Create clients
    app = Flask(__name__)
    client = ElevenLabs(
        api_key=elevenlabs_api_key,
    )

    ### Endpoints ###

    # GET /voices
    @app.route("/voices")
    def get_voices():
        # raise Exception("problem1")
        # Query param "category" must be one of "premade", "cloned", "generated", "professional", or not defined
        category = request.args["category"] if "category" in request.args else None
        if category is not None and category not in ["premade", "cloned", "generated", "professional"]:
            raise BadRequest("'category' parameter must be one of 'premade', 'cloned', 'generated', or 'professional'!")
        search = request.args["search"] if "search" in request.args else None
        return client.voices.search(
            category=category,
            search=search,
            page_size=100,
        ).dict()

    # POST /speak
    @app.route("/speak", methods=["POST"])
    def post_speak():
        if "application/json" not in request.content_type:
            raise BadRequest("Request body must be application/json!")
        text = request.json["text"] if "text" in request.json else None
        voice_id = request.json["voice_id"] if "voice_id" in request.json else None
        if text is None or voice_id is None:
            raise BadRequest("Request body must include 'text' and 'voice_id' properties!")
        
        audio_stream = client.text_to_speech.stream(
            text=text,
            voice_id=voice_id,
        )
        play(audio_stream)
        return {
            "message": "done!",
        }

    ### Error Handlers ###
    @app.errorhandler(ApiError)
    def handle_elevenlabs_error(e):
        response = {
            "status": e.status_code,
            "error": f"ElevenLabs API Error: {e.body['detail']['status']}: {e.body['detail']['message']}",
        }
        if app.debug:
            response["traceback"] = traceback.format_exc()
        return response, e.status_code

    @app.errorhandler(Exception)
    def handle_exception(e):
        status = 500
        if isinstance(e, HTTPException):
            status = e.code
        response = {
            "status": status,
            "error": str(e),
        }
        if app.debug:
            response["traceback"] = traceback.format_exc()
        return response, status

    # Done!
    return app