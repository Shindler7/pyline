//! Module for selecting code files for subsequent analysis.

pub mod config;
pub mod models;
pub mod result;
pub mod traits;
pub mod traversal;
pub mod types;

pub use config::Collector;
pub use models::FileData;
pub use result::CollectorResult;
pub use traits::FileDataExt;

#[cfg(test)]
mod tests;
