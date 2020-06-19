import os

base_url = os.getenv("BASE_URL")

not_found = f"""
<!DOCTYPE html>
<html>
    <body>
        <h1>Error 404!</h1>
        <h2>This waifu REALLY doesn't exist.</h2>
    </body>
</html>
"""


def return_html(waifu_id, embed=False):
    return f"""
    <!DOCTYPE html>
    <html>
        <head>
            <!-- Necessary to embed properly -->
            <!-- if this is empty you have visited a random size! -->
            {f'''
                <link rel="image_src" href="{base_url}/img/seed{waifu_id:0>4}.png">
                <meta property="og:image" content="{base_url}/img/seed{waifu_id:0>4}.png">
                <meta name="twitter:image" content="{base_url}/img/seed{waifu_id:0>4}.png">

                <meta property="og:image:width" content="1024">
                <meta property="og:image:height" content="1024">
                <meta name="twitter:card" content="summary_large_image">
            ''' if embed else ''}

            <title>Waifu #{waifu_id}</title>
        </head>
        <body>
            <a href="/img/seed{waifu_id:0>4}.png"><h1>Waifu #{waifu_id}</h1></a>
            <img src="/img/seed{waifu_id:0>4}.png" alt="Waifu #{waifu_id}" width="768" height="768">
        </body>
    </html>
    """
