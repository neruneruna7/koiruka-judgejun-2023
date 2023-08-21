use actix_web::web::to;
use actix_web::{get, web, web::ServiceConfig};
use actix_web::{HttpResponse, Responder, post};
// use lindera_analyzer::analyzer::Analyzer;
use shuttle_actix_web::ShuttleActixWeb;
use shuttle_static_folder::StaticFolder;

use std::fs;
use std::path::PathBuf;

use chatgpt::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use std::env;

mod liejudge_chatgpt;
mod load_key;

// const HOME_DIR: &str = "dict";
// // let dict_path = "ipadic-mecab-2_7_0/system.dic.zst";
// // let dict_folder_path = "bccwj-suw+unidic-cwj-3_1_1";
// const DICT_FOLDER_PATH: &str = "bccwj-suw+unidic-cwj-3_1_1-extracted+compact";

// const DICT_FILE_PATH: &str = "system.dic.zst";

#[derive(Debug, Clone)]
pub struct SecretKeys {
    chagpt_api_key: &'static str,
    my_app_key: &'static str,
}

#[derive(Debug, serde::Deserialize)]
pub struct ChatGptRequest {
    my_app_key: String,
    content: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct  ChatGptResponse {
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

// #[get("/tokenize/{text}")]
// async fn tokenize(text: web::Path<String>, analyzer: web::Data<Analyzer>) -> impl Responder {
//     let tokens = analyzer.analyze(&mut text.into_inner()).unwrap(); // 形態素解析を実行します

//     let result_txt = tokens
//         .iter()
//         .map(|token| format!("{}, {:?}", token.text, token.details))
//         .collect::<Vec<String>>()
//         .join("\n");

//     result_txt
// }

#[post("/fake_check")]
async fn fake_check(
    client: web::Data<ChatGPT>,
    keys: web::Data<SecretKeys>,
    req: web::Json<ChatGptRequest>,
) -> impl Responder{
    let chatgpt_response = liejudge_chatgpt::lie_judge_gpt(client, keys, req).await;

    let fake_check_response = FakeCheckResponse {
        chatgpt_response: chatgpt_response.unwrap(),
        other_params: None,
    };

    HttpResponse::Ok().json(fake_check_response)
}



#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    
    // 形態素解析用の設定
    // let path = PathBuf::from("lindera_ipadic_conf.json");
    // let config_bytes = fs::read(path)?;
    // let analyzer = Analyzer::from_slice(&config_bytes).unwrap();
    // let analyzer_data = web::Data::new(analyzer);

    // ChatGPTの設定
    // dotenv().ok();

    // chatGPTのAPIkeyを.envから取得
    // let key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY is not set in .env");
    // let my_app_key = env::var("MY_APP_KEY").expect("MY_APP_KEY is not set in .env");
    let secret_keys = load_key::keys();
    let sercret_keys_data = web::Data::new(secret_keys.clone());


    // chatGPTのAPIkeyを設定
    let mut client = ChatGPT::new(secret_keys.chagpt_api_key).unwrap();
    client.config.engine = chatgpt::config::ChatGPTEngine::Gpt35Turbo;

    let client_data = web::Data::new(client);

    let config = move |cfg: &mut ServiceConfig| {
        // cfg.app_data(analyzer_data.clone());
        cfg.app_data(client_data.clone());
        cfg.app_data(sercret_keys_data.clone());
        cfg.service(hello_world);
        cfg.service(health);
        // cfg.service(tokenize);
        cfg.service(fake_check);
    };

    Ok(config.into())
}
