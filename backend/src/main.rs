use axum::{Router, response::Html, routing::get,Json,extract::Path};
use serde_json::json;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use serde::{Deserialize,Serialize};

#[tokio::main]
async fn main() {
    println!("🚀 Starting Artchain backend server...");
    let app = Router::new()
        .route("/", get(home))
        .route("/api/health", get(health_check))
        .route("/api/nfts",get(list_nfts))
        .route("/api/wallet/{address}",get(get_wallet_info))
        .route("/api/nft/{id}",get(get_nft))
        .route("/api/nfts/user/{address}",get(get_user_nfts));
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
async fn list_nfts() -> Json<Vec<Nft>> {
    let nfts = vec![
        Nft {
            id: 1,
            name: "Cool Art #1".to_string(),
            description: "A beautiful digital artwork".to_string(),
            image_url: "https://picsum.photos/id/237/200/300".to_string(),
            artist: "Artist1".to_string(),
            price_sol: 0.6,
            owner: "JCK39hgE32NjNTbjCfUfh4uhwP9sm7KRL6GBSCpfWYS4".to_string(),
            mint_address: None,
            is_listed: true,
        },
        Nft {
            id: 2,
            name: "Digital Masterpiece".to_string(),
            description: "An amazing piece of art".to_string(),
            image_url: "https://picsum.photos/id/237/200/300".to_string(),
            artist: "Artist2".to_string(),
            price_sol: 1.2,
            owner: "JCK39hgE32NjNTbjCfUfh4uhwP9sm7KRL6GBSCpfWYS4".to_string(),
            mint_address: None,
            is_listed: true,
        }
    ];
    
    Json(nfts)
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

#[derive(Serialize,Deserialize,Clone)]
pub struct Nft{
    pub id:u32,
    pub name:String,
    pub description:String,
    pub image_url:String,
    pub artist:String,
    pub price_sol:f64,
    pub owner:String,
    pub mint_address:Option<String>,
    pub is_listed:bool,
}

async fn get_nft(Path(id):Path<u32>)->Json<serde_json::Value>{
    if id==1{
        Json(json!({
            "id": 1,
            "name": "Cool Art #1",
            "description": "A beautiful digital artwork",
            "image_url": "https://example.com/image1.jpg",
            "artist": "Artist1",
            "price_sol": 0.6,
            "owner": "JCK39hgE32NjNTbjCfUfh4uhwP9sm7KRL6GBSCpfWYS4",
            "mint_address": null,
            "is_listed": true
        }))
    }else{
        Json(json!({"error":"NFT not found "}))
    }
}
async fn get_user_nfts(Path(address):Path<String>)->Json<serde_json::Value>{

    Json(json!({
        "owner": address,
        "nfts": [
            {
                "id": 1,
                "name": "Cool Art #1",
                "price_sol": 0.6
            }
        ],
        "total": 1
    }))
}

