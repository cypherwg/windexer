use criterion::{criterion_group, criterion_main, Criterion};
use std::time::Duration;
use windexer::{grpc::RpcClient, indexer::Indexer, storage::Storage};

fn benchmark_indexing(c: &mut Criterion) {
    let rpc_client = RpcClient::new("https://api.mainnet-beta.solana.com".to_string());
    let storage = Storage::new("scylla://localhost:9042/windexer").unwrap();
    let indexer = Indexer::new(rpc_client, storage);

    c.bench_function("index_block", |b| {
        b.iter(|| {
            indexer.index_next_block().unwrap();
        })
    });
}

fn benchmark_query(c: &mut Criterion) {
    let storage = Storage::new("scylla://localhost:9042/windexer").unwrap();

    c.bench_function("query_account", |b| {
        b.iter(|| {
            storage.get_account("SomeAccountPubkey").unwrap();
        })
    });
}

criterion_group! {
    name = benches;
    config = Criterion::default().measurement_time(Duration::from_secs(10));
    targets = benchmark_indexing, benchmark_query
}
criterion_main!(benches);
