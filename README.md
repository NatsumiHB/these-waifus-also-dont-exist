# these-waifus-also-dont-exist
I used the dataset of ThisWaifuDoesNotExist and generated ten thousand images on my own. The website automatically returns a random image. 

## Usage
You will need to download and provide the images yourself. You can get them [here](https://docs.google.com/uc?export=download&id=1PnHPLrV549MsYuGgcMXfk6qP0lcAIdPZ).
You need to set up your webserver to deliver images at `BASE_URL/img/`. Alternatively, you can remove the commented line in `main.py` and comment the non-commented one
and put the images into `/img`.

You can either use Python by installing the packages from `requirements.txt` and then running `python src/main.py` or alternatively using pipenv. If you want to use Docker, you can use the provided `docker-compose.yml`.

For the `docker-compose` file you will need to create a network called `proxy_net` and set the BASE_URL inside of the file to the URL you will be hosting it at.

| Endpoint         | Result                       |
| :--------------: | :--------------------------: |
| /                | Random image (HTML)          |
| /<waifu_id>      | Image by ID (HTML)           |
|  /get_url        | Random image (plaintext URL) |
