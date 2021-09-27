-- CreateTable
CREATE TABLE "files" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "created_at" TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    "updated_at" TIMESTAMPTZ,
    "download_url" TEXT,
    "external_id" TEXT,
    "filename" TEXT,
    "preview_url" TEXT,
    "status" TEXT,
    "type" TEXT NOT NULL,

    CONSTRAINT "files_pkey" PRIMARY KEY ("id")
);

-- ManageUpdatedAt
SELECT manage_updated_at('files');
