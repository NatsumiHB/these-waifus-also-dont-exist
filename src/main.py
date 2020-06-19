import os

import waitress
from flask import Flask, abort

import templates
import util

app = Flask("these-waifus-also-dont-exist", static_folder="./img", static_url_path="/img/")
base_url = os.getenv("BASE_URL")


# Show the custom 404 page for 404s
@app.errorhandler(404)
def page_not_found(e):
    return templates.not_found, 404


# Return a website with a random waifu
@app.route("/", methods=["GET"])
def random_image():
    waifu_id = util.get_image()
    return templates.return_html(waifu_id)


# Root but with a specified ID
@app.route("/<waifu_id>", methods=["GET"])
def get_image_by_id(waifu_id):
    if util.validate_id(waifu_id):
        return templates.return_html(waifu_id, embed=True)
    else:
        abort(404)


# Return a random URL
@app.route("/get_random_url", methods=["GET"])
def random_image_url():
    return f"{base_url}/img/seed{util.get_image():0>4}.png"


app.register_error_handler(404, page_not_found)
waitress.serve(app, host="0.0.0.0", port=5002)
