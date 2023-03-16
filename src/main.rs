//lock -
//runtime - Runtime describes software/instructions that are executed while your program is running, especially those instructions that you did not write explicitly, but are necessary for the proper execution of your code.

// Low-level languages like C have very small (if any) runtime. More complex languages like Objective-C, which allows for dynamic message passing, have a much more extensive runtime.

// You are correct that runtime code is library code, but library code is a more general term, describing the code produced by any library. Runtime code is specifically the code required to implement the features of the language itself.

//Ideas :: Embedded device idea :: device for guitar that translates physical movements with the guitar to the data sent to the amp related to feedback.

//What creates/causes feedback?

//Video Idea :: Forest in SVHS

// ---------------
//Actix / Networking Specific Info ::

//01 -- Use App::service for the handlers using routing macros and App::route for manually routed handlers, declaring the path and method.

//02 -- Request handler - A handler, or request handler, is a program that receives a client request for access to the service, and translates the request into a form that the service can understand.

//03 -- Extrctors??

//04 -- TLS??

//05 --

//06 --

//07 --

//08 --

//05 --

//06 --

//07 --

//08 --

//05 --

//06 --

//07 --

//08 --

//----

use actix_files::Files;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use handlebars::Handlebars;
use serde_json::json;
use std::sync::Mutex;

struct AppState {
    app_name: String,
}

struct AppStateWithCounter {
    counter: Mutex<i32>, // <- Mutex is necessary to mutate safely across threads
}

#[get("/")]
async fn app_name(data: web::Data<AppState>) -> String {
    let app_name = &data.app_name; // <- get app_name
    format!("Hello My Friend this is {app_name}!") // <- response with app_name
}

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world! It's Award")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

//This is an example of manual routing
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

async fn index(hb: web::Data<Handlebars<'_>>) -> HttpResponse {
    let data = json!({
        "project_name": "Book Store",
        "books":[
            {
                "name":"Harry Potter",
                "author":"J K Rowlings",
                "image_path":"/static/image/download.jpeg"
            },
            {
                "name":"Lord of the ring",
                "author":"Tolken",
                "image_path": "/static/image/lord_of.jpeg"
            },
            {
                "name":"Americanah",
                "author":"Chimamada Adichie",
                "image_path":"/static/image/americanah.jpeg"
            },
            {
                "name":"Elon Musk",
                "author":"#####",
                "image_path":"/static/image/elon.jpeg"
            },
        ]


    });

    let body = hb.render("index", &data).unwrap();
    HttpResponse::Ok().body(body)
}

#[actix_web::main]

async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateWithCounter {
        counter: Mutex::new(0),
    });

    let mut handlebars = Handlebars::new();
    handlebars
        .register_templates_directory(".html", "./static/")
        .unwrap();
    let handlebars_ref = web::Data::new(handlebars);

    HttpServer::new(move || {
        App::new()
            .service(Files::new("/static", "static").show_files_listing())
            .app_data(handlebars_ref.clone())
            .route("/", web::get().to(index))
            .app_data(counter.clone()) // <- register the created data
            .route("/sharedstate", web::get().to(shared_state))
            .app_data(web::Data::new(AppState {
                app_name: String::from("Actix Web"),
            }))
            .service(app_name) //example of a use of a routing macro with the .service() method. See line 34 for the function.
            .service(web::scope("/apples").route("/award", web::get().to(apples))) //example of using Actix's Scope method :: An application's scope acts as a namespace for all routes, i.e. all routes for a specific application scope have the same url path prefix.
            .service(hello)
            //example of a manual route with the .route() method.
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8090))?
    .run()
    .await
}
