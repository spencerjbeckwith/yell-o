from api.app import create_app
import os
from dotenv import load_dotenv

def dev():
    load_dotenv()
    app = create_app()
    app.run(host=os.getenv("HOST"), port=os.getenv("PORT"), debug=True)

def prod():
    load_dotenv()
    # TODO: implement production server - hook this up to a WSGI app somehow?
    raise NotImplementedError("Production mode is not implemented!")