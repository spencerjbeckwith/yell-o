import pytest
from api.app import create_app
from flask import Flask
from flask.testing import FlaskClient

@pytest.fixture()
def app(mocker):
    mock_client = mocker.MagicMock()
    return create_app(client=mock_client, check_ui=False)

@pytest.fixture()
def client(app: Flask):
    return app.test_client()

def test_get_voices_invalid_category(client: FlaskClient):
    r = client.get("/voices?category=asdf")
    assert r.status_code == 400

def test_get_voices_with_search(client: FlaskClient):
    r = client.get("/voices?search=american")
    assert r.status_code == 200

def test_post_speak_bad_content_type(client: FlaskClient):
    r = client.post("/speak", headers={
        "Content-Type": "application/x-www-form-urlencoded",
    })
    assert r.status_code == 400

def test_post_speak_missing_params(client: FlaskClient):
    r = client.post("/speak", json={})
    assert r.status_code == 400

def test_post_speak(client: FlaskClient):
    r = client.post("/speak", json={
        "text": "text",
        "voice_id": "voice_id",
    })
    assert r.status_code == 200