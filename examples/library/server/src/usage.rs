use sauron::prelude::*;

pub fn usage_section<T>() -> Node<T> {
    node! {
        <section id="usage">
            <h2>"Usage"</h2>
            <p>"Components and documentation"</p>
            <h4>"Installation"</h4>
            <p>"Install Rust"</p>
            <p>"For macOS, Linux or other Unix-like OS"</p>
            <pre>
                <code>
                    "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
                </code>
            </pre>
            <p>"Install through cargo"</p>
            <pre><code>"cargo install sauron-components"</code></pre>
        </section>
    }
}
