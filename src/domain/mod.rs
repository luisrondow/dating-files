// Module declarations
pub mod app_state;
pub mod decision;
pub mod decision_engine;
pub mod discovery;
pub mod file_entry;
pub mod file_type;

// Re-exports for convenience
pub use app_state::AppState;
pub use decision::{Decision, DecisionStatistics};
pub use decision_engine::DecisionEngine;
pub use discovery::{discover_files, discover_files_with_options, DiscoveryOptions, SortBy};
pub use file_entry::FileEntry;
pub use file_type::FileType;
