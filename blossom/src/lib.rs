#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used)]
#![forbid(clippy::indexing_slicing)]

pub mod account;
pub mod auth;
pub mod broker;
pub mod command;
pub mod commands;
pub mod config;
pub mod connection;
pub mod connection_handler;
pub mod constants;
pub mod context;
pub mod database;
pub mod direction;
pub mod entity;
pub mod error;
pub mod event;
pub mod game;
pub mod input;
pub mod logging;
pub mod monster;
pub mod player;
pub mod prelude;
pub mod prompt;
pub mod quickmap;
pub mod region;
pub mod response;
pub mod role;
pub mod room;
pub mod scripting;
pub mod searchable;
pub mod server;
pub mod stores;
pub mod system;
pub mod systems;
pub mod theme;
pub mod timer;
pub mod utils;
pub mod vec3;
pub mod web;
pub mod world;
