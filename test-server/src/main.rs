use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Person {
    name: String,
    age: u32,
}

async fn add2(Json(body): Json<u32>) -> Json<u32> {
    println!("{}", body + 2);
    Json(body + 2)
}

async fn person(Json(body): Json<Person>) -> String {
    format!("{} is {} years old!", body.name, body.age)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(|| async { "pong!" }))
        .route("/add2", post(add2))
        .route("/person", post(person));

    // run our app with hyper, listening globally on port 8080
    println!("Listening at port 8080...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
