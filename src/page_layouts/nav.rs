use sauron::prelude::*;

use crate::SauronComponent;

pub struct NavEntry {
    url: String,
    display_text: String,
}

impl NavEntry {
    pub fn new(url: &str, display_text: &str) -> NavEntry {
        NavEntry {
            url: url.to_owned(),
            display_text: display_text.to_owned(),
        }
    }
}

impl SauronComponent for NavEntry {
    fn component<T>(&self) -> Node<T> {
        node! {
            <li><a href={&self.url} class="secondary">{text(&self.display_text)}</a></li>
        }
    }
}

pub struct NavGroup {
    display_title: String,
    entries: Vec<NavEntry>,
}

impl NavGroup {
    pub fn new(display_title: &str) -> NavGroup {
        NavGroup {
            display_title: display_title.to_owned(),
            entries: vec![],
        }
    }

    pub fn add_entry(&mut self, entry: NavEntry) -> &mut Self {
        self.entries.push(entry);
        self
    }
}

impl SauronComponent for NavGroup {
    fn component<T>(&self) -> Node<T> {
        node! {
            <details open>
                <summary>{text(&self.display_title)}</summary>
                <ul>
                {for entry in &self.entries {
                    entry.component()
                }}
                </ul>
            </details>
        }
    }
}

pub struct Navigation {
    groups: Vec<NavGroup>,
}

impl Navigation {
    pub fn new() -> Navigation {
        Navigation { groups: vec![] }
    }

    pub fn add_group(&mut self, group: NavGroup) -> &mut Self {
        self.groups.push(group);
        self
    }
}

impl SauronComponent for Navigation {
    fn component<T>(&self) -> Node<T> {
        node! {
            <aside><nav>
            {for group in &self.groups {
                group.component()
            }}
            </nav></aside>
        }
    }
}
