# yell-o

Alternatively, the annoy-the-shit-outta-Kevin machine. This is an application to run on your PC (or Raspberry PI) that'll allow you to send voice AI messages. As long as they have internet access and speakers (and the application installed, duh) you'll get to hear whatever nonsense everybody wants to say to you, without the exhausting physical activity of walking to the other room.

Each running application is a Flask app that serves a static React frontend. This frontend makes it easy to submit text, which is sent to the API, converted into AI sound data, then played on the host machine.

## Installation/Configuration

Must have Poetry>2.1 installed. If you have a lower version, upgrade first via `pipx upgrade poetry` or `poetry self upgrade`.

`poetry install`

Then create a `.env` file in the repository root with the following variables:

### Required Variables

- None yet!

### Optional Variables

- `HOST`: Hostname to use for the Flask server.
- `PORT`: Port to use for the Flask server.

### Running

`poetry run start` will run the app in *production* mode.

`poetry run dev` will run the app in *development* mode, with Flask debug and hot-reload enabled on code change.
