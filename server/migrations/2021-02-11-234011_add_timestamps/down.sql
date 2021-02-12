ALTER TABLE "users" DROP COLUMN created_at;
ALTER TABLE "users" DROP COLUMN updated_at;

DROP TRIGGER set_updated_at ON "users";

ALTER TABLE "cards" DROP COLUMN created_at;
ALTER TABLE "cards" DROP COLUMN updated_at;

DROP TRIGGER set_updated_at ON "cards";