use actix_web::{App, HttpResponse, HttpServer, Responder, error, post, web};
use serde::Deserialize;
use serde::Serialize;
use solana_sdk::{signature::Keypair, signer::Signer, pubkey::Pubkey};
use std::str::FromStr;

#[derive(Serialize)]
struct KeypairResponse {
    success: bool,
    data: KeypairData,
}

#[derive(Serialize)]
struct KeypairData {
    public_key: String,
    secret_key: String,
}

#[post("/keypair")]
async fn gen_keypair() -> impl Responder {
    println!("Generating new Solana keypair...");
    let keypair = Keypair::new();
    let response = KeypairResponse {
        success: true,
        data: KeypairData {
            public_key: keypair.pubkey().to_string(),
            secret_key: keypair.to_base58_string(),
        },
    };
    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct CreateTokenRequest {
    mintAuthority: String,
    mint: String,
    decimals: u8,
}

#[derive(Serialize)]
struct CreateTokenResponse {
    success: bool,
    data: CreateTokenData,
}

#[derive(Serialize)]
struct CreateTokenData {
    program_id: String,
    accounts: TokenAccounts,
    instruction_data: String,
}

#[derive(Serialize)]
struct TokenAccounts {
    is_signer: bool,
    is_writable: bool,
}

#[post("/token/create")]
async fn create_token(req_body: web::Json<CreateTokenRequest>) -> impl Responder {
    println!("Creating new token...");
    println!("Mint Authority: {}", req_body.mintAuthority);
    println!("Mint: {}", req_body.mint);
    println!("Decimals: {}", req_body.decimals);

    let auth_kp = Keypair::from_base58_string(&req_body.mintAuthority);
    let mint = Keypair::from_base58_string(&req_body.mint);

    println!("Mint Authority Public Key: {}", auth_kp.pubkey());
    println!("Mint Public Key: {}", mint.pubkey());

    HttpResponse::Ok().body("Token created successfully")
}

#[derive(Deserialize)]
struct MintTokenRequest {
    mint: String,
    destination: String,
    authority: String,
    amount: u64,
}

#[derive(Serialize)]
struct MintTokenResponse {
    success: bool,
    data: MintTokenData,
}

#[derive(Serialize)]
struct MintTokenData {
    program_id: String,
    accounts: Vec<MintTokenAccount>,
    instruction_data: String,
}

#[derive(Serialize)]
struct MintTokenAccount {
    pubkey: String,
    is_signer: bool,
    is_writable: bool,
}

#[post("/token/mint")]
async fn mint_token(req_body: web::Json<MintTokenRequest>) -> impl Responder {
    println!("Minting token...");
    println!("Mint: {}", req_body.mint);
    println!("Destination: {}", req_body.destination);
    println!("Authority: {}", req_body.authority);
    println!("Amount: {}", req_body.amount);

    let mint_pubkey = match Pubkey::from_str(&req_body.mint) {
        Ok(pubkey) => pubkey,
        Err(_) => return HttpResponse::BadRequest().body("Invalid mint address"),
    };

    let destination_pubkey = match Pubkey::from_str(&req_body.destination) {
        Ok(pubkey) => pubkey,
        Err(_) => return HttpResponse::BadRequest().body("Invalid destination address"),
    };

    let authority_pubkey = match Pubkey::from_str(&req_body.authority) {
        Ok(pubkey) => pubkey,
        Err(_) => return HttpResponse::BadRequest().body("Invalid authority address"),
    };

    println!("Mint Public Key: {}", mint_pubkey);
    println!("Destination Public Key: {}", destination_pubkey);
    println!("Authority Public Key: {}", authority_pubkey);

    let spl_token_program_id = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    
    let instruction_data = format!("mint_to_{}", req_body.amount);
    let instruction_data_base64 = base64::encode(instruction_data.as_bytes());

    let response = MintTokenResponse {
        success: true,
        data: MintTokenData {
            program_id: spl_token_program_id.to_string(),
            accounts: vec![
                MintTokenAccount {
                    pubkey: mint_pubkey.to_string(),
                    is_signer: false,
                    is_writable: true,
                },
                MintTokenAccount {
                    pubkey: destination_pubkey.to_string(),
                    is_signer: false,
                    is_writable: true,
                },
                MintTokenAccount {
                    pubkey: authority_pubkey.to_string(),
                    is_signer: true,
                    is_writable: false,
                },
            ],
            instruction_data: instruction_data_base64,
        },
    };

    HttpResponse::Ok().json(response)
}

#[derive(Deserialize)]
struct SignMessageRequest {
    message: String,
    secret: String,
}

#[derive(Serialize)]
struct SignMessageResponse {
    success: bool,
    data: SignMessageData,
}

#[derive(Serialize)]
struct SignMessageData {
    signature: String,
    public_key: String,
    message: String,
}

#[post("/message/sign")]
async fn sign_message(req_body: web::Json<SignMessageRequest>) -> impl Responder {
    println!("Signing message: {}", req_body.message);

    let kp: Keypair = Keypair::from_base58_string(&req_body.secret);

    let message = req_body.message.as_bytes();
    let signature = kp.sign_message(message);
    let response = SignMessageResponse {
        success: true,
        data: SignMessageData {
            signature: signature.to_string(),
            public_key: kp.pubkey().to_string(),
            message: req_body.message.clone(),
        },
    };
    HttpResponse::Ok().json(response)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting Solana Keypair Generator Server on http://127.0.0.1:8080");
    HttpServer::new(|| App::new().service(gen_keypair).service(sign_message).service(create_token).service(mint_token))
        .bind(("0.0.0.0", 4000))?
        .run()
        .await
}