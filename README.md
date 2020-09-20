# these-waifus-also-dont-exist
I used the dataset of ThisWaifuDoesNotExist and generated ten thousand images on my own. The website automatically returns a random image. 

## Usage
Set the environment variables according to this table (in `.env` ):

| Variable     | Value                                                                                              |
| :----------: | :------------------------------------------------------------------------------------------------: |
| RUST_LOG     | [logging level](https://docs.rs/env_logger/0.7.1/env_logger/#enabling-logging)                     |
| BASE_URL     | The base URL the service is hosted at, defaults to [my instance](https://waifus-are.fun-stuff.xyz) |
| IMAGE_FORMAT | The format of the image, defaults to `webp`                                                        |
| DEFULT_ID    | The ID of the preview image, defaults to `2`                                                       |

After that, just run `cargo run` or run it with docker.

For the `docker-compose` file you will need to create a network called `proxy_net` or remove that from `docker-compose.yml`.

The server runs on port 5002, you will need to public that port in the `docker-compose.yml` file if you are
not running this service through a reverse proxy on the same bridge network.

| Endpoint             | Result                          |
| :------------------: | :-----------------------------: |
| /                    | Random image (HTML)             |
| /<waifu_id>          | Image by ID (HTML)              |
| /get_json            | JSON containing ID and file_url |
| /get_json/<waifu_is> | JSON by ID                      |

## Important note
This does not come with any images, the default format is `webp` and the images will
need to be placed in a folder called `img`. The images need to be indixed from 1 to `image amount - 1`.