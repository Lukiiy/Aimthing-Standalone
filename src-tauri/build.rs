use std::fs;

fn main() {
    let html_path = "../src/index.html";
    let mut html = fs::read_to_string(html_path).expect("Failed to read main page");

    html = regex::Regex::new(r"(<h5[^>]*>Version\s+[\d.]+)(</h5>)") // Patch edition
        .unwrap()
        .replace(&html, "$1 (Standalone)$2")
        .to_string();

    html = html.replace("</head>", "  <script src=\"standalone.js\"></script>\n</head>"); // Patchin in standalone code

    fs::write(html_path, html).expect("Failed to write main page");

    tauri_build::build()
}
