use actix_web::Responder;
use actix_web::{get, web, web::ServiceConfig};
use lindera_analyzer::analyzer::Analyzer;
use shuttle_actix_web::ShuttleActixWeb;

use std::path::PathBuf;
use std::fs;
// mod endpoints;

const HOME_DIR: &str = "dict";
// let dict_path = "ipadic-mecab-2_7_0/system.dic.zst";
// let dict_folder_path = "bccwj-suw+unidic-cwj-3_1_1";
const DICT_FOLDER_PATH: &str = "bccwj-suw+unidic-cwj-3_1_1-extracted+compact";

const DICT_FILE_PATH: &str = "system.dic.zst";


#[get("/")]
async fn hello_world() -> &'static str {
    "Hello World!"
}

#[get("/health")]
async fn health() -> &'static str {
    "OK"
}

// #[get("/keitaiso")]
// async fn keitaiso() -> String {
//     let dict_full_path = format!("{}/{}/{}", HOME_DIR, DICT_FOLDER_PATH, DICT_FILE_PATH);
//     // URLからtextを取得
//     let text = "ある日の超暮方(ほぼ夜)の事である。一人の下人が、クソデカい羅生門の完全な真下で雨やみを気持ち悪いほどずっと待ちまくっていた。";
//     let result_txt = api_lib::keitaiso::keitaiso(text, &dict_full_path);

//     // Stringベクタを１つの文字列にする

//     let text = result_txt.join("\n");

//     println!("{}", &text);

//     // String型を&'static strに変換
//     text
// }

#[get("/tokenize/{text}")]
async fn tokenize(text: web::Path<String>, analyzer: web::Data<Analyzer>) -> impl Responder {

    let tokens = analyzer.analyze(&mut text.into_inner()).unwrap(); // 形態素解析を実行します

    let result_txt = tokens.iter().map(|token| {
        format!(
            "{}, {:?}",
            token.text,
            token.details
        )
    }).collect::<Vec<String>>().join("\n");

    result_txt
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let path = PathBuf::from("lindera_ipadic_conf.json");
    let config_bytes = fs::read(path)?;
    let analyzer = Analyzer::from_slice(&config_bytes).unwrap();
    let analyzer_data = web::Data::new(analyzer);

    let config = move |cfg: &mut ServiceConfig| {
        cfg.app_data(analyzer_data.clone());
        cfg.service(hello_world);
        cfg.service(health);
        cfg.service(tokenize);
    };

    Ok(config.into())
}
