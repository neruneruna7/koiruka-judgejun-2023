use chatgpt::prelude::*;

use std::time::Instant;

use super::{ChatGptRequest, ChatGptResponse, SecretKeys};

use actix_web::{web};


pub async fn lie_judge_gpt(
    client: web::Data<ChatGPT>,
    keys: web::Data<SecretKeys>,
    req: web::Json<ChatGptRequest>,
) -> Result<ChatGptResponse> {
    if req.my_app_key != keys.my_app_key {
        return Err(chatgpt::err::Error::ParsingError("Invalid my_app_key".to_string()));
    }
    // レスポンスが返ってくるまでの時間を計測する
    let start = Instant::now();

    let req_prompt = format!("
検証するコンテンツ：{}

タスク：検証するコンテンツについて以下のプロセスに従って噓かどうかの判定を行ってください．結果は以下のようにしてコードブロック内に表示してください
```
{{judge_possible_science: bool,judge_possible_logic: bool, true_percent: int, description: string}}
```
プロセス：
1. 検証するコンテンツが一般的な知識として事実であるかどうかを判断してください．一般的な事実でない場合は嘘の可能性が高いです．
2. 検証するコンテンツが文章として成り立っているかを判断してください．成り立っていない場合や非常識な内容の場合は嘘の可能性が高いです．
3.検証するコンテンツの事象を文章として抽出してください.
4.抽出した事実ごとにGoogleウェブ検索できる検索クエリを作成してください.
5.作成した検索クエリをもとにウェブ検索し，関連情報を収集してください．
6.収集した情報（エビデンス）をもとに検証するコンテンツの事象が科学的,論理的に正しいかを判断して,科学的判断が可能か難しいかをjudge_possible_science: boolで,論理的に判断が可能か難しいかをjudge_possible_logicでそれぞれtrue or falseで示したうえで,信憑性をtrue_percent: intで%表示してください．
    ", req.content);

    let response = client.send_message(&req_prompt).await.unwrap();

    let end = start.elapsed();
    println!("{}.{:03}秒", end.as_secs(), end.subsec_millis());
    println!("{}", &response.message().content);

    // responseを構造体に変換
    Ok(parse_response(&response.message().content))
}

// 独自の文字列パーサー
// 今回は，以下のような形式の文字列をパースする
// {judge_possible_science: false, judge_possible_logic: true, true_percent: 60, description: "This content is not a general knowledge and it is difficult to verify scientifically. However, it is logically possible. The credibility is 60%."}
// この文字列をパースして，以下の構造体に変換する
// struct fake_check_response {
//     judge_possible_science: bool,
//     judge_possible_logic: bool,
//     true_percent: i32,
//     description: String,
// }
fn parse_response(response: &str) -> ChatGptResponse {
    // 文字列中から, judge_possible_scienceの位置を探す
    let judge_possible_science_index = response.find("judge_possible_science:").unwrap();
    // 文字列中から, judge_possible_logicの位置を探す
    let judge_possible_logic_index = response.find("judge_possible_logic:").unwrap();
    // 文字列中から, true_percentの位置を探す
    let true_percent_index = response.find("true_percent:").unwrap();
    // 文字列中から, descriptionの位置を探す
    let description_index = response.find("description:").unwrap();

    // judge_possible_scienceとjudge_possible_logicの間の文字列を取得
    let judge_possible_science_str =
        &response[judge_possible_science_index..judge_possible_logic_index];
    // judge_possible_logicとtrue_percentの間の文字列を取得
    let judge_possible_logic_str = &response[judge_possible_logic_index..true_percent_index];
    // true_percentとdescriptionの間の文字列を取得
    let true_percent_str = &response[true_percent_index..description_index];
    // descriptionと文字列の最後の間の文字列を取得
    let description_str = &response[description_index..];

    // judge_possible_science_strの中にtrueという文字列があるかどうかを判定
    let judge_possible_science = judge_possible_science_str.contains("true");
    // judge_possible_logic_strの中にtrueという文字列があるかどうかを判定
    let judge_possible_logic = judge_possible_logic_str.contains("true");
    // true_percent_strの中に数字があるかどうかを判定
    let true_percent = true_percent_str
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect::<String>()
        .parse::<i32>()
        .unwrap();
    // description_strのうち，余計な文字列を削除
    let description = description_str
        .replace("description:", "")
        .replace(['}', '\"'], "")
        .trim()
        .to_string();

        ChatGptResponse {
        judge_possible_science,
        judge_possible_logic,
        true_percent,
        description,
    }
}

#[test]
fn test_parse_response() {
    let response = "{judge_possible_science: false, judge_possible_logic: true, true_percent: 60, description: \"This content is not a general knowledge and it is difficult to verify scientifically. However, it is logically possible. The credibility is 60%.\"}";
    let res_json = parse_response(response);
    eprintln!("{:?}", &res_json);
    assert!(!res_json.judge_possible_science);
    assert!(res_json.judge_possible_logic);
    assert_eq!(res_json.true_percent, 60);
    assert_eq!(res_json.description, "This content is not a general knowledge and it is difficult to verify scientifically. However, it is logically possible. The credibility is 60%.");
}
