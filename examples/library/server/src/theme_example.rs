use sauron::prelude::*;

pub fn theme_example_card<T>(theme: &str) -> Node<T> {
    let card_label = format!("Forced {} theme example", theme);
    node! {
        <article
            data-theme={theme.to_owned()}
            aria-label={card_label.to_owned()}
        >
          <h4>{text(format!("{} theme", theme))}</h4>
          <form class="grid">
            <input
                type="text"
                name="login"
                placeholder="Login"
                aria-label="Login"
                autocomplete="nickname"
                required
            />
            <input
                type="password"
                name="password"
                placeholder="Password"
                aria-label="Password"
                autocomplete="current-password"
                required
            />
            <button
                type="submit"
                aria-label="Example button"
                onclick="event.preventDefault()"
            >
                "Login"
            </button>
          </form>
          <footer class="code">
            <pre>
                <code>
                    "&lt;"<b>"article "</b>
                    <i>"data-theme"</i>"="
                    <u>{text(theme)}</u>"&gt;&lt;/"<b>"article"</b>"&gt;"
                </code>
            </pre>
          </footer>
        </article>
    }
}
