use warp::Filter;
use crate::storage::Storage;

pub async fn run_api_server(storage: impl Storage + Clone + Send + Sync + 'static) {
    let api = filters::accounts(storage)
        .or(filters::transactions(storage));

    warp::serve(api).run(([127, 0, 0, 1], 8080)).await;
}

mod filters {
    // TODO
}

mod handlers {
    // TODO
}