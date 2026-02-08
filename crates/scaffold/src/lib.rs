// SPDX-License-Identifier: MIT
pub mod scaffold;

pub use scaffold::{ProjectConfig, create_project, open_in_editor};

#[cfg(test)]
mod tests;
