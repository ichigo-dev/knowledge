# QZgenについて


## 仕様

- QZgenはユーザ向けツールであり、デスクトップアプリとして起動する
- QZgenはAWS LambdaによりRDSからクイズデータを取得する
- terms_registerはEC2上でcronにより毎日、日本時間の夜3時に定期実行される。このバイナリにより、GitHub上にあるknowledgeノートの内容がRDSに保存される。
- RDSのテーブル定義はmigrationsに記載
