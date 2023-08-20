// 形態素解析用のプロジェクト
use std::fs::File;
use vibrato::{Dictionary, Tokenizer};

fn main() {
    let home_dir = "dict";
    // let dict_path = "ipadic-mecab-2_7_0/system.dic.zst";
    // let dict_folder_path = "bccwj-suw+unidic-cwj-3_1_1";
    let dict_folder_path = "bccwj-suw+unidic-cwj-3_1_1-extracted+compact";

    let dict_file_path = "system.dic.zst";

    let dict_full_path = format!("{}/{}/{}", home_dir, dict_folder_path, dict_file_path);
    // let user_lex_csv = format!("{}/{}", home_dir, "user_lex.csv");

    // 形態素解析する文章
    let text = "
    ある日の超暮方(ほぼ夜)の事である。一人の下人が、クソデカい羅生門の完全な真下で雨やみを気持ち悪いほどずっと待ちまくっていた。
s    　馬鹿みたいに広い門の真下には、この大男のほかに全然誰もいない。ただ、所々丹塗のびっくりするくらい剥げた、信じられないほど大きな円柱に、象くらいある蟋蟀が一匹とまっている。クソデカ羅生門が、大河のように広い朱雀大路にある以上は、この狂った男のほかにも、激・雨やみをする巨大市女笠や爆裂揉烏帽子が、もう二三百人はありそうなものである。それが、この珍妙男のほかには全然誰もマジで全くいない。
    ";

    keitaiso(text, &dict_full_path);

}

fn keitaiso(text: &str, dict_full_path: &str) {
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
