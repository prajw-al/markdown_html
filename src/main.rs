use std::fs;
use std::io::Write;
use pulldown_cmark::{html, Options, Parser};
use reqwest::blocking::Client;

fn main() {
    let markdown_input = fs::read_to_string("input.md").expect("failed to create input file");

    let options = Options::empty();
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let client = Client::new();
    let css_file = client
        .get("https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/4.0.0/github-markdown.min.css")
        .send()
        .expect("failed fetching css content")
        .text()
        .expect("failed to read css content");

    let mut html_file = fs::File::create("output.html").expect("failed to create html output file");
    writeln!(
        html_file,
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Markdown to HTML</title>
            <style>
                {}
            </style>
        </head>
        <body class="markdown-body">{}</body>
        </html>
        "#,
        css_file, html_output
    )
    .expect("failed writing html content to file");
}







