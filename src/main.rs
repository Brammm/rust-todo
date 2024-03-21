use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;
use serde::Deserialize;
use maud::html;

struct Todo {
    description: String,
    finished: bool,
}

struct AppState {
    todos: Mutex<Vec<Todo>>,
}

#[derive(Deserialize)]
struct Create {
    description: String,
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();

    let markup = html! {
        h1 { "Hi, world"}
        p { "These are your todos: " }
        ul {
            @for todo in todos.iter() {
                li { 
                    input type="checkbox" checked[todo.finished];
                    span {(todo.description) }
                }
            }
            li {
                form action="/todo" method="post" {
                    input name="description";
                    button type="submit" { "Add" }
                }
            }
        }
    };
    HttpResponse::Ok().body(markup.into_string())
}

#[post("/todo")]
async fn create(form: web::Form<Create>, data: web::Data<AppState>) -> impl Responder {
    let form = form.into_inner();
    let mut todos = data.todos.lock().unwrap();

    todos.push(Todo { description: form.description, finished: false });

    HttpResponse::Found().append_header(("Location", "/")).finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = web::Data::new(AppState {
        todos: Mutex::new(vec![Todo { description: String::from("Learn Rust"), finished: true }, Todo { description: String::from("Create small Todo app"), finished: false}]),
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