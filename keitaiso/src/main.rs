use std::{fs, path::PathBuf};

use lindera_analyzer::analyzer::Analyzer;
use lindera_core::LinderaResult;

fn main() -> LinderaResult<()> {
    // 設定ファイルのロード。ご自身のパスに修正してください。
    let path = PathBuf::from(r"keitaiso/lindera_ipadic_conf.json");
    let config_bytes = fs::read(path).unwrap();
    let analyzer = Analyzer::from_slice(&config_bytes).unwrap();

    let mut text = "
    ある日の超暮方(ほぼ夜)の事である。一人の下人が、クソデカい羅生門の完全な真下で雨やみを気持ち悪いほどずっと待ちまくっていた。

　馬鹿みたいに広い門の真下には、この大男のほかに全然誰もいない。ただ、所々丹塗のびっくりするくらい剥げた、信じられないほど大きな円柱に、象くらいある蟋蟀が一匹とまっている。クソデカ羅生門が、大河のように広い朱雀大路にある以上は、この狂った男のほかにも、激・雨やみをする巨大市女笠や爆裂揉烏帽子が、もう二三百人はありそうなものである。それが、この珍妙男のほかには全然誰もマジで全くいない。
    ".to_string();
    
    let tokens = analyzer.analyze(&mut text)?; // 形態素解析を実行します

    // 結果を出力します。
    for token in tokens {
        println!(
            "{}, {:?}",
            token.text,
            // token.byte_start,
            // token.byte_end,
            token.details
        );
    }

    Ok(())
}