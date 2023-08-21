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
// このjsonを格納する構造体を作成する

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    index: String,
    message_tree_id: String,
    message_id: String,
    parent_id: String,
    text: String,
    text_ja: String,
    role: String,
    lang: String,
    ng_translation: String,
    use_deepl: i64,
}

use serde::{Deserialize, Serialize};

//

// pub fn json_get_text<T: Deserialize>(output_path: &str, data:T) -> Result<(), Box<dyn std::error::Error>> {
//     // println!("Hello, world!");
//     // // カレントディレクトリのパスを取得する
//     // let current_dir = std::env::current_dir().unwrap();
//     // println!("current_dir: {:?}", current_dir);

//     let file_path = "data/oasst1_89k_ja.json";
//     let mut file = File::open(file_path).expect("Failed to open the file");

//     // Read the content of the file into a String
//     let mut file_content = String::new();
//     file.read_to_string(&mut file_content)
//         .expect("Failed to read the file");

//     // Deserialize the JSON content into your data structure
//     let parsed_data: Vec<Message> = serde_json::from_str(&file_content)
//         .expect("Failed to parse JSON");

//     let mut texts = Vec::<String>::new();

//     parsed_data.iter().for_each(|message| {
//         texts.push(
//             message.text_ja
//             .clone()
//             .replace("\n", "")
//         );
//     });

//     let out_file_path = "data/texts1.csv";

//     if let Err(err) = write_csv(texts, out_file_path) {
//         eprintln!("Error writing CSV: {}", err);
//     } else {
//         println!("CSV file written successfully!");
//     }

//     // Now you can work with the parsed data
//     // println!("{:?}", texts);
// }

// fn write_csv(data: Vec<String>, file_path: &str) -> anyhow::Result<()> {
//     let mut file = File::create(file_path)?;

//     for line in data {
//         file.write_all(line.as_bytes())?;
//         file.write_all(b",\n")?; // Add a newline after each line
//     }

//     Ok(())
// }
