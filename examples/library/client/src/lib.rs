use sauron::js_sys::TypeError;
use sauron::prelude::*;
use serde::{Deserialize, Serialize};

#[macro_use]
extern crate log;

const DATA_URL: &'static str = "/api";

#[derive(Debug, Deserialize, Serialize)]
pub enum FetchStatus<T> {
    Idle(T),
    Loading,
    Complete(T),
    Error(Option<String>),
}

#[derive(Debug)]
pub enum Msg {
    EditName(String),
    ReceivedData(Data),
    JsonError(serde_json::Error),
    RequestError(TypeError),
    QueryAPI,
}

#[derive(Debug, Deserialize, Serialize, PartialEq, Clone)]
pub struct Data {
    pub length: usize,
    pub modified_name: String,
}

// App and all its members should be Serializable by serde
#[derive(Debug, Deserialize, Serialize)]
pub struct App {
    pub name: String,
    pub data: FetchStatus<Data>,
    error: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            name: String::from(""),
            data: FetchStatus::Idle(Data {
                length: 0,
                modified_name: String::from(""),
            }),
            error: None,
        }
    }
}

impl App {
    pub fn with_name_and_data(name: &str, data: Data) -> Self {
        Self {
            name: name.to_string(),
            data: FetchStatus::Complete(data),
            ..Default::default()
        }
    }

    fn fetch_data(&self) -> Cmd<Self, Msg> {
        let url = format!("{}/{}", DATA_URL, self.name);
        Http::fetch_with_text_response_decoder(
            &url,
            |v: String| {
                let data: Result<Data, _> = serde_json::from_str(&v);
                trace!("data: {:#?}", data);
                match data {
                    Ok(data) => Msg::ReceivedData(data),
                    Err(e) => Msg::JsonError(e),
                }
            },
            Msg::RequestError,
        )
    }
}

impl Application<Msg> for App {
    fn init(&mut self) -> Cmd<Self, Msg> {
        Cmd::none()
    }

    fn view(&self) -> Node<Msg> {
        node! {
            <div>
                <article>
                    <form on_submit={|e| {
                        e.prevent_default();
                        Msg::QueryAPI
                    }} method="POST" action="/" >
                        <label>
                            "What’s your name, man?"
                            <input
                                name="name"
                                type="text"
                                value={&self.name}
                                on_input={|e| Msg::EditName(e.value)}
                            />
                        </label>
                        <button type="submit">"Okay!"</button>
                    </form>
                </article>
                {self.view_data()}
            </div>
        }
    }

    fn update(&mut self, msg: Msg) -> Cmd<Self, Msg> {
        trace!("App is updating from msg: {:?}", msg);
        let mut cmd = Cmd::none();
        match msg {
            Msg::EditName(name) => self.name = name,
            Msg::QueryAPI => {
                self.data = FetchStatus::Loading;
                cmd = self.fetch_data()
            }
            Msg::ReceivedData(data) => self.data = FetchStatus::Complete(data),
            Msg::JsonError(err) => {
                trace!("Error fetching data! {:#?}", err);
                self.data = FetchStatus::Error(Some(format!(
                    "There was an error reaching the api: {:?}",
                    err
                )));
            }
            Msg::RequestError(type_error) => {
                trace!("Error requesting the page: {:?}", type_error);
                self.data = FetchStatus::Error(Some(format!(
                    "There was an error fetching the page: {:?}",
                    type_error
                )));
            }
        };
        cmd
    }
}

impl App {
    fn view_data(&self) -> Node<Msg> {
        match &self.data {
            FetchStatus::Idle(_) => node! { <p>"Waiting around..."</p> },
            FetchStatus::Error(Some(e)) => {
                node! {
                    <article>
                        <p>"Okay, something went wrong. I think it was: "</p>
                        <code>{text(e)}</code>
                    </article>
                }
            }
            FetchStatus::Error(None) => {
                node! {
                    <article>
                        <p>"Okay, something went wrong. I have no idea what it is."</p>
                    </article>
                }
            }
            FetchStatus::Loading => {
                node! {
                    <article>
                        <p>"Loading..."</p>
                    </article>
                }
            }
            FetchStatus::Complete(data) => {
                node! {
                    <article>
                        <p>"Hello, "<span class="modified-name">{text(&data.modified_name)}</span></p>
                        <p>"Hey, did you know that’s "<span class="length">{text(&data.length)}</span>" characters long?"</p>
                    </article>
                }
            }
        }
    }
}

/// The serialized_state is supplied by the generated page from the webserver.
/// The generated page in index function has a main function which is supplied by a json text
/// serialized state. This json text is deserialized and used here as our `App` value which
/// will then be injected into the view
#[wasm_bindgen]
pub fn main(serialized_state: String) {
    console_log::init_with_level(log::Level::Trace).unwrap();
    console_error_panic_hook::set_once();

    /* Deserialize starting app data from the argument, which is passed in the generated index page
     * (but generated in server/src/main.rs) */
    let app = if let Ok(app_state) = serde_json::from_str::<App>(&serialized_state) {
        app_state
    } else {
        App::default()
    };

    /* If there's a window (i.e., if this is running in the browser)
     * then mount the app by swapping out the <main> tag */
    match web_sys::window() {
        Some(window) => {
            trace!("found window, will try to replace <main>");
            let document = window.document().expect("should have a document on window");
            Program::replace_mount(app, &document.get_element_by_id("view").unwrap());
        }
        None => {
            trace!("window not found");
            Program::mount_to_body(app);
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
