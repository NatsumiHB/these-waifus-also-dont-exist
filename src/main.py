import os

import waitress
from flask import Flask, abort

import templates
import util

api = Flask("these-waifus-also-dont-exist", static_url_path="/img", static_folder="./img/")
base_url = os.getenv("BASE_URL")


# Show the custom 404 page for 404s
@api.errorhandler(404)
def page_not_found(e):
    return templates.not_found, 404


# Return a website with a random waifu
@api.route("/", methods=["GET"])
def random_image():
    waifu_id = util.get_image()
    return templates.return_html(waifu_id)


# Root but with a specified ID
@api.route("/<waifu_id>", methods=["GET"])
def get_image_by_id(waifu_id):
    if util.validate_id(waifu_id):
        return templates.return_html(waifu_id, embed=True)
    else:
        abort(404)


# Return a random URL
@api.route("/get_random_url", methods=["GET"])
def random_image_url():
    return f"{base_url}/seed{util.get_image():0>4}.png"


api.register_error_handler(404, page_not_found)
waitress.serve(api, host="0.0.0.0", port=5002)
