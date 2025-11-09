#![warn(clippy::all, rust_2018_idioms)]

mod app;
mod components;
mod config;
mod db;
mod light;
mod only_one_toggleable_item;
mod path;

pub use app::App;
