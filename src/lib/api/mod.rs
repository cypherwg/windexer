mod handlers;
mod middleware;

use warp::Filter;
use crate::storage::database::Database;

pub fn start_server(port: u16, db: Database) -> anyhow::Result<impl std::future::Future<Output = ()>> {
    let api = handlers::routes(db)
        .with(middleware::logging())
        .with(middleware::cors());
    
    Ok(warp::serve(api).run(([0, 0, 0, 0], port)))
}