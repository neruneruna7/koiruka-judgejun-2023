# Open Hack U 2023 コイルカ - usotsukey - judgejun
このリポジトリは、Open Hack U 2023 コイルカチームの usotsukey のリポジトリその2です。

## 利用技術
- Rust
- Actix-web
- Shuttle

### Rust採用理由
- ChatGPTを使うことが確定していたかつ，自身があまり時間がとれない都合上，あまりテストができないので実行時エラーを嫌がった
- 余力があれば他の処理の組み込む予定だったため，高速な処理ができることがよさそうだと考えた．(その残滓として https://koiruka-judgejun-2023.shuttleapp.rs/tokenize/{text} にGETリクエストを送ると形態素解析ができる)
- Shuttleにより，デプロイが非常に簡単だった
- 個人の趣味

## 注意点
本リポジトリではActixと形態素解析などを組み合わせる（予定）だが，ActixはTokioベースの実装であり，IOバウンドな処理を想定している.
CPUバウンド向きではないことに留意．


## 現在の異常または問題


## 解決済みの異常または問題
`cargo shuttle deploy`時に.envファイルやdictフォルダなどを含めることができない．  


