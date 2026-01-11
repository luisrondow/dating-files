#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Decision {
    Keep,
    Trash,
}

#[derive(Debug, Clone)]
pub struct DecisionStatistics {
    pub total_files: usize,
    pub kept: usize,
    pub trashed: usize,
}
