use std::fs;
use regex::Regex;

fn main() {
    let html_path = "../src/index.html";
    let mut html = fs::read_to_string(html_path).expect("Failed to read main page");

    html = Regex::new(r"(<h5[^>]*>Version\s+[\d.]+)(</h5>)").unwrap() // Patch edition
        .replace(&html, "$1 (Standalone)$2")
        .to_string();

    html = html.replace("</head>", "    <script src=\"scripts/standalone.js\"></script>\n</head>"); // Patchin in standalone code

    fs::write(html_path, html).expect("Failed to write main page");

    tauri_build::build()
}
