mod prometheus;

use warp::Filter;

pub fn start_server(port: u16) -> anyhow::Result<impl std::future::Future<Output = ()>> {
    let metrics = warp::path!("metrics").and_then(prometheus::metrics_handler);
    Ok(warp::serve(metrics).run(([0, 0, 0, 0], port)))
}