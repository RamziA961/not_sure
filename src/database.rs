use diesel::r2d2::{ConnectionManager, Pool, R2D2Connection};

use crate::configuration::Config;

pub(crate) fn create_pool<C>(cf: &Config) -> Result<Pool<ConnectionManager<C>>, crate::Error>
where
    C: R2D2Connection + Send + 'static,
{
    let &Config {
        ref db_address,
        ref db_pool,
        ..
    } = cf;

    Pool::builder()
        .min_idle(Some((db_pool / 2).into()))
        .max_size(db_pool.clone().into())
        .build(ConnectionManager::<C>::new(db_address))
        .map_err(|e| {
            tracing::error!(
                error = %e,
                "database connection pool initialization failed",
            );
            e.into()
        })
}
