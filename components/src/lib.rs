use sauron::prelude::*;
mod github;
mod nav;

pub use github::github_logo;
pub use nav::{NavEntry, NavGroup, Navigation};

pub trait SauronComponent {
    fn component<T>(&self) -> Node<T>;
}
