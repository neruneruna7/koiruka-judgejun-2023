use chatgpt::prelude::*;
use dotenvy::dotenv;
use std::env;
// レスポンスが返ってくるまでの時間を計測する
use std::time::Instant;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // chatGPTのAPIkeyを.envから取得
    let key = env::var("CHATGPT_API_KEY").expect("CHATGPT_API_KEY is not set in .env");

    // chatGPTのAPIkeyを設定
    let mut client = ChatGPT::new(key)?;
    client.config.engine = chatgpt::config::ChatGPTEngine::Gpt35Turbo;

    let check_text = "
    ある日の超暮方(ほぼ夜)の事である。一人の下人が、クソデカい羅生門の完全な真下で雨やみを気持ち悪いほどずっと待ちまくっていた。
    ";

    let fake_chack_request = format!("
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
    ", check_text);

    // レスポンスが返ってくるまでの時間を計測する
    let start = Instant::now();

    let response = client.send_message(fake_chack_request).await?;

    let end = start.elapsed();
    println!("{}.{:03}秒", end.as_secs(), end.subsec_millis());

    println!("Response: {}", response.message().content);

    Ok(())
}
