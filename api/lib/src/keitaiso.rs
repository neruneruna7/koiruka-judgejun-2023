// 形態素解析用のプロジェクト
use std::fs::File;
use vibrato::{Dictionary, Tokenizer};

pub fn keitaiso(text: &str, dict_full_path: &str) {
    // 辞書ファイルのロード
    let reader = zstd::Decoder::new(File::open(dict_full_path).unwrap()).unwrap();

    let dict = Dictionary::read(reader).unwrap();

    // トークナイザーの生成
    let tokenizer = Tokenizer::new(dict)
        .ignore_space(true)
        .unwrap()
        .max_grouping_len(64);

    // ワーカーの生成
    let mut worker = tokenizer.new_worker();

    // 文章をセット。繰り返したい場合は、これを再度呼び出し、ワーカーを使い回す。
    worker.reset_sentence(text);
    worker.tokenize(); // 形態素解析の実行。mutable self

    println!("num_tokens: {}", worker.num_tokens());

    // 抽出したトークンをループで表示する
    worker
        .token_iter()
        // .filter(|t| {
        //     // 絞り込み
        //     let words: Vec<&str> = t.feature().split(',').collect();
        //     let subwords: Vec<&str> = words[0].split('-').collect();
        //     subwords[0] == "名詞" || subwords[0] == "カスタム名詞"
        // })
        .for_each(|t| {
            // 出力
            println!("{}: {}", t.surface(), t.feature());
        });
}
