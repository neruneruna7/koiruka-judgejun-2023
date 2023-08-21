
// {
//     "index": "0",
//     "message_tree_id": "6ab24d72-0181-4594-a9cd-deaf170242fb",
//     "message_id": "6ab24d72-0181-4594-a9cd-deaf170242fb",
//     "parent_id": "nan",
//     "text": "Can you write a short introduction about the relevance of the term \"monopsony\" in economics? Please use examples related to potential monopsonies in the labour market and cite relevant research.",
//     "text_ja": "経済学における「モノプソニー」という用語の関連性について簡単な紹介を書くことができますか？労働市場の潜在的なモノプニーに関連する例を使用し、関連する研究を引用してください。",
//     "role": "prompter",
//     "lang": "en",
//     "ng_translation": "0.0",
//     "use_deepl": 0
// },

use std::fs::File;
use std::io::{Read, Write};
use serde::{Deserialize, Serialize};

pub fn json_get_text(output_path: &str, json_data: serde_json::Value, select_colmun: &str) -> anyhow::Result<()> {
    // println!("Hello, world!");
    // // カレントディレクトリのパスを取得する
    // let current_dir = std::env::current_dir().unwrap();
    // println!("current_dir: {:?}", current_dir);


    let mut texts = Vec::<String>::new();

    json_data.as_array().unwrap().iter().for_each(|message| {
        texts.push(
            message[select_colmun]
            .clone()
            .as_str().unwrap()
            .replace("\n", "")
        );
    });

    let out_file_path = "data/texts1.csv";
    println!("{:?}", &texts);

    if let Err(err) = write_csv(texts, out_file_path) {
        eprintln!("Error writing CSV: {}", err);
    } else {
        println!("CSV file written successfully!");
    }
    // Now you can work with the parsed data

    Ok(())
}

fn write_csv(data: Vec<String>, file_path: &str) -> anyhow::Result<()> {
    let mut file = File::create(file_path)?;

    for line in data {
        file.write_all(line.as_bytes())?;
        file.write_all(b",\n")?; // Add a newline after each line
    }

    Ok(())
}