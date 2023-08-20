use lindera_analyzer::analyzer::Analyzer;
use lindera_dictionary::{DictionaryConfig, DictionaryKind};
use lindera_core::{LinderaResult, dictionary, mode::Mode};
use lindera_tokenizer::tokenizer::{TokenizerConfig, Tokenizer};


fn main() -> LinderaResult<()>{
    let mut text = "
    ある日の超暮方(ほぼ夜)の事である。一人の下人が、クソデカい羅生門の完全な真下で雨やみを気持ち悪いほどずっと待ちまくっていた。

　馬鹿みたいに広い門の真下には、この大男のほかに全然誰もいない。ただ、所々丹塗のびっくりするくらい剥げた、信じられないほど大きな円柱に、象くらいある蟋蟀が一匹とまっている。クソデカ羅生門が、大河のように広い朱雀大路にある以上は、この狂った男のほかにも、激・雨やみをする巨大市女笠や爆裂揉烏帽子が、もう二三百人はありそうなものである。それが、この珍妙男のほかには全然誰もマジで全くいない。
    ";

    let dictionary = DictionaryConfig {
        kind: Some(DictionaryKind::IPADIC),
        path: None,
    };

    let config = TokenizerConfig {
        dictionary,
        user_dictionary: None,
        mode: Mode::Normal,
    };

    let tokenizer = Tokenizer::from_config(config)?;

    let tokens = tokenizer.tokenize(text)?;

    tokens.iter().for_each(|token| {
        println!("{}", token.text);
    });

    Ok(())
}