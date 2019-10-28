/// User-level commands that map to those in the CLI
pub mod commands;

/// Abstraction to compress data
pub mod compression;

/// Determines what files to ignore when committing changes
mod ignore;

/// Persistence to the Git database
mod objects;

mod config;

mod refs;
