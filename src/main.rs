use serde::{Deserialize, Serialize};
use std::fs;
use std::time::{SystemTime, UNIX_EPOCH};
use reqwest;
use anyhow::Result;
use jsonwebtoken::{encode, EncodingKey, Header, Algorithm};

#[derive(Deserialize)]
struct Credentials {
    private_key: String,
    client_email: String,
}

#[derive(Serialize)]
struct JwtClaims {
    iss: String,
    sub: String,
    aud: String,
    iat: u64,
    exp: u64,
    scope: String,
}

#[derive(Serialize)]
struct ChatRequest {
    contents: Vec<Content>,
    generation_config: GenerationConfig,
}

#[derive(Serialize)]
struct Content {
    role: String,
    parts: Vec<Part>,
}

#[derive(Serialize)]
struct Part {
    text: String,
}

#[derive(Serialize)]
struct GenerationConfig {
    temperature: f32,
    top_p: f32,
    top_k: i32,
    max_output_tokens: i32,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct ChatResponse {
    candidates: Vec<Candidate>,
}

#[derive(Deserialize)]
struct Candidate {
    content: ResponseContent,
}

#[derive(Deserialize)]
struct ResponseContent {
    parts: Vec<ResponsePart>,
}

#[derive(Deserialize)]
struct ResponsePart {
    text: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    // 서비스 계정 키 파일 읽기
    let key_file = fs::read_to_string("credentials.json")?;
    let credentials: Credentials = serde_json::from_str(&key_file)?;

    // JWT 생성
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs();
    let claims = JwtClaims {
        iss: credentials.client_email.clone(),
        sub: credentials.client_email.clone(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        iat: now,
        exp: now + 3600,
        scope: "https://www.googleapis.com/auth/cloud-platform".to_string(),
    };

    let header = Header::new(Algorithm::RS256);
    let jwt = encode(
        &header,
        &claims,
        &EncodingKey::from_rsa_pem(credentials.private_key.as_bytes())?
    )?;

    //println!("Final JWT: {}", jwt);

    // OAuth 토큰 얻기
    let client = reqwest::Client::new();
    let token_request = serde_json::json!({
        "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
        "assertion": jwt
    });

    let token_response = client
        .post("https://oauth2.googleapis.com/token")
        .header("Content-Type", "application/json")
        .json(&token_request)
        .send()
        .await?;

    let token_status = token_response.status();
    if !token_status.is_success() {
        let error_text = token_response.text().await?;
        println!("Token Error: Status: {}, Body: {}", token_status, error_text);
        anyhow::bail!("Token request failed");
    }

    let token_data: TokenResponse = token_response.json().await?;

    // Vertex AI API 호출
    let chat_request = ChatRequest {
        contents: vec![Content {
            role: "user".to_string(),
            parts: vec![Part {
                text: "Hello! How can you assist me today?".to_string(),
            }],
        }],
        generation_config: GenerationConfig {
            temperature: 0.7,
            top_p: 0.8,
            top_k: 40,
            max_output_tokens: 1024,
        },
    };

    let project_id = "oqoqai"; // 프로젝트 ID를 입력하세요
    let location = "us-central1";
    let model_id = "gemini-pro";

    let response = client
        .post(format!(
            "https://us-central1-aiplatform.googleapis.com/v1/projects/{}/locations/{}/publishers/google/models/{}:generateContent",
            project_id, location, model_id
        ))
        .bearer_auth(token_data.access_token)
        .header("Content-Type", "application/json")
        .json(&chat_request)
        .send()
        .await?;

    // 응답 상태 코드 확인
    let status = response.status();
    if !status.is_success() {
        let error_text = response.text().await?;
        println!("API Error: Status: {}, Body: {}", status, error_text);
        anyhow::bail!("API request failed");
    }

    // 응답 본문 읽기
    let response_text = response.text().await?;
    println!("Raw response: {}", response_text);

    // JSON 파싱
    let chat_response: ChatResponse = serde_json::from_str(&response_text)
        .map_err(|e| anyhow::anyhow!("Failed to parse response: {}", e))?;

    // 응답 출력
    if let Some(candidate) = chat_response.candidates.first() {
        for part in &candidate.content.parts {
            println!("AI: {}", part.text);
        }
    } else {
        println!("No response generated");
    }

    Ok(())
}