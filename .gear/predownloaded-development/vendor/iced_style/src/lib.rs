//! The styling library of Iced.
//!
//! It contains a set of styles and stylesheets for most of the built-in
//! widgets.
//!
//! ![The foundations of the Iced ecosystem](https://github.com/iced-rs/iced/blob/0525d76ff94e828b7b21634fa94a747022001c83/docs/graphs/foundations.png?raw=true)
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iced-rs/iced/9ab6923e943f784985e9ef9ca28b10278297225d/docs/logo.svg"
)]
#![forbid(unsafe_code, rust_2018_idioms)]
#![deny(
    unused_results,
    missing_docs,
    unused_results,
    rustdoc::broken_intra_doc_links
)]
pub use iced_core as core;

pub mod application;
pub mod button;
pub mod checkbox;
pub mod container;
pub mod menu;
pub mod pane_grid;
pub mod pick_list;
pub mod progress_bar;
pub mod qr_code;
pub mod radio;
pub mod rule;
pub mod scrollable;
pub mod slider;
pub mod svg;
pub mod text_editor;
pub mod text_input;
pub mod theme;
pub mod toggler;

pub use theme::Theme;
