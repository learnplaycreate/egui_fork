//! Miscellaneous tools used by the rest of egui.

pub mod cache;
pub(crate) mod fixed_cache;
pub mod id_type_map;
pub mod undoer;
//mod pm_egui_plot_helpers;

pub use id_type_map::IdTypeMap;
//use pm_egui_plot_helpers::*;

pub use epaint::emath::History;
pub use epaint::util::{hash, hash_with};
