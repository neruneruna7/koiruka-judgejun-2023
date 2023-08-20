use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;


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

#[get("/keitaiso")]
async fn keitaiso() -> String {
    let dict_full_path = format!("{}/{}/{}", HOME_DIR, DICT_FOLDER_PATH, DICT_FILE_PATH);
    // URLからtextを取得
    let text = "ある日の超暮方(ほぼ夜)の事である。一人の下人が、クソデカい羅生門の完全な真下で雨やみを気持ち悪いほどずっと待ちまくっていた。";
    let result_txt = api_lib::keitaiso::keitaiso(text, &dict_full_path);

    // Stringベクタを１つの文字列にする

    let text = result_txt.join("\n");

    println!("{}", &text);

    // String型を&'static strに変換
    text
}


#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
        cfg.service(health);
    };

    Ok(config.into())
}
