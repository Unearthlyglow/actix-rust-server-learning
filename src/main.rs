//lock -
//runtime - Runtime describes software/instructions that are executed while your program is running, especially those instructions that you did not write explicitly, but are necessary for the proper execution of your code.

// Low-level languages like C have very small (if any) runtime. More complex languages like Objective-C, which allows for dynamic message passing, have a much more extensive runtime.

// You are correct that runtime code is library code, but library code is a more general term, describing the code produced by any library. Runtime code is specifically the code required to implement the features of the language itself.

//Ideas :: Embedded device idea :: device for guitar that translates physical movements with the guitar to the data sent to the amp related to feedback.

//What creates/causes feedback?

//Video Idea :: Forest in SVHS

// ---------------

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn index(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello {app_name}!") // <- response with app_name
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! It's Award")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

async fn apples() -> impl Responder {
    HttpResponse::Ok().body("Where are all of the apples, people?")
}

//How do I get a route to the index.html file?
async fn home() -> impl Responder {
    "index.html"
}

async fn shared_state(data: web::Data<AppStateWithCounter>) -> String {
    let mut counter = data.counter.lock().unwrap(); // <- get counter's MutexGuard
    *counter += 1; // <- access counter inside MutexGuard

    format!("Request number: {counter}") // <- response with count
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Note: web::Data created _outside_ HttpServer::new closure
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(counter.clone()) // <- register the created data
            .route("/sharedstate", web::get().to(shared_state))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(index)
            .service(web::scope("/apples").route("/award", web::get().to(apples))) //example of using Actix's Scope method :: An application's scope acts as a namespace for all routes, i.e. all routes for a specific application scope have the same url path prefix.
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
            .service(Files::new("/static", "./static").show_files_listing())
            .route("/index.html", web::get().to(home))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}

//The service() method -

//Request handler -
//A handler, or request handler, is a program that receives a client request for access to the service, and translates the request into a form that the service can understand.
