use criterion::{black_box, Criterion};
use std::{thread, time::Duration};
use template_rust::helpers;

fn main() {
    let mut criterion = Criterion::default()
        .warm_up_time(Duration::from_secs(10))
        .measurement_time(Duration::from_secs(120))
        .configure_from_args();
    do_benchmark(&mut criterion);
    criterion.final_summary();
}

fn do_benchmark(criterion: &mut Criterion) {
    helpers::init_logging("WARN", &[]).expect("No user-input, so this should be fine.");

    criterion.bench_function("Do something", |b| b.iter(|| do_sth(black_box(1))));
}

//------------------------------------------------------------------------------------------------//

fn do_sth(millis: u64) {
    let duration = Duration::from_millis(millis);
    thread::sleep(duration);
}
