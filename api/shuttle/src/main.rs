use std::path::PathBuf;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{get, web, web::ServiceConfig};
use actix_web::{post, HttpResponse, Responder};
use lindera_analyzer::analyzer::Analyzer;
use shuttle_actix_web::ShuttleActixWeb;

use chatgpt::prelude::*;

use serde::{Deserialize, Serialize};

use std::{env, fs};
mod liejudge_chatgpt;

// const HOME_DIR: &str = "dict";
// // let dict_path = "ipadic-mecab-2_7_0/system.dic.zst";
// // let dict_folder_path = "bccwj-suw+unidic-cwj-3_1_1";
// const DICT_FOLDER_PATH: &str = "bccwj-suw+unidic-cwj-3_1_1-extracted+compact";

// const DICT_FILE_PATH: &str = "system.dic.zst";

#[derive(Debug, Clone)]
pub struct SecretKeys {
    chagpt_api_key: String,
    my_app_key: String,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatGptRequest {
    my_app_key: String,
    content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ChatGptResponse {
    judge_possible_science: bool,
    judge_possible_logic: bool,
    true_percent: i32,
    description: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct FakeCheckResponse {
    chatgpt_response: ChatGptResponse,
    other_params: Option<String>,
}

#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/health")]
async fn health() -> &'static str {
    "OK"
}

#[get("/tokenize/{text}")]
async fn tokenize(text: web::Path<String>, analyzer: web::Data<Analyzer>) -> actix_web::Result<impl Responder> {
    let Ok(tokens) = analyzer.analyze(&mut text.into_inner()) else {
        return Err(actix_web::error::ErrorBadRequest("Failed to tokenize"));
    }; // 形態素解析を実行します

    let result_txt = tokens
        .iter()
        .map(|token| format!("{}, {:?}", token.text, token.details))
        .collect::<Vec<String>>()
        .join("\n");

    Ok(HttpResponse::Ok().body(result_txt))
}

#[post("/fake_check")]
async fn fake_check(
    client: web::Data<ChatGPT>,
    keys: web::Data<SecretKeys>,
    req: web::Json<ChatGptRequest>,
) -> impl Responder {
    let chatgpt_response = liejudge_chatgpt::lie_judge_gpt(client, keys, req).await;

    if let Err(e) = chatgpt_response {
        eprintln!("{}", e);
        return HttpResponse::InternalServerError().body(e.to_string());
    }

    let fake_check_response = FakeCheckResponse {
        chatgpt_response: chatgpt_response.unwrap(),
        other_params: None,
    };

    HttpResponse::Ok().json(fake_check_response)
}

#[shuttle_runtime::main]
async fn actix_web(
    // deploy時には有効にする
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    // env_logger::init();

    // 形態素解析用の設定
    let path = PathBuf::from(static_folder.join("lindera_ipadic_conf.json"));
    // let path = PathBuf::from("static/lindera_ipadic_conf.json");

    let config_bytes = fs::read(path)?;
    let analyzer = Analyzer::from_slice(&config_bytes).unwrap();
    let analyzer_data = web::Data::new(analyzer);

    // ChatGPTの設定

    dotenvy::from_path(static_folder.join(".env")).ok();
    // dotenvy::from_path("static/.env").ok();

    // chatGPTのAPIkeyを.envから取得
    let gpt_key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY is not set in .env");
    let my_app_key = env::var("MY_APP_KEY").expect("MY_APP_KEY is not set in .env");

    let secret_keys = SecretKeys {
        chagpt_api_key: gpt_key,
        my_app_key,
    };

    let sercret_keys_data = web::Data::new(secret_keys.clone());

    // chatGPTのAPIkeyを設定
    let mut client = ChatGPT::new(secret_keys.chagpt_api_key).unwrap();
    client.config.engine = chatgpt::config::ChatGPTEngine::Gpt4;

    let client_data = web::Data::new(client);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(analyzer_data.clone())
            .app_data(client_data.clone())
            .app_data(sercret_keys_data.clone())
            .service(
                web::scope("")
                    .wrap(Logger::default())
                    .wrap(
                        Cors::default()
                            .allow_any_origin()
                            .allow_any_method()
                            .allow_any_header(),
                    )
                    .service(hello_world)
                    .service(health)
                    .service(tokenize)
                    .service(fake_check),
            );
    };

    Ok(config.into())
}
