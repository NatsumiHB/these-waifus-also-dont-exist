pub(crate) fn get_html(id: usize, embedded: bool) -> String {
    format!(r#"
    <!DOCTYPE html>
    <html>
        <head>
            <!-- Necessary to embed properly -->
            <!-- if this is empty you have visited a random size! -->
            {0}
            <title>Waifu #{1}</title>
        </head>
        <body>
            <a href="/img/seed{1:04}.png"><h1>Waifu #{1}</h1></a>
            <img src="/img/seed{1:04}.png" alt="Waifu #{1}" width="768" height="768">
        </body>
    </html>
    "#, if embedded {
        r#"
                <link rel="image_src" href="{base_url}/img/seed{waifu_id:0>{util.waifu_len}}.png">
                <meta property="og:image" content="{base_url}/img/seed{waifu_id:0>{util.waifu_len}}.png">
                <meta name="twitter:image" content="{base_url}/img/seed{waifu_id:0>{util.waifu_len}}.png">
                <meta property="og:image:width" content="1024">
                <meta property="og:image:height" content="1024">
                <meta name="twitter:card" content="summary_large_image">
        "#
    } else {
        ""
    }, id)
}

pub const NOT_FOUND: &str = r#"
<!DOCTYPE html>
<html>
    <body>
        <h1>Error 404!</h1>
        <h2>This waifu REALLY doesn't exist.</h2>
    </body>
</html>
"#;