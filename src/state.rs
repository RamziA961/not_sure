use std::sync::Arc;

use axum::extract::FromRef;
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};

#[derive(Clone, FromRef)]
pub(crate) struct AppState {
    pub(crate) db_conn_pool: Pool<ConnectionManager<PgConnection>>,
}
