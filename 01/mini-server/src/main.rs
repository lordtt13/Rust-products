use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize};

#[derive(Serialize)]
struct User {
    user_id: u32,
    name: &'static str,
}

static USERS: [User; 2] = [
    User { user_id: 0, name: "John Doe" },
    User { user_id: 1, name: "Jane Doe" },
];

#[get("/")]
async fn hello_world() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[get("/users/{user_id}")]
async fn get_user(user_id: web::Path<u32>) -> impl Responder {
    let user = USERS.iter().find(|user| user.user_id == *user_id);
    
    match user {
        Some(user) => {
            let body = serde_json::to_string(user).unwrap();

            HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
        },
        None => HttpResponse::NotFound().body("User not found"),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
        .service(hello_world)
        .service(get_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
