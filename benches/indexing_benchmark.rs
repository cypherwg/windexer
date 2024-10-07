use criterion::{black_box, criterion_group, criterion_main, Criterion};
use windexer::indexer::Indexer;
use windexer::rpc::MockRpcClient;
use windexer::storage::MockStorage;

fn indexing_benchmark(c: &mut Criterion) {
    let mock_rpc = MockRpcClient::new();
    let mock_storage = MockStorage::new();
    let indexer = Indexer::new(mock_rpc, mock_storage);

    c.bench_function("index_block", |b| {
        b.iter(|| {
            black_box(indexer.index_next_block());
        });
    });
}

criterion_group!(benches, indexing_benchmark);
criterion_main!(benches);