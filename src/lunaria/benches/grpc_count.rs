use criterion::{criterion_group, criterion_main, Criterion};

use lunaria_api::counter::count_service_client::CountServiceClient;
use lunaria_api::counter::GetCountRequest;
use tokio::runtime::Runtime;

fn sequential(c: &mut Criterion) {
    let mut rt = Runtime::new().unwrap();

    let mut client = rt
        .block_on(CountServiceClient::connect("http://127.0.0.1:10000"))
        .unwrap();

    c.bench_function("sequential", |b| {
        b.iter(|| {
            rt.block_on(client.get_count(GetCountRequest {})).unwrap();
        });
    });
}

criterion_group!(benches, sequential);
criterion_main!(benches);
