mod io;
mod parser;
mod render;

fn main() -> anyhow::Result<()> {
    // 1. Ensure output dir exists
    std::fs::create_dir_all("public")?;

    // 2. Init template engine
    let tera = render::init_template_engine()?;

    // 3. Collect and process markdown files
    let files = io::collect_markdown_files("content")?;
    for (src_path, md) in files {
        let html = parser::markdown_to_html(&md);
        assets::copy_static_assets("static", "public")?;
        // derive a filename, e.g. "content/foo.md" → "public/foo.html"
        let filename = src_path
            .strip_prefix("content/")?
            .strip_suffix(".md")
            .unwrap();
        let out_path = format!("public/{}.html", filename);
        render::render_page(&tera, &html, filename, &out_path)?;
    }

    Ok(())
}
