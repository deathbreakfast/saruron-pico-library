use sauron::prelude::*;
mod page_layouts;
mod svg_icons;

pub use page_layouts::{NavEntry, NavGroup, Navigation};
pub use svg_icons::github_logo;

pub trait SauronComponent {
    fn component<T>(&self) -> Node<T>;
}
