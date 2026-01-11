#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileType {
    Text,
    Image,
    Pdf,
    Binary,
}

impl FileType {
    pub fn from_extension(ext: &str) -> Self {
        let ext = ext.to_lowercase();
        match ext.as_str() {
            // Text files
            "txt" | "md" | "rs" | "py" | "js" | "ts" | "jsx" | "tsx" | "json" | "yaml" | "yml"
            | "toml" | "xml" | "html" | "css" | "sh" | "bash" | "c" | "cpp" | "h" | "hpp"
            | "java" | "go" | "rb" | "php" | "swift" | "kt" | "cs" | "sql" => FileType::Text,

            // Image files
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "ico" => FileType::Image,

            // PDF files
            "pdf" => FileType::Pdf,

            // Everything else is binary
            _ => FileType::Binary,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_type_from_extension_text() {
        assert_eq!(FileType::from_extension("txt"), FileType::Text);
        assert_eq!(FileType::from_extension("rs"), FileType::Text);
        assert_eq!(FileType::from_extension("py"), FileType::Text);
        assert_eq!(FileType::from_extension("js"), FileType::Text);
        assert_eq!(FileType::from_extension("md"), FileType::Text);
    }

    #[test]
    fn test_file_type_from_extension_image() {
        assert_eq!(FileType::from_extension("png"), FileType::Image);
        assert_eq!(FileType::from_extension("jpg"), FileType::Image);
        assert_eq!(FileType::from_extension("jpeg"), FileType::Image);
        assert_eq!(FileType::from_extension("gif"), FileType::Image);
        assert_eq!(FileType::from_extension("webp"), FileType::Image);
    }

    #[test]
    fn test_file_type_from_extension_pdf() {
        assert_eq!(FileType::from_extension("pdf"), FileType::Pdf);
    }

    #[test]
    fn test_file_type_from_extension_binary() {
        assert_eq!(FileType::from_extension("exe"), FileType::Binary);
        assert_eq!(FileType::from_extension("bin"), FileType::Binary);
        assert_eq!(FileType::from_extension("unknown"), FileType::Binary);
        assert_eq!(FileType::from_extension(""), FileType::Binary);
    }

    #[test]
    fn test_file_type_case_insensitive() {
        assert_eq!(FileType::from_extension("PNG"), FileType::Image);
        assert_eq!(FileType::from_extension("TXT"), FileType::Text);
        assert_eq!(FileType::from_extension("PDF"), FileType::Pdf);
    }
}
