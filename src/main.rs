use pulldown_cmark::{html, Options, Parser};
use reqwest::blocking::Client;
use std::fs;
use std::io::Write;
use std::env;
use std::path::Path;
use webbrowser;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <path to input.md>", args[0]);
        std::process::exit(1);
    }

    let input_path = Path::new(&args[1]);
    if input_path.extension().unwrap_or_default() != "md" {
        eprintln!("The file should have a .md extension");
        std::process::exit(1);
    }

    let markdown_input = fs::read_to_string(input_path)
        .expect("Failed to read input file");

    let options = Options::ENABLE_STRIKETHROUGH
        | Options::ENABLE_TABLES
        | Options::ENABLE_FOOTNOTES
        | Options::ENABLE_TASKLISTS
        | Options::ENABLE_SMART_PUNCTUATION;
    let parser = Parser::new_ext(&markdown_input, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    let client = Client::new();
    let css_file = client
        .get("https://cdnjs.cloudflare.com/ajax/libs/github-markdown-css/4.0.0/github-markdown.min.css")
        .send()
        .expect("Failed fetching CSS content")
        .text()
        .expect("Failed to read CSS content");

    let output_file_name = input_path.with_extension("html");
    let mut html_file = fs::File::create(&output_file_name)
        .expect("Failed to create HTML output file");

    writeln!(
        html_file,
        r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Markdown to HTML</title>
    <style>
        {}
    </style>
    <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/default.min.css">
    <script src="https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/mermaid@8.13.5/dist/mermaid.min.js"></script>
</head>
<body class="markdown-body">
{}
<script>
    hljs.highlightAll();
    document.addEventListener('DOMContentLoaded', function() {{
        const mermaidElements = document.querySelectorAll('code.language-mermaid');
        mermaidElements.forEach(element => {{
            const mermaidCode = element.textContent.trim();
            mermaid.mermaidAPI.render('mermaid-svg-' + Math.random().toString(36).substr(2, 9), mermaidCode, svgCode => {{
                element.parentElement.innerHTML = svgCode;
            }});
        }});
    }});
</script>
</body>
</html>
"#,
        css_file, html_output
    ).expect("Failed writing HTML content to file");

    println!("Generated HTML file: {:?}", output_file_name);

    if webbrowser::open(output_file_name.to_str().unwrap()).is_ok() {
        println!("Opened the HTML file in a web browser.");
    } else {
        eprintln!("Failed to open the HTML file in a web browser.");
    }
}
