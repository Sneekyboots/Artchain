use axum::{Router, response::Html, routing::get,Json};
use serde_json::json;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Artchain backend server...");
    let app = Router::new()
        .route("/", get(home))
        .route("/api/health", get(health_check))
        .route("/api/nfts",get(list_nfts));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server is running at http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> Html<&'static str> {
    Html(
        "<h1>Welcome to Artchain</h1>
    <p>This is nft marketplace backend is running!</p>",
    )
}
async fn health_check() -> &'static str {
    "Artchain API is healthy and running smooth"
}
async fn list_nfts()->Json<serde_json::Value>{
    Json(json!({
        "nfts":[
            {
                "id":1,
                "name":"Cool Art #1",
                "price":0.6,
            "artist":"Artist1"            },
            {
                "id":2,
                "name":"Digital MasterPiece",
                "price":1.2,
            "artist":"Artist2" 
            }
            
        ],
        "total":2
    }))
}
