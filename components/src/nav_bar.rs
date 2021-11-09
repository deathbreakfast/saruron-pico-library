use super::theme_example::theme_example_card;
use sauron::prelude::*;

/// TODO: Move into section folder
/// as a component
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
                <li>{github_logo}</li>
            </ul>
        </nav>

    }
}
