ALTER TABLE "users" ADD created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE "users" ADD updated_at TIMESTAMP NOT NULL;

SELECT diesel_manage_updated_at('users');

ALTER TABLE "cards" ADD created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP;
ALTER TABLE "cards" ADD updated_at TIMESTAMP NOT NULL;

SELECT diesel_manage_updated_at('cards');