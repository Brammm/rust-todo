use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use serde::Deserialize;
use maud::html;

struct AppState {
    todos: Mutex<Vec<String>>,
}

#[derive(Deserialize)]
struct Create {
    name: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();

    let markup = html! {
        h1 { "Hi, world"}
        ul {
            @for todo in todos.iter() {
                li { (todo) }
            }
        }
    };
    HttpResponse::Ok().body(markup.into_string())
}

#[post("/todo")]
async fn create(form: web::Form<Create>, data: web::Data<AppState>) -> impl Responder {
    let form = form.into_inner();
    let mut todos = data.todos.lock().unwrap();

    todos.push(form.name);

    web::Redirect::to("/")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        todos: Mutex::new(vec![String::from("Learn Rust"), String::from("Create small Todo app")]),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(index)
            .service(create)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}