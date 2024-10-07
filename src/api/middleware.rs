use warp::Filter;

pub fn logging() -> impl Filter<Extract = (), Error = std::convert::Infallible> + Clone {
    warp::log("api")
}

pub fn cors() -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["Content-Type"])
}