use actix_web::{web, App, HttpServer, HttpResponse, Error};
use reqwest::Client;
use serde::{Deserialize, Serialize};
#[derive(Deserialize, Serialize)]
struct GeminiRequest { contents: Vec<Content> }
#[derive(Deserialize, Serialize)]
struct Content { parts: Vec<Part> }
#[derive(Deserialize, Serialize)]
struct Part { text: String }
async fn proxy_gemini(
    req_body: web::Json<GeminiRequest>,
    client: web::Data<Client>, // Keep client for making HTTP requests
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, Error> {
    let api_key = req.headers().get("X-Gemini-API-Key").and_then(|v| v.to_str().ok()).map(|s| s.to_string());
    if api_key.is_none() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({"error": "Missing API key"})));
    }

    let url = format!("{}?key={}", "https://generativelanguage.googleapis.com/v1beta/models/gemini-2.0-flash:generateContent", api_key.unwrap());
    let response = client.post(&url).json(&req_body.into_inner()).send().await.map_err(|_| actix_web::error::ErrorInternalServerError("Request failed"))?;

    if response.status().is_success() {
        let body: serde_json::Value = response.json().await.map_err(|_| actix_web::error::ErrorInternalServerError("Parse failed"))?;
        Ok(HttpResponse::Ok().json(body))
    } else {
        Ok(HttpResponse::build(response.status()).body(response.text().await.unwrap_or_default()))
    }
}

fn create_app(client: Client) -> App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, Error = actix_web::Error, InitError = ()>> {
    App::new().app_data(web::Data::new(client)).route("/proxy", web::post().to(proxy_gemini))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| create_app(Client::new())).bind("0.0.0.0:8089")?.run().await
}