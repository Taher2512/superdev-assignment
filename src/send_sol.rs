use serde::{Deserialize, Serialize};
use solana_sdk::{
    pubkey::Pubkey,
    system_instruction,
};
use axum::{Json, http::StatusCode};
use std::str::FromStr;
use base64::{Engine as _, engine::general_purpose};

#[derive(Serialize)]
pub struct ErrorResponse {
    success: bool,
    error: String,
}

#[derive(Deserialize)]
pub struct SendSolRequest {
    from: String,
    to: String,
    lamports: u64,
}

#[derive(Serialize)]
pub struct SendSolResponse {
    success: bool,
    data: SendSolData,
}

#[derive(Serialize)]
pub struct SendSolData {
    program_id: String,
    accounts: Vec<String>,
    instruction_data: String,
}

fn is_valid_pubkey(pubkey_str: &str) -> bool {
    match Pubkey::from_str(pubkey_str) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn is_valid_lamports(lamports: u64) -> bool {
    lamports > 0 && lamports <= 1_000_000_000_000_000
}

pub async fn send_sol(
    Json(payload): Json<SendSolRequest>,
) -> Result<Json<SendSolResponse>, (StatusCode, Json<ErrorResponse>)> {
    if payload.from.is_empty() || payload.to.is_empty() {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Missing required fields".to_string(),
        })));
    }

    if !is_valid_lamports(payload.lamports) {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Invalid lamports amount".to_string(),
        })));
    }

    if !is_valid_pubkey(&payload.from) {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Invalid sender address".to_string(),
        })));
    }

    if !is_valid_pubkey(&payload.to) {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Invalid recipient address".to_string(),
        })));
    }

    let from_pubkey = Pubkey::from_str(&payload.from).unwrap();
    let to_pubkey = Pubkey::from_str(&payload.to).unwrap();

    if from_pubkey == to_pubkey {
        return Err((StatusCode::BAD_REQUEST, Json(ErrorResponse {
            success: false,
            error: "Cannot send SOL to the same address".to_string(),
        })));
    }

    let instruction = system_instruction::transfer(
        &from_pubkey,
        &to_pubkey,
        payload.lamports,
    );

    let instruction_data = general_purpose::STANDARD.encode(&instruction.data);

    let accounts: Vec<String> = instruction
        .accounts
        .iter()
        .map(|acc| acc.pubkey.to_string())
        .collect();

    let response = SendSolResponse {
        success: true,
        data: SendSolData {
            program_id: instruction.program_id.to_string(),
            accounts,
            instruction_data,
        },
    };

    Ok(Json(response))
}
