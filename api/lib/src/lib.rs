use chatgpt::prelude::*;
use dotenvy::dotenv;
use std::env;
// レスポンスが返ってくるまでの時間を計測する
use std::time::Instant;


pub async fn chatgpt_request(prompt: &str) -> Result<String>{
    dotenv().ok();

    // chatGPTのAPIkeyを.envから取得
    let key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY is not set in .env");

    // chatGPTのAPIkeyを設定
    let mut client = ChatGPT::new(key)?;
    client.config.engine = chatgpt::config::ChatGPTEngine::Gpt35Turbo;

    // レスポンスが返ってくるまでの時間を計測する
    let start = Instant::now();

    let response = client
        .send_message(prompt)
        .await?;

    let end = start.elapsed();
    println!("{}.{:03}秒", end.as_secs(), end.subsec_nanos() / 1_000_000);

    Ok(response.message().content.to_owned())
}

pub async fn fake_chack_gpt_request(check_text: &str) -> Result<String> {
    let fake_chack_request = format!("
    検証するコンテンツ：{}

    タスク：検証するコンテンツについて以下のプロセスに従って噓かどうかの判定を行ってください．結果は以下のようにしてコードブロック内に表示してください
    ```
    {{judge_possible_science: bool,judge_possible_logic: bool, true_percent: int}}
    ```
    プロセス：
    1. 検証するコンテンツが一般的な知識として事実であるかどうかを判断してください．一般的な事実でない場合は嘘の可能性が高いです．
    2. 検証するコンテンツが文章として成り立っているかを判断してください．成り立っていない場合や非常識な内容の場合は嘘の可能性が高いです．
    3．検証するコンテンツの事象を文章として抽出してください．
    4．抽出した事実ごとにGoogleウェブ検索できる検索クエリを作成してください．
    5．作成した検索クエリをもとにウェブ検索し，関連情報を収集してください．
    6．収集した情報（エビデンス）をもとに検証するコンテンツの事象が科学的，論理的に正しいかを判断して，科学的判断が可能か難しいかをjudge_possible_science: boolで，論理的に判断が可能か難しいかをjudge_possible_logicでそれぞれtrue or falseで示したうえで，信憑性をtrue_percent: intで%表示してください．
    ", check_text);

    let response = chatgpt_request(&fake_chack_request).await?;

    Ok(response)
}


#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // async fn gpt_test(){
    //     let response = chatgpt_request("プログラミング言語Rustを，5行で説明してください");

    //     println!("{:?}", response.await.unwrap())
    // }
}


