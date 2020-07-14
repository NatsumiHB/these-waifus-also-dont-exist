# these-waifus-also-dont-exist
I used the dataset of ThisWaifuDoesNotExist and generated ten thousand images on my own. The website automatically returns a random image. 

## Usage
You will need to download and provide the images yourself. You can get them [here](https://docs.google.com/uc?export=download&id=1PnHPLrV549MsYuGgcMXfk6qP0lcAIdPZ).
You need to create a folder and put the images into `/img`.

The base URL has to be set in the `.env` file in the root directory. If you are running this locally set it to `hostname:5002` and if you don't have a domain associated
set it to `ip:5002`

You can either use Python by installing the packages from `requirements.txt` and then running `python ./main.py` inside of `/src` or 
alternatively doing the same using pipenv. If you want to use Docker, you can use the provided `docker-compose.yml`.

For the `docker-compose` file you will need to create a network called `proxy_net`.

| Endpoint    | Result                          |
| :---------: | :-----------------------------: |
| /           | Random image (HTML)             |
| /<waifu_id> | Image by ID (HTML)              |
|  /get_json  | JSON containing ID and file_url |
