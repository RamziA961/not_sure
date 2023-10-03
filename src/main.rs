use tower_http::services::{ServeDir, ServeFile};

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) mod trace;
mod configuration;

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

    let app = axum::Router::new()
        .nest_service("/", ServeFile::new("templates/index.html"))
        .nest_service("/sty", ServeDir::new("styles/out"))
        .nest_service("/templ", ServeDir::new("templates"));

    let _s = tracing::info_span!("Server Online", addr = cf.address, port = cf.port).entered();
    tracing::info!("Starting up");
    axum::Server::try_bind(&format!("{}:{}", cf.address, cf.port).parse()?)?
        .serve(app.into_make_service())
        .await?;

    tracing::info!("Server Shutting Down.");
    Ok(())
}
