use super::theme_section::theme_section;
use super::usage::usage_section;
use client::{App, Msg};
use sauron::prelude::*;
use sauron_pico_library::{github_logo, NavEntry, NavGroup, Navigation, SauronComponent};

fn navigation() -> Navigation {
    let mut getting_started_group = NavGroup::new("Getting Started");
    getting_started_group
        .add_entry(NavEntry::new("#usage", "Usage"))
        .add_entry(NavEntry::new("#themes", "Themes"))
        .add_entry(NavEntry::new("#customization", "Customization"));

    let mut layout_group = NavGroup::new("Layout");
    layout_group
        .add_entry(NavEntry::new("#containers", "Containers"))
        .add_entry(NavEntry::new("#grids", "Grids"))
        .add_entry(NavEntry::new("scroller", "Horizontal Scroller"));
    let mut navigation = Navigation::new();
    navigation
        .add_group(getting_started_group)
        .add_group(layout_group);

    navigation
}

/// We are creating an index page.
/// From the `App` supplied, we can derive the view by calling `App.view` function.
/// we extract the state and serialize it.
///
/// TODO: Refactor this into an example page.
pub fn index(app: &App) -> Node<Msg> {
    let serialized_state = serde_json::to_string(&app).unwrap();
    let view = app.view();
    let github_logo = github_logo("https://github.com/deathbreakfast/Saroun-Rocket".to_owned());

    // TODO: Move Script into it's own component.
    // TODO: Use minified css?
    node! {
        <html lang="en">
          <head>
            <meta http-equiv="Content-type" content="text/html; charset=utf-8"/>
            <link rel="stylesheet" href="css/pico.css"/> <link rel="stylesheet" href="css/docs.css"/>
            <title>"Pico CSS Examples"</title>
            <script type="module">
                {text!("
                      import init, {{ main }} from '/pkg/client.js';
                      async function start() {{
                        await init();
                        main(`{}`);
                      }}
                      start();
                ",serialized_state)}
            </script>
          </head>
          <body>
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

            // Main
            <main class="container" id="docs">

                // Navigation
                {navigation().component()}

                <div role="document">
                    {usage_section()}
                    {theme_section()}

                <article class="grid" id="view">{view}</article>
                </div>
            </main>
      </body>
        </html>
    }
}
