use crate::config::Config;

pub fn get_html(id: usize, config: &Config, random: bool) -> String {
    let (image_src, header_id, og_url) = if !random {
        (
            format!(
                r#"<link rel="image_src" href="{0}/img/seed{1:0width$}.{2}">"#,
                config.base_url,
                id,
                config.image_format,
                width = config.padding_width,
            ),
            id,
            format!("/img/seed{0:04}.png", id),
        )
    } else {
        (String::new(), config.default_id, String::new())
    };

    let meta_tags = format!(
        r#"
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            
            <meta property="og:title" content="These Waifus Also Don't Exist">
            <meta property="og:description" content="AI-generated waifus">
            <meta name="description" content="AI-generated waifus">

            {image_src}
            <meta property="og:image" content="{base_url}/img/seed{id:0width$}.{format}">
            <meta name="twitter:image" content="{base_url}/img/seed{id:0width$}.{format}">
            <meta property="og:image:width" content="148">
            <meta property="og:image:height" content="148">

            <meta name="theme-color" content="\#57ECEF">

            <meta property="og:url" content="{base_url}/{og_url}">
        "#,
        base_url = config.base_url,
        image_src = image_src,
        id = header_id,
        og_url = og_url,
        width = config.padding_width,
        format = config.image_format
    );

    format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
            <head>
                {meta_tags}
                <title>Waifu #{id}</title>
            </head>
            <body>
                <a href="/img/seed{id:0width$}.{format}"><h1>Waifu #{id}</h1></a>
                <img src="/img/seed{id:0width$}.{format}" alt="Waifu #{id}" width="768" height="768">
            </body>
        </html>
        "#,
        meta_tags = meta_tags,
        id = id,
        width = config.padding_width,
        format = config.image_format
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
