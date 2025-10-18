use walkdir::WalkDir;
use std::{fs, path::Path};

pub fn collect_markdown_files(dir: &str) -> anyhow::Result<Vec<(String, String)>> {
    let mut files = Vec::new();
    for entry in WalkDir::new(dir).into_iter().filter_map(Result::ok) {
        if entry.path().extension().and_then(|e| e.to_str()) == Some("md") {
            let path_str = entry.path().display().to_string();
            let contents = fs::read_to_string(&path_str)?;
            files.push((path_str, contents));
        }
    }
    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::collect_markdown_files;
    use tempfile::tempdir;
    use std::fs::write;

    #[test]
    fn finds_md_and_reads_content() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("foo.md");
        write(&file, "# Foo").unwrap();

        let files = collect_markdown_files(dir.path().to_str().unwrap()).unwrap();
        assert_eq!(files.len(), 1);
        let (path, contents) = &files[0];
        assert!(path.ends_with("foo.md"));
        assert_eq!(contents.trim(), "# Foo");
    }
}
