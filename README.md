# Markdown to HTML Converter

This is a simple command-line tool written in Rust to convert Markdown files to HTML.

## Features

- Convert Markdown files to HTML
- Syntax highlighting using `highlight.js`
- Rendering of Mermaid diagrams

## Usage

1. Clone the repository:

    ```bash
    git clone https://github.com/prajw-al/markdown_html.git
    ```

2. Navigate to the project directory:

    ```bash
    cd markdown_html
    ```

3. Build the project:

    ```bash
    cargo build --release
    ```

4. Run the converter:

    ```bash
cargo run --release --<your/path/to/.md file>
    ```

    Replace `<your/path/to/.md file>` with the path to your Markdown file.

5. The HTML output file will be generated in the same directory as your input file with the same name but with a `.html` extension.

6. The generated HTML file will be opened in your default web browser automatically.

## Dependencies

- [pulldown_cmark](https://crates.io/crates/pulldown-cmark): For Markdown parsing.
- [reqwest](https://crates.io/crates/reqwest): For fetching CSS content.
- [webbrowser](https://crates.io/crates/webbrowser): For opening the HTML file in a web browser.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
