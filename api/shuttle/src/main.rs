use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

mod endpoints;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/health")]
async fn health() -> &'static str {
    "OK"
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(health);
    };

    Ok(config.into())
}
