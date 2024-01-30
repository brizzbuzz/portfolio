-- Create "posts" table
CREATE TABLE "posts" (
  "id" uuid NOT NULL DEFAULT gen_random_uuid(),
  "title" character varying(100) NOT NULL,
  "description" character varying(100) NOT NULL,
  "body" text NOT NULL,
  "published_on" date NOT NULL,
  "created_at" timestamptz NOT NULL DEFAULT now(),
  "updated_at" timestamptz NOT NULL DEFAULT now(),
  PRIMARY KEY ("id")
);
