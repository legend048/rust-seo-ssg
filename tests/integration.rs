use assert_cmd::Command;
use tempfile::TempDir;
use std::{fs, path::Path};

#[test]
fn full_build_generates_expected_html_and_assets() {
    // 1. Create a fake project layout
    let proj = TempDir::new().unwrap();
    let content = proj.path().join("content");
    let templates = proj.path().join("templates");
    let static_dir = proj.path().join("static");
    let public = proj.path().join("public");
    fs::create_dir_all(&content).unwrap();
    fs::create_dir_all(&templates).unwrap();
    fs::create_dir_all(&static_dir).unwrap();

    fs::write(content.join("a.md"), "# A").unwrap();
    fs::write(templates.join("base.html"), "<body>{{ content }}</body>").unwrap();
    fs::write(static_dir.join("x.txt"), "hi").unwrap();

    // 2. Run your binary with overridden cwd
    Command::cargo_bin("rust_seo_ssg")
        .unwrap()
        .current_dir(proj.path())
        .assert()
        .success();

    // 3. Check outputs
    let out_html = fs::read_to_string(public.join("a.html")).unwrap();
    assert!(out_html.contains("<h1>A</h1>"));
    assert!(public.join("x.txt").exists());
}
