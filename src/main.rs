use tower_http::services::{ServeDir, ServeFile};

use crate::state::AppState;

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;

mod configuration;
mod trace;

pub(crate) mod database;
pub(crate) mod schema;
pub(crate) mod model;

pub(crate) mod state;

#[tokio::main]
async fn main() -> Result<(), Error> {
    trace::initialize();

    let cf: configuration::Config = std::fs::File::open("./Config.toml")
        .map_err(|e| {
            tracing::error!(error = %e, "could not open configration file");
            e
        })?
        .try_into()
        .unwrap_or_default();

    let state = AppState {
        db_conn_pool: database::create_pool::<diesel::PgConnection>(&cf)?,
    };

    let app = axum::Router::new()
        .nest_service("/", ServeFile::new("templates/index.html"))
        .nest_service("/sty", ServeDir::new("styles/out"))
        .nest_service("/templ", ServeDir::new("templates"))
        .with_state(state);

    tracing::info!("Starting up");
    let _s = tracing::info_span!("Server Online", addr = cf.address, port = cf.port).entered();
    axum::Server::try_bind(&format!("{}:{}", cf.address, cf.port).parse()?)?
        .serve(app.into_make_service())
        .await?;
    tracing::info!("Server Shutting Down.");
    Ok(())
}
