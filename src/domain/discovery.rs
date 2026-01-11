use super::{FileEntry, FileType};
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortBy {
    Date,
    Name,
    Size,
    Type,
}

#[derive(Debug, Clone)]
pub struct DiscoveryOptions {
    pub file_types: Option<Vec<FileType>>,
    pub show_hidden: bool,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub sort_by: SortBy,
    pub reverse: bool,
}

impl Default for DiscoveryOptions {
    fn default() -> Self {
        DiscoveryOptions {
            file_types: None,
            show_hidden: false,
            min_size: None,
            max_size: None,
            sort_by: SortBy::Date,
            reverse: false,
        }
    }
}

pub fn discover_files(dir_path: &Path) -> io::Result<Vec<FileEntry>> {
    discover_files_with_options(dir_path, &DiscoveryOptions::default())
}

pub fn discover_files_with_options(
    dir_path: &Path,
    options: &DiscoveryOptions,
) -> io::Result<Vec<FileEntry>> {
    let entries = fs::read_dir(dir_path)?;
    let mut files = Vec::new();

    for entry_result in entries {
        let entry = entry_result?;
        let path = entry.path();

        if path.is_dir() {
            continue;
        }

        if !options.show_hidden {
            if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                if filename.starts_with('.') {
                    continue;
                }
            }
        }

        if let Ok(file_entry) = FileEntry::from_path(&path) {
            if let Some(ref type_filters) = options.file_types {
                if !type_filters.contains(&file_entry.file_type) {
                    continue;
                }
            }

            if let Some(min_size) = options.min_size {
                if file_entry.size < min_size {
                    continue;
                }
            }

            if let Some(max_size) = options.max_size {
                if file_entry.size > max_size {
                    continue;
                }
            }

            files.push(file_entry);
        }
    }

    files.sort_by(|a, b| match options.sort_by {
        SortBy::Date => a.modified_date.cmp(&b.modified_date),
        SortBy::Name => a.name.cmp(&b.name),
        SortBy::Size => a.size.cmp(&b.size),
        SortBy::Type => {
            let a_type = format!("{:?}", a.file_type);
            let b_type = format!("{:?}", b.file_type);
            a_type.cmp(&b_type)
        }
    });

    if options.reverse {
        files.reverse();
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use std::thread;
    use std::time::Duration;
    use tempfile::TempDir;

    #[test]
    fn test_discover_files_in_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");
        fs::write(&file1, "content1").unwrap();
        fs::write(&file2, "content2").unwrap();

        let files = discover_files(temp_dir.path()).unwrap();

        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_discover_files_empty_directory() {
        let temp_dir = TempDir::new().unwrap();
        let files = discover_files(temp_dir.path()).unwrap();
        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_discover_files_nonexistent_directory() {
        let result = discover_files(Path::new("/nonexistent/directory"));
        assert!(result.is_err());
    }

    #[test]
    fn test_discover_files_only_files_not_directories() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let subdir = temp_dir.path().join("subdir");
        fs::write(&file1, "content").unwrap();
        fs::create_dir(&subdir).unwrap();

        let files = discover_files(temp_dir.path()).unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "file1.txt");
    }

    #[test]
    fn test_discover_files_filters_hidden_files() {
        let temp_dir = TempDir::new().unwrap();
        let visible = temp_dir.path().join("visible.txt");
        let hidden = temp_dir.path().join(".hidden.txt");
        fs::write(&visible, "content").unwrap();
        fs::write(&hidden, "content").unwrap();

        let files = discover_files(temp_dir.path()).unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "visible.txt");
    }

    #[test]
    fn test_discover_files_filters_hidden_directories() {
        let temp_dir = TempDir::new().unwrap();
        let hidden_dir = temp_dir.path().join(".hidden_dir");
        fs::create_dir(&hidden_dir).unwrap();

        let files = discover_files(temp_dir.path()).unwrap();

        assert_eq!(files.len(), 0);
    }

    #[test]
    fn test_discover_files_sorts_by_modification_date() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = temp_dir.path().join("file1.txt");
        let file2 = temp_dir.path().join("file2.txt");

        fs::write(&file1, "content1").unwrap();
        thread::sleep(Duration::from_millis(10));
        fs::write(&file2, "content2").unwrap();

        let files = discover_files(temp_dir.path()).unwrap();

        assert_eq!(files.len(), 2);
        assert_eq!(files[0].name, "file1.txt");
        assert_eq!(files[1].name, "file2.txt");
    }

    #[test]
    fn test_discover_with_file_type_filter() {
        let temp_dir = TempDir::new().unwrap();
        let txt_file = temp_dir.path().join("file.txt");
        let png_file = temp_dir.path().join("image.png");
        fs::write(&txt_file, "content").unwrap();
        fs::write(&png_file, "content").unwrap();

        let options = DiscoveryOptions {
            file_types: Some(vec![FileType::Text]),
            ..Default::default()
        };

        let files = discover_files_with_options(temp_dir.path(), &options).unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "file.txt");
    }

    #[test]
    fn test_discover_with_size_filter() {
        let temp_dir = TempDir::new().unwrap();
        let small_file = temp_dir.path().join("small.txt");
        let large_file = temp_dir.path().join("large.txt");
        fs::write(&small_file, "small").unwrap();
        fs::write(&large_file, "large content here").unwrap();

        let options = DiscoveryOptions {
            min_size: Some(10),
            ..Default::default()
        };

        let files = discover_files_with_options(temp_dir.path(), &options).unwrap();

        assert_eq!(files.len(), 1);
        assert_eq!(files[0].name, "large.txt");
    }

    #[test]
    fn test_discover_with_show_hidden() {
        let temp_dir = TempDir::new().unwrap();
        let visible = temp_dir.path().join("visible.txt");
        let hidden = temp_dir.path().join(".hidden.txt");
        fs::write(&visible, "content").unwrap();
        fs::write(&hidden, "content").unwrap();

        let options = DiscoveryOptions {
            show_hidden: true,
            ..Default::default()
        };

        let files = discover_files_with_options(temp_dir.path(), &options).unwrap();

        assert_eq!(files.len(), 2);
    }

    #[test]
    fn test_discover_sort_by_name() {
        let temp_dir = TempDir::new().unwrap();
        let file_b = temp_dir.path().join("b.txt");
        let file_a = temp_dir.path().join("a.txt");
        fs::write(&file_b, "content").unwrap();
        fs::write(&file_a, "content").unwrap();

        let options = DiscoveryOptions {
            sort_by: SortBy::Name,
            ..Default::default()
        };

        let files = discover_files_with_options(temp_dir.path(), &options).unwrap();

        assert_eq!(files[0].name, "a.txt");
        assert_eq!(files[1].name, "b.txt");
    }

    #[test]
    fn test_discover_sort_by_size() {
        let temp_dir = TempDir::new().unwrap();
        let small = temp_dir.path().join("small.txt");
        let large = temp_dir.path().join("large.txt");
        fs::write(&small, "hi").unwrap();
        fs::write(&large, "hello world").unwrap();

        let options = DiscoveryOptions {
            sort_by: SortBy::Size,
            ..Default::default()
        };

        let files = discover_files_with_options(temp_dir.path(), &options).unwrap();

        assert_eq!(files[0].name, "small.txt");
        assert_eq!(files[1].name, "large.txt");
    }

    #[test]
    fn test_discover_reverse_sort() {
        let temp_dir = TempDir::new().unwrap();
        let file_a = temp_dir.path().join("a.txt");
        let file_b = temp_dir.path().join("b.txt");
        fs::write(&file_a, "content").unwrap();
        fs::write(&file_b, "content").unwrap();

        let options = DiscoveryOptions {
            sort_by: SortBy::Name,
            reverse: true,
            ..Default::default()
        };

        let files = discover_files_with_options(temp_dir.path(), &options).unwrap();

        assert_eq!(files[0].name, "b.txt");
        assert_eq!(files[1].name, "a.txt");
    }
}
