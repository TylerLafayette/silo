#![deny(missing_docs)]

//! Silo's core crate, containing an actor system and logging.

/// The actor module which contains an Actix actor.
pub mod actor;

/// The service of the database.
pub mod service;

/// Database config struct.
pub mod config;

/// Errors.
pub mod errors;

/// Utility functions for working with databases.
mod db_utils;

/// Internal models for representing database structures.
mod models;

/// Provides methods and structures for connecting to databases.
pub mod connection;

/// Test
mod migrations;
