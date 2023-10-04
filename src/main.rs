use tower_http::services::{ServeDir, ServeFile};
use crate::state::AppState;

#[cfg(all(feature = "sqlite", feature = "postgres"))]
compile_error!("Enabling PostgresSQL and SQLite simultaneously is not supported.");

#[cfg(feature = "postgres")]
extern crate diesel_postgres as diesel;

#[cfg(feature = "sqlite")]
extern crate diesel_sqlite as diesel;


pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

mod configuration;
mod trace;

pub(crate) mod database;
pub(crate) mod model;
pub(crate) mod schema;

pub(crate) mod state;

#[tokio::main]
async fn main() -> Result<(), Error> {
    trace::initialize();
    let _s = tracing::info_span!("starting up").entered();
    let cf: configuration::Config = std::fs::File::open("./Config.toml")
        .map_err(|e| {
            tracing::error!(error = %e, "could not open configration file");
            e
        })?
        .try_into()
        .unwrap_or_default();
    
    tracing::info!("establishing database connection pool");
    let state = AppState {
        #[cfg(feature = "postgres")]
        db_conn_pool: database::create_pool::<diesel::PgConnection>(&cf)?,
        #[cfg(feature = "sqlite")]
        db_conn_pool: database::create_pool::<diesel::SqliteConnection>(&cf)?,
    };
    
    tracing::info!("constructing router");
    let app = axum::Router::new()
        .nest_service("/", ServeFile::new("templates/index.html"))
        .nest_service("/sty", ServeDir::new("styles/out"))
        .with_state(state);
    tracing::info!("start up successful");
    _s.exit();

    let _s = tracing::info_span!("online", addr = cf.address, port = cf.port).entered();
    tracing::info!("binding server to port");
    axum::Server::try_bind(&format!("{}:{}", cf.address, cf.port).parse()?)?
        .serve(app.into_make_service())
        .await?;
    tracing::info!("shutting down");
    Ok(())
}
