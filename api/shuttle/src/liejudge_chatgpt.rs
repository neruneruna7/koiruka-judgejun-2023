use chatgpt::prelude::*;
use dotenvy::dotenv;
use std::env;
use std::time::Instant;


use super::{SecretKeys, fake_check_request, fake_check_response};

use actix_web::{Responder, HttpResponse};
use actix_web::{get,post, web, web::ServiceConfig};
const PROMPT: &str = "
検証するコンテンツ：{}

タスク：検証するコンテンツについて以下のプロセスに従って噓かどうかの判定を行ってください．結果は以下のようにしてコードブロック内に表示してください
```
{{judge_possible_science: bool,judge_possible_logic: bool, true_percent: int, description: string}}
```
プロセス：
1. 検証するコンテンツが一般的な知識として事実であるかどうかを判断してください．一般的な事実でない場合は嘘の可能性が高いです．
2. 検証するコンテンツが文章として成り立っているかを判断してください．成り立っていない場合や非常識な内容の場合は嘘の可能性が高いです．
3．検証するコンテンツの事象を文章として抽出してください．
4．抽出した事実ごとにGoogleウェブ検索できる検索クエリを作成してください．
5．作成した検索クエリをもとにウェブ検索し，関連情報を収集してください．
6．収集した情報（エビデンス）をもとに検証するコンテンツの事象が科学的，論理的に正しいかを判断して，科学的判断が可能か難しいかをjudge_possible_science: boolで，論理的に判断が可能か難しいかをjudge_possible_logicでそれぞれtrue or falseで示したうえで，信憑性をtrue_percent: intで%表示してください．
";




#[post("/chatgpt")]
async fn lie_judge_gpt(
    client: web::Data<ChatGPT>,
    keys: web::Data<SecretKeys>,
    req: web::Json<fake_check_request>,
) -> impl Responder{

        if req.my_app_key != keys.my_app_key {
            return HttpResponse::Unauthorized().body("無効なリクエスト");
        }
        // レスポンスが返ってくるまでの時間を計測する
        let start = Instant::now();

        let response = client.send_message(&req.content).await.unwrap();
    
        let end = start.elapsed();
        println!("{}.{:03}秒", end.as_secs(), end.subsec_millis());


        // responseをjsonに変換
        let res_json: fake_check_response = serde_json::from_str(&response.message().content).unwrap();

        HttpResponse::Ok().json(res_json)

}