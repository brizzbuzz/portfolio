use lazy_static::lazy_static;
use sqlx_postgres::PgPool;

lazy_static! {
    pub static ref PG_POOL: PgPool = {
        PgPool::connect_lazy("postgres://portfolio_admin:portfolio_password@localhost:5432/portfolio?search_path=public&sslmode=disable").expect("Failed to create pool")
    };
}
