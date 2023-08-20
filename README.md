# Open Hack U 2023 コイルカ - usotsukey - judgejun
このリポジトリは、Open Hack U 2023 コイルカチームの usotsukey のリポジトリその2です。

## 利用技術
- Rust
- Actix-web
- Shuttle

## 注意点
本リポジトリではActixと形態素解析などを組み合わせる（予定）だが，ActixはTokioベースの実装であり，IOバウンドな処理を想定している.
CPUバウンド向きではないことに留意．