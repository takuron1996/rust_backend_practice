use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, http::header, web, App, HttpServer, Responder, Result};
use env_logger::Env;

mod middleware;
mod response;

/// ヘルスチェックAPI
#[get("health")]
async fn health() -> Result<impl Responder> {
    let health_response = response::health::HealthResponse {
        status: "pass".to_string(),
    };
    Ok(web::Json(health_response))
}

/// CORSの設定
/// LOCAL用
fn get_cors() -> actix_cors::Cors {
    Cors::default()
        .allowed_origin("http://localhost:9000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .supports_credentials()
        .max_age(3600)
}

// App インスタンスを作成し、リクエスト ハンドラーを登録
// ルーティング マクロを使用するハンドラーには App::service を使用
// 手動でルーティングされるハンドラーには App::route を使用して、パスとメソッドを宣言
// #[actix_web::main] マクロは、actix ランタイム内で非同期メイン関数を実行
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let cors: Cors = get_cors();
        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .wrap(middleware::logging::Logging)
            .service(web::scope("/api").service(health))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
