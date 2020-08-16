# these-waifus-also-dont-exist
I used the dataset of ThisWaifuDoesNotExist and generated ten thousand images on my own. The website automatically returns a random image. 

## Usage
Set the `RUST_LOG` environment variable to specify your
[logging level](https://docs.rs/env_logger/0.7.1/env_logger/#enabling-logging)
and `BASE_URL` to the base URL that you are running this server on. If none is specified
this will default to [my hosted instance](https://waifus-are.fun-stuff.xyz).

After that, just run `cargo run` or run it with docker.

For the `docker-compose` file you will need to create a network called `proxy_net`.

| Endpoint    | Result                          |
| :---------: | :-----------------------------: |
| /           | Random image (HTML)             |
| /<waifu_id> | Image by ID (HTML)              |
|  /get_json  | JSON containing ID and file_url |
