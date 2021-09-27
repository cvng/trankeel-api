CREATE TABLE "companies" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "address" JSONB,
    "email" TEXT NOT NULL,
    "legal_entity" TEXT NOT NULL,
    "legal_entity_identifier" TEXT,
    "legal_entity_type" TEXT,
    "legal_entity_type_other" TEXT,
    "phone_number" TEXT,

    CONSTRAINT "companies_pkey" PRIMARY KEY ("id")
);

SELECT manage_updated_at('companies');
