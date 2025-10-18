use tera::{Tera, Context};

pub fn init_template_engine() -> anyhow::Result<Tera> {
    // globs all .html files under templates/
    let tera = Tera::new("templates/**/*.html")?;
    Ok(tera)
}

pub fn render_page(
    tera: &Tera,
    html_content: &str,
    page_title: &str,
    output_path: &str,
) -> anyhow::Result<()> {
    let mut ctx = Context::new();
    ctx.insert("content", &html_content);
    ctx.insert("title", &page_title);
    let rendered = tera.render("base.html", &ctx)?;
    std::fs::write(output_path, rendered)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{init_template_engine, render_page};
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn injects_content_into_template() {
        // Prepare a minimal template
        let tpl_dir = tempfile::tempdir().unwrap();
        fs::write(tpl_dir.path().join("base.html"),
            "<html><body>{{ content }}</body></html>").unwrap();

        // Point Tera at that dir
        let tera = tera::Tera::new(&format!("{}/**/*.html", tpl_dir.path().display())).unwrap();

        // Render
        let out_dir = tempfile::tempdir().unwrap();
        let out_file = out_dir.path().join("out.html");
        render_page(&tera, "<p>Hi</p>", "title", &out_file.to_string_lossy()).unwrap();

        let rendered = fs::read_to_string(out_file).unwrap();
        assert!(rendered.contains("<p>Hi</p>"));
    }
}
