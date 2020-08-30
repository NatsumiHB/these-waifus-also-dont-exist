use crate::config;

pub fn get_html(id: usize, base_url: &str, random: bool) -> String {
    let (image_src, header_id, og_url) = if !random {
        (
            format!(
                r#"<link rel="image_src" href="{0}/img/seed{1:04}.png">"#,
                base_url, id
            ),
            id,
            format!("/img/seed{0:04}.png", id),
        )
    } else {
        (String::new(), config::DEFAULT_WAIFU_ID, String::new())
    };

    let headers = format!(
        r#"
            <meta property="og:title" content="These Waifus Also Don't Exist">
            <meta property="og:description" content="AI-generated waifus">
            <meta name="description" content="AI-generated waifus">

            {image_src}
            <meta property="og:image" content="{base_url}/img/seed{id:04}.png">
            <meta name="twitter:image" content="{base_url}/img/seed{id:04}.png">
            <meta property="og:image:width" content="148">
            <meta property="og:image:height" content="148">

            <meta name="theme-color" content="\#57ECEF">

            <meta property="og:url" content="{base_url}/{og_url}">
        "#,
        base_url = base_url,
        image_src = image_src,
        id = header_id,
        og_url = og_url
    );

    format!(
        r#"
        <!DOCTYPE html>
        <html>
            <head>
                {headers}
                <meta name="viewport" content="width=device-width, initial-scale=1.0">
                <title>Waifu #{id}</title>
            </head>
            <body>
                <a href="/img/seed{id:04}.png"><h1>Waifu #{id}</h1></a>
                <img src="/img/seed{id:04}.png" alt="Waifu #{id}" width="768" height="768">
            </body>
        </html>
        "#,
        headers = headers,
        id = id
    )
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
