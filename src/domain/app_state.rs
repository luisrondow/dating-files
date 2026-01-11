use super::{Decision, FileEntry};

#[derive(Debug)]
pub struct AppState {
    pub files: Vec<FileEntry>,
    pub current_index: usize,
    pub decisions_stack: Vec<(usize, Decision)>,
}

impl AppState {
    pub fn new(files: Vec<FileEntry>) -> Self {
        Self {
            files,
            current_index: 0,
            decisions_stack: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        if self.current_index < self.files.len().saturating_sub(1) {
            self.current_index += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub fn current_file(&self) -> Option<&FileEntry> {
        self.files.get(self.current_index)
    }

    pub fn record_decision(&mut self, decision: Decision) {
        self.decisions_stack.push((self.current_index, decision));
    }

    pub fn undo(&mut self) -> Option<(usize, Decision)> {
        self.decisions_stack.pop()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::FileType;
    use chrono::Utc;
    use std::path::PathBuf;

    fn create_test_entry(name: &str) -> FileEntry {
        FileEntry {
            path: PathBuf::from(name),
            name: name.to_string(),
            size: 0,
            modified_date: Utc::now(),
            file_type: FileType::Text,
        }
    }

    #[test]
    fn test_app_state_new() {
        let files = vec![
            create_test_entry("file1.txt"),
            create_test_entry("file2.txt"),
        ];

        let state = AppState::new(files.clone());

        assert_eq!(state.files.len(), 2);
        assert_eq!(state.current_index, 0);
        assert_eq!(state.decisions_stack.len(), 0);
    }

    #[test]
    fn test_app_state_current_file() {
        let files = vec![create_test_entry("file1.txt")];
        let state = AppState::new(files);

        let current = state.current_file();
        assert!(current.is_some());
        assert_eq!(current.unwrap().name, "file1.txt");
    }

    #[test]
    fn test_app_state_current_file_empty() {
        let state = AppState::new(vec![]);
        assert!(state.current_file().is_none());
    }

    #[test]
    fn test_app_state_next() {
        let files = vec![
            create_test_entry("file1.txt"),
            create_test_entry("file2.txt"),
            create_test_entry("file3.txt"),
        ];
        let mut state = AppState::new(files);

        state.next();
        assert_eq!(state.current_index, 1);

        state.next();
        assert_eq!(state.current_index, 2);

        state.next();
        assert_eq!(state.current_index, 2);
    }

    #[test]
    fn test_app_state_previous() {
        let files = vec![
            create_test_entry("file1.txt"),
            create_test_entry("file2.txt"),
        ];
        let mut state = AppState::new(files);

        state.current_index = 1;
        state.previous();
        assert_eq!(state.current_index, 0);

        state.previous();
        assert_eq!(state.current_index, 0);
    }

    #[test]
    fn test_app_state_record_decision() {
        let files = vec![
            create_test_entry("file1.txt"),
            create_test_entry("file2.txt"),
        ];
        let mut state = AppState::new(files);

        state.record_decision(Decision::Keep);

        assert_eq!(state.decisions_stack.len(), 1);
        assert_eq!(state.decisions_stack[0], (0, Decision::Keep));
    }

    #[test]
    fn test_app_state_undo() {
        let files = vec![
            create_test_entry("file1.txt"),
            create_test_entry("file2.txt"),
        ];
        let mut state = AppState::new(files);

        state.record_decision(Decision::Keep);
        state.current_index = 1;
        state.record_decision(Decision::Trash);

        let undone = state.undo();
        assert!(undone.is_some());
        assert_eq!(undone.unwrap(), (1, Decision::Trash));
        assert_eq!(state.decisions_stack.len(), 1);
    }

    #[test]
    fn test_app_state_undo_empty() {
        let files = vec![create_test_entry("file1.txt")];
        let mut state = AppState::new(files);

        let undone = state.undo();
        assert!(undone.is_none());
    }
}
