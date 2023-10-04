use axum::extract::FromRef;
use diesel::r2d2::{ConnectionManager, Pool};

#[cfg(feature = "postgres")]
use diesel::PgConnection;

#[cfg(feature = "sqlite")]
use diesel::SqliteConnection;

#[derive(Clone, FromRef)]
pub(crate) struct AppState {
    #[cfg(feature = "postgres")]
    pub(crate) db_conn_pool: Pool<ConnectionManager<PgConnection>>,
    #[cfg(feature = "sqlite")]
    pub(crate) db_conn_pool: Pool<ConnectionManager<SqliteConnection>>,
}
