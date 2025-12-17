use actix_web::{web, App, HttpServer, HttpResponse, Error};
use reqwest::Client;
use serde_json;
async fn proxy_gemini(
    query: web::Query<std::collections::HashMap<String, String>>,
    client: web::Data<Client>,
) -> Result<HttpResponse, Error> {
    // Hardcoded Gemini API Key
    let api_key = std::env::var("GEMINI_API_KEY")
        .map_err(|_| actix_web::error::ErrorInternalServerError("Missing GEMINI_API_KEY env variable"))?;
    let text = match query.get("text") {
        Some(t) => t,
        None => return Ok(HttpResponse::BadRequest().json(serde_json::json!({"error": "Missing 'text' query parameter"}))),
    };

    let url = "https://gemini.weolopez.com/proxy";
    let payload = serde_json::json!({
        "contents": [
            { "parts": [ { "text": text } ] }
        ]
    });

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("X-Gemini-API-Key", api_key)
        .json(&payload)
        .send()
        .await
        .map_err(|_| actix_web::error::ErrorInternalServerError("Request failed"))?;

    if response.status().is_success() {
        let body: serde_json::Value = response.json().await.map_err(|_| actix_web::error::ErrorInternalServerError("Parse failed"))?;
        // Extract only the content text
        let text = body["candidates"]
            .get(0)
            .and_then(|c| c["content"]["parts"].get(0))
            .and_then(|p| p["text"].as_str())
            .map(|s| s.to_owned()) // <-- clone the string
            .unwrap_or_default();
        Ok(HttpResponse::Ok().body(text))
    } else {
        Ok(HttpResponse::build(response.status()).body(response.text().await.unwrap_or_default()))
    }
}

fn create_app(client: Client) -> App<impl actix_web::dev::ServiceFactory<actix_web::dev::ServiceRequest, Config = (), Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>, Error = actix_web::Error, InitError = ()>> {
    App::new()
        .app_data(web::Data::new(client))
        .route("/", web::get().to(proxy_gemini))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| create_app(Client::new())).bind("0.0.0.0:8089")?.run().await
}