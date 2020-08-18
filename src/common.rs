mod file_entry;
mod line_entry;
mod warning;

pub use file_entry::FileEntry;
pub use line_entry::LineEntry;
pub use warning::Warning;

pub const LF: &str = "\n";
