use super::theme_example::theme_example_card;
use sauron::prelude::*;

/// TODO: Move into section folder
pub fn theme_section<T>() -> Node<T> {
    node! {
        <section id="themes">
            <h2>"Themes"</h2>
            <p>"Support light, dark, and 10 other colors"</p>
            <h4>"Light"</h4>
            <p>"The light theme is used by default. The dark theme can be toggled"</p>
            <article aria-label="Theme switcher">
                // Convert to client side button.
                <button class="contrast theme-switcher">
                    "..."
                </button>
            </article>
            <p>
                "Themes can be forced on document level "
                <code>
                    "&lt;"<b>"html "</b>
                    <i>"data-theme"</i>"="
                    <u>"light"</u>"&gt;"
                </code>
                " or on any HTML element "
                <code>
                    "&lt;"<b>"article "</b>
                    <i>"data-theme"</i>"="
                    <u>"dark"</u>"&gt;"
                </code>
            </p>
            {theme_example_card("light")}
            {theme_example_card("dark")}
            <h4>"Color Switcher"</h4>
        </section>
    }
}
