CREATE TYPE personrole AS ENUM ('admin', 'tenant', 'user', 'viewer', 'warrant');

CREATE TABLE "persons" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "account_id" UUID NOT NULL,
    "auth_id" TEXT,
    "email" TEXT NOT NULL,
    "first_name" TEXT NOT NULL,
    "last_name" TEXT NOT NULL,
    "address" JSONB,
    "photo_url" TEXT,
    "role" PERSONROLE,
    "phone_number" TEXT,

    CONSTRAINT "persons_pkey" PRIMARY KEY ("id")
);

CREATE UNIQUE INDEX "persons_auth_id_key" ON "persons"("auth_id");

ALTER TABLE "persons" ADD CONSTRAINT "persons_account_id_fkey" FOREIGN KEY ("account_id") REFERENCES "accounts"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

SELECT manage_updated_at('persons');
