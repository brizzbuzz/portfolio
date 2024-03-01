schema "public" {}

table "posts" {
  schema = schema.public

  // Columns

  column "id" {
    null = false
    type = uuid
    default = sql("gen_random_uuid()")
  }

  column "title" {
    null = false
    type = varchar(100)
  }

  column "description" {
    null = false
    type = varchar(100)
  }

  column "body" {
    null = false
    type = text
  }

  column "published_on" {
    null = false
    type = date
  }

  column "created_at" {
    null = false
    type = timestamptz
    default = sql("now()")
  }

  column "updated_at" {
    null = false
    type = timestamptz
    default = sql("now()")
  }

  // Constraints

  primary_key {
    columns = [column.id]
  }
}
