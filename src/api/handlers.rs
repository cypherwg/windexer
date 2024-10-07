use warp::Filter;
use crate::storage::database::Database;
use crate::utils::error::Error;

pub fn routes(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    get_compressed_account(db.clone())
        .or(get_compressed_balance(db.clone()))
        .or(get_compressed_token_balance(db))
}

fn get_compressed_account(
    db: Database,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("api" / "account" / String)
        .and(warp::get())
        .and_then(move |address: String| {
            let db = db.clone();
            async move {
                db.get_compressed_account(&address)
                    .await
                    .map(|account| warp::reply::json(&account))
                    .map_err(|e| warp::reject::custom(Error::from(e)))
            }
        })
}
