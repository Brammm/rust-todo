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

#[get("/")]
async fn index(data: web::Data<AppState>) -> impl Responder {
    let todos = data.todos.lock().unwrap();

    let markup = html! {
        h1 { "Hello world"}
        p { "These are your todos: " }
        ul {
            @for (i, todo) in todos.iter().enumerate() {
                li { 
                    form action="/toggle" method="post" {
                        input type="hidden" name="index" value=(i);
                        label for=(i) {
                            input id=(i) type="checkbox" checked[todo.finished] name="finished" value="checked" onChange="this.form.submit()";
                            @if todo.finished {
                                s {(todo.description)}
                            } @else {
                                (todo.description)
                            }
                            
                        }
                    }
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

#[derive(Deserialize)]
struct Create {
    description: String,
}

#[post("/todo")]
async fn create(form: web::Form<Create>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();

    todos.push(Todo { description: form.description.clone(), finished: false });

    HttpResponse::Found().append_header(("Location", "/")).finish()
}

#[derive(Deserialize)]
struct Toggle {
    index: usize,
    finished: Option<String>,
}

#[post("/toggle")]
async fn toggle(form: web::Form<Toggle>, data: web::Data<AppState>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();

    todos[form.index].finished = matches!(form.finished.clone(), Some(_i));

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
            .service(toggle)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}