use crate::svg_icons::github_logo;
use sauron::prelude::*;

pub fn nav_bar<T>() -> Node<T> {
    node! {

        // TOP Bar
        // TODO: Move into it's own component class
        <nav class="container-fluid">
            <ul>
                <li>"[Logo]"</li>
                <li><h2>"Saroun Component Library"</h2></li>
            </ul>
            <ul>
                <li>"Examples"</li>
                <li>"Docs"</li>
                <li>
                    {github_logo(
                        "https://github.com/deathbreakfast/saruron-pico-library".to_owned()
                    )}
                </li>
            </ul>
        </nav>

    }
}
