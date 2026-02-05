pub mod scaffold;

pub use scaffold::{create_project, open_in_editor, ProjectConfig};

#[cfg(test)]
mod tests;