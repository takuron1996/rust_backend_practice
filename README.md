# rust_backend_practice
Rustのバックエンド練習

# 環境変数

| 変数名 | 内容 |
|--|--|
| ALLOWED_ORIGIN | CORSで許可するURL |
| POSTGRES_USER | postgreSQLのユーザー |
| POSTGRES_PASSWORD | postgreSQLのパスワード |
| POSTGRES_HOST | postgreSQLのホスト |
| POSTGRES_PORT | postgreSQLのポート |
| POSTGRES_DB | postgreSQLのDB |

# コマンド

## マイグレーションファイル作成

```sh
$ sqlx migrate add ${ファイル名}
```

## マイグレーション実行

```sh
$ sqlx migrate run --database-url postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DB
```

# 参考

- [sqlx-cli](https://crates.io/crates/sqlx-cli)
