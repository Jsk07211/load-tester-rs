use axum::{
    Json, Router,
    routing::{get, post},
};

async fn add2(Json(body): Json<u32>) -> Json<u32> {
    println!("{}", body + 2);
    Json(body + 2)
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/ping", get(|| async { "pong!\n" }))
        .route("/add2", post(add2));

    // run our app with hyper, listening globally on port 8080
    println!("Listening at port 8080...");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
