CREATE TABLE "professional_warrants" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "name" TEXT NOT NULL,
    "identifier" TEXT NOT NULL,

    CONSTRAINT "professional_warrants_pkey" PRIMARY KEY ("id")
);

SELECT manage_updated_at('professional_warrants');
