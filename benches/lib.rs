mod common;

use common::{get_locale_as_usize, Locale, ENTER_YOUR_NAME, HELLO};
use core::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use localize_it::localize;

const STR: &'static str = ENTER_YOUR_NAME[0];
const NAME: &'static str = black_box("Ivan");
const LOCALE: Locale = black_box(Locale::En);

fn default(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("bench");

    group.bench_function("str", |bencher| bencher.iter(|| STR));

    group.bench_function("format", |bencher| {
        bencher.iter(|| format!("Hello, {NAME}!"))
    });

    group.finish();
}

fn localize_it(criterion: &mut Criterion) {
    let mut group = criterion.benchmark_group("bench");

    group.bench_function("expression", |bencher| {
        bencher.iter(|| localize!(ENTER_YOUR_NAME))
    });

    group.bench_function("expression_manual_locale", |bencher| {
        bencher.iter(|| localize!(ENTER_YOUR_NAME, LOCALE))
    });

    group.bench_function("callable_expression", |bencher| {
        bencher.iter(|| localize!(HELLO as (NAME)))
    });

    group.finish();
}

criterion_group!(benches, default, localize_it);
criterion_main!(benches);
