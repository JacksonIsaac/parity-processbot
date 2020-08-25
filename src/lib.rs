pub mod bamboo;
pub mod companion;
pub mod config;
pub mod constants;
pub mod error;
pub mod github;
pub mod github_bot;
pub mod http;
pub mod matrix;
pub mod matrix_bot;
pub mod performance;
pub mod process;
pub mod rebase;
pub mod server;
pub mod webhook;

pub type Result<T> = std::result::Result<T, error::Error>;
