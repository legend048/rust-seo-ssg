use walkdir::WalkDir;
use std::{fs, path::Path};

pub fn copy_static_assets(src: &str, dest: &str) -> anyhow::Result<()> {
    for entry in WalkDir::new(src).into_iter().filter_map(Result::ok) {
        let rel_path = entry.path().strip_prefix(src)?;
        let target = Path::new(dest).join(rel_path);
        if entry.file_type().is_dir() {
            fs::create_dir_all(&target)?;
        } else {
            fs::copy(entry.path(), &target)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::copy_static_assets;
    use tempfile::tempdir;
    use std::{fs, path::Path};

    #[test]
    fn copies_directory_structure_and_files() {
        let src = tempdir().unwrap();
        let nested = src.path().join("css");
        fs::create_dir_all(&nested).unwrap();
        fs::write(nested.join("app.css"), "body {}").unwrap();

        let dest = tempdir().unwrap();
        copy_static_assets(src.path().to_str().unwrap(), dest.path().to_str().unwrap()).unwrap();

        let copied = dest.path().join("css").join("app.css");
        assert!(copied.exists());
        assert_eq!(fs::read_to_string(copied).unwrap(), "body {}");
    }
}
