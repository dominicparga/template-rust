use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::{thread, time};
use template_rust::helpers;

fn criterion_benchmark(c: &mut Criterion) {
    helpers::init_logging("WARN", vec![]).expect("No user-input, so this should be fine.");

    c.bench_function("Do something", |b| b.iter(|| do_sth(black_box(1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

//------------------------------------------------------------------------------------------------//

fn do_sth(millis: u64) {
    let duration = time::Duration::from_millis(millis);
    thread::sleep(duration);
}
