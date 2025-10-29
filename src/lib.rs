#![warn(clippy::all, rust_2018_idioms)]

mod app;
// allow usage of entities within the widgets module in the application
mod widgets;
mod apps;

pub use app::TemplateApp;
