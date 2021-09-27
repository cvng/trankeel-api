CREATE TABLE "lenders" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT current_timestamp,
    "updated_at" TIMESTAMPTZ,
    "account_id" UUID NOT NULL,
    "individual_id" UUID,
    "company_id" UUID,

    CONSTRAINT "lenders_pkey" PRIMARY KEY ("id")
);

ALTER TABLE "lenders" ADD CONSTRAINT "lenders_account_id_fkey" FOREIGN KEY ("account_id") REFERENCES "accounts"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

ALTER TABLE "lenders" ADD CONSTRAINT "lenders_individual_id_fkey" FOREIGN KEY ("individual_id") REFERENCES "persons"("id") ON DELETE SET NULL ON UPDATE CASCADE;

ALTER TABLE "lenders" ADD CONSTRAINT "lenders_company_id_fkey" FOREIGN KEY ("company_id") REFERENCES "companies"("id") ON DELETE SET NULL ON UPDATE CASCADE;

SELECT manage_updated_at('lenders');
