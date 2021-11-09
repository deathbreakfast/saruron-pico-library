#[macro_use]
extern crate rocket;
use client::{App, Data};
use rocket::form::Form;
use rocket::response::content;
use rocket::serde::json::Json;
use sauron::prelude::*;

mod page;
mod theme_example;
mod theme_section;
mod usage;

const DEFAULT_NAME: &str = "Ferris";

fn fake_api_call(name: String) -> Data {
    Data {
        length: name.len(),
        modified_name: name.to_uppercase(),
    }
}

fn render_page(name: String) -> content::Html<String> {
    let api_data = fake_api_call(name.clone());
    let app = App::with_name_and_data(&name, api_data);
    content::Html(page::index(&app).render_to_string())
}

#[get("/<name>")]
fn api_call(name: String) -> Json<Data> {
    Json(fake_api_call(name))
}

#[get("/", rank = 1)]
fn root() -> content::Html<String> {
    render_page(DEFAULT_NAME.to_owned())
}

#[get("/<name>", rank = 2)]
async fn named(name: String) -> content::Html<String> {
    render_page(name)
}

#[derive(FromForm)]
struct NameForm {
    name: String,
}

#[post("/", data = "<form_data>")]
async fn submit(form_data: Form<NameForm>) -> content::Html<String> {
    render_page(form_data.name.clone())
}

mod static_files {
    use rocket::fs::{relative, NamedFile};
    use std::path::{Path, PathBuf};

    #[get("/favicon.ico")]
    pub async fn favicon() -> Option<NamedFile> {
        let favicon_path = Path::new(relative!("../client/favicon.ico"));
        NamedFile::open(favicon_path).await.ok()
    }

    #[get("/pico.css")]
    pub async fn css() -> Option<NamedFile> {
        let css_path = Path::new(relative!("../client/pico.css"));
        NamedFile::open(css_path).await.ok()
    }

    #[get("/docs.css")]
    pub async fn docs_css() -> Option<NamedFile> {
        let css_path = Path::new(relative!("../client/docs.css"));
        NamedFile::open(css_path).await.ok()
    }

    #[get("/<file..>")]
    pub async fn pkg(file: PathBuf) -> Option<NamedFile> {
        let pkg = Path::new(relative!("../client/pkg")).join(file);
        NamedFile::open(pkg).await.ok()
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root, named, static_files::favicon, submit])
        .mount("/css", routes![static_files::css, static_files::docs_css])
        .mount("/pkg", routes![static_files::pkg])
        .mount("/api", routes![api_call])
}
