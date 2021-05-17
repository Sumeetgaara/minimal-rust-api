use actix_web::{get,web, App, HttpResponse, HttpServer, Responder};
use mobc_redis::RedisConnectionManager;
use mobc_redis::redis;
use redis::AsyncCommands;
use serde::Deserialize;
type Pool = mobc::Pool<RedisConnectionManager>;


#[derive(Deserialize)]
struct Info {
    key: String,
}

#[get("/api/data/{key}")]
async fn fetch_data(pool: web::Data<Pool>,info: web::Path<Info>) -> impl Responder {
    let mut conn = pool.get().await.unwrap();
    let s: String = conn.get(info.key.to_string()).await.unwrap();
    HttpResponse::Ok().body(s)
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let client = redis::Client::open("redis://127.0.0.1:6379").unwrap();
    let manager = RedisConnectionManager::new(client);
    let pool = Pool::builder().max_open(100).build(manager);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(fetch_data)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}