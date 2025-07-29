use axum::{Router, response::Html, routing::get,Json,extract::Path};
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

#[tokio::main]
async fn main() {
    println!("🚀 Starting Artchain backend server...");
    let app = Router::new()
        .route("/", get(home))
        .route("/api/health", get(health_check))
        .route("/api/nfts",get(list_nfts))
        .route("/api/wallet/{address}",get(get_wallet_info));
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

async fn get_wallet_info(Path(address):Path<String>)->Json<serde_json::Value>{
    let client =RpcClient::new("https://api.devnet.solana.com".to_string());
    match Pubkey::from_str(&address){
        Ok(pubkey)=>{
            match client.get_balance(&pubkey){
                Ok(balance)=>{
                    let sol_balance=balance as f64 /1_000_000_000.0;
                    Json(json!({
                        "address":address,
                        "balance_sol":sol_balance,
                        "balance_lamports":balance,
                        "network":"devnet"
                    }))
                },
                Err(_)=>Json(json!({"error":"Failed to get balance"}))
            }

        },
        Err(_)=>Json(json!({"error":"Invalid wallet address"}))
    }
}
