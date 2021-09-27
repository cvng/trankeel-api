-- CreateTable
CREATE TABLE "properties" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "account_id" UUID NOT NULL,
    "address" JSONB NOT NULL,
    "build_period" TEXT,
    "building_legal_status" TEXT,
    "common_spaces" TEXT,
    "energy_class" TEXT,
    "equipments" TEXT,
    "gas_emission" TEXT,
    "heating_method" TEXT,
    "housing_type" TEXT,
    "name" TEXT NOT NULL,
    "note" TEXT,
    "description" TEXT,
    "ntic_equipments" TEXT,
    "other_spaces" TEXT,
    "tax" DECIMAL(65,30),
    "room_count" TEXT NOT NULL,
    "status" TEXT,
    "surface" REAL NOT NULL,
    "tenant_private_spaces" TEXT,
    "usage_type" TEXT,
    "water_heating_method" TEXT,
    "lender_id" UUID NOT NULL,

    CONSTRAINT "properties_pkey" PRIMARY KEY ("id")
);

-- AddForeignKey
ALTER TABLE "properties" ADD CONSTRAINT "properties_account_id_fkey" FOREIGN KEY ("account_id") REFERENCES "accounts"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "properties" ADD CONSTRAINT "properties_lender_id_fkey" FOREIGN KEY ("lender_id") REFERENCES "lenders"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- ManageUpdatedAt
SELECT manage_updated_at('properties');
