use super::FileType;
use chrono::{DateTime, Utc};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub modified_date: DateTime<Utc>,
    pub file_type: FileType,
}

impl FileEntry {
    pub fn from_path(path: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        let modified = metadata.modified()?;
        let modified_date: DateTime<Utc> = modified.into();

        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let extension = path.extension().and_then(|e| e.to_str()).unwrap_or("");

        let file_type = FileType::from_extension(extension);

        Ok(FileEntry {
            path: path.to_path_buf(),
            name,
            size: metadata.len(),
            modified_date,
            file_type,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_entry_from_path() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        fs::write(path, b"test content").unwrap();

        let entry = FileEntry::from_path(path).unwrap();

        assert_eq!(entry.path, path);
        assert!(entry.name.len() > 0);
        assert_eq!(entry.size, 12);
        assert_eq!(entry.file_type, FileType::Binary);
    }

    #[test]
    fn test_file_entry_from_path_with_extension() {
        let temp_file = NamedTempFile::new().unwrap();
        let path = temp_file.path();
        let txt_path = path.with_extension("txt");
        fs::write(&txt_path, b"hello").unwrap();

        let entry = FileEntry::from_path(&txt_path).unwrap();

        assert_eq!(entry.file_type, FileType::Text);
        assert_eq!(entry.size, 5);

        fs::remove_file(&txt_path).ok();
    }

    #[test]
    fn test_file_entry_nonexistent_file() {
        let result = FileEntry::from_path(Path::new("/nonexistent/file.txt"));
        assert!(result.is_err());
    }
}
