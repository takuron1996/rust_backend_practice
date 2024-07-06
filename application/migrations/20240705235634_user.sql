-- userテーブルを生成
CREATE TABLE Users (
    id uuid PRIMARY KEY NOT NULL DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL,
    email VARCHAR(254) NOT NULL UNIQUE,
    refresh_token VARCHAR(60),
    password VARCHAR(60) NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- テーブル更新時に現在時刻でupdated_atを更新するトリガー
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER update_users_updated_at
BEFORE UPDATE ON Users
FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- pgcryptoモジュールの有効化
CREATE EXTENSION IF NOT EXISTS pgcrypto;

-- ハッシュ関数の作成
CREATE OR REPLACE FUNCTION hash_sensitive_data()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.refresh_token IS NOT NULL THEN
        NEW.refresh_token = crypt(NEW.refresh_token, gen_salt('bf'));
    END IF;
    NEW.password = crypt(NEW.password, gen_salt('bf'));
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER hash_sensitive_data_trigger
BEFORE INSERT OR UPDATE ON users
FOR EACH ROW
EXECUTE FUNCTION hash_sensitive_data();