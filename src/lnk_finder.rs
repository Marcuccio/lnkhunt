use std::fs;
use std::path::{Path, PathBuf};

/// Function to find all .lnk files in a directory recursively
pub fn find_lnk_files(dir: &Path) -> Result<Vec<PathBuf>, std::io::Error> {
    let mut lnk_files = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_dir() {
            // Recursively search subdirectories
            lnk_files.extend(find_lnk_files(&path)?);
        } else if path.extension().and_then(|ext| ext.to_str()) == Some("lnk") {
            lnk_files.push(path);
        }
    }

    Ok(lnk_files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs::File;

    #[test]
    fn test_find_lnk_files() {
        let temp_dir = tempdir().unwrap();
        let temp_path = temp_dir.path();

        let lnk_file_1 = temp_path.join("file1.lnk");
        let lnk_file_2 = temp_path.join("file2.lnk");
        File::create(&lnk_file_1).unwrap();
        File::create(&lnk_file_2).unwrap();

        let sub_dir = temp_path.join("subdir");
        std::fs::create_dir(&sub_dir).unwrap();
        let sub_lnk_file = sub_dir.join("file3.lnk");
        File::create(&sub_lnk_file).unwrap();

        let lnk_files = find_lnk_files(temp_path).unwrap();

        assert_eq!(lnk_files.len(), 3);
        assert!(lnk_files.contains(&lnk_file_1));
        assert!(lnk_files.contains(&lnk_file_2));
        assert!(lnk_files.contains(&sub_lnk_file));
    }
}
