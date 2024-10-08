mod handlers;
mod middleware;

use crate::storage::database::Database;
use warp::Filter;

pub fn start_server(
    port: u16,
    storage: Arc<Database>,
) -> Result<impl std::future::Future<Output = Result<(), anyhow::Error>>> {
    let api = handlers::routes(db)
        .with(middleware::logging())
        .with(middleware::cors());

    Ok(warp::serve(api).run(([0, 0, 0, 0], port)))
    unimplemented!("API server not implemented yet")
}
