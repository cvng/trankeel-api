-- CreateTable
CREATE TABLE "plans" (
    "id" UUID NOT NULL DEFAULT gen_random_uuid(),
    "code" TEXT NOT NULL,
    "price" DECIMAL(65,30),
    "subtitle" TEXT,
    "title" TEXT,

    CONSTRAINT "plans_pkey" PRIMARY KEY ("id")
);
