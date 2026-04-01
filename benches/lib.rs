use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use localize_it::init_locale;

init_locale!(En, Ru, storage = true);

expression!(TEST => {En: "Test", Ru: "Тест"});

fn bench(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("bench");

    group.bench_function("locale_from_storage", |bencher| {
        bencher.iter(|| black_box(localize!(TEST)))
    });

    group.bench_function("manual_locale", |bencher| {
        bencher.iter(|| black_box(localize!(TEST, Locale::En)))
    });

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
