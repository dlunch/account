CREATE TABLE "user_credentials" (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,

    type VARCHAR NOT NULL,
    login_id BYTEA NOT NULL,
    login_password BYTEA NOT NULL,
    nonce BYTEA NOT NULL,

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,

    FOREIGN KEY(user_id) REFERENCES "users"(id)
);

SELECT diesel_manage_updated_at('user_credentials');