use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx_postgres::PgPool;
use uuid::Uuid;

use domain::post::Post;

use crate::connection_pool::PG_POOL;

struct PostRepository<'a> {
    pool: &'a PgPool,
}

#[derive(Serialize, Deserialize, FromRow)]
struct PostEntity {
    id: Uuid,
    title: String,
    description: String,
    body: String,
    published_on: NaiveDate,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl From<PostEntity> for Post {
    fn from(entity: PostEntity) -> Self {
        Self {
            id: Some(entity.id),
            title: entity.title,
            body: entity.body,
            published: false,
        }
    }
}

impl<'a> PostRepository<'a> {
    pub fn new() -> Self {
        Self { pool: &PG_POOL }
    }

    fn new_from_pool(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    async fn get_all(&self) -> Result<Vec<Post>, sqlx::Error> {
        let post_entities = sqlx::query_as::<_, PostEntity>("SELECT * FROM posts")
            .fetch_all(self.pool)
            .await?;
        let posts = post_entities
            .into_iter()
            .map(|entity| entity.into())
            .collect();
        Ok(posts)
    }
}

#[cfg(test)]
mod tests {
    use testcontainers::{clients, RunnableImage};
    use testcontainers_modules::postgres::Postgres;

    use persistence_test_utils::atlas_image::AtlasImage;

    use super::*;

    #[tokio::test]
    async fn test_get_all_empty() {
        // Arrange
        let docker = clients::Cli::default();
        let network = "db";
        let postgres_config = Postgres::default().with_db_name("portfolio");
        let postgres_image = RunnableImage::from(postgres_config)
            .with_tag("15-alpine")
            .with_network(network)
            .with_container_name("postgres");
        let postgres_instance = docker.run(postgres_image);
        let postgres_port = postgres_instance.get_host_port_ipv4(5432);
        let connection_string = format!(
            "postgres://postgres:postgres@127.0.0.1:{}/postgres?sslmode=disable",
            postgres_port
        );

        let atlas_config = AtlasImage::new().with_postgres_port(postgres_port);
        let atlas_image = RunnableImage::from(atlas_config)
            .with_network(network);
        let atlas_instance = docker.run(atlas_image);

        // Get the id of the migration_instance
        let container_id = atlas_instance.id();

        // Wait for the migration_instance to exit
        let output = std::process::Command::new("docker")
            .arg("wait")
            .arg(&container_id)
            .output()
            .expect("Failed to wait for container to exit");

        println!("Migration container exited with status: {}", output.status);
        println!(
            "Migration container stdout:\n {}\n",
            String::from_utf8_lossy(&output.stdout)
        );
        println!(
            "Migration container stderr:\n {}\n",
            String::from_utf8_lossy(&output.stderr)
        );

        let pool = PgPool::connect(&connection_string).await.unwrap();
        let repository = PostRepository::new_from_pool(&pool);

        // Act
        let posts = repository.get_all().await.unwrap();

        // Assert
        assert_eq!(posts.len(), 0);
    }
}
