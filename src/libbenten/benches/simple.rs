use criterion::{ criterion_group, criterion_main, Criterion };
use benten::{ BentenEngine, BentenConfig };

fn simple(c: &mut Criterion) {
    let config = BentenConfig { id:"kana".to_string() };
    let mut engine = BentenEngine::new(&config);

    c.bench_function("simple 5", |b| {
        b.iter(|| {
            for _ in 0..5 {
                engine.on_key_press(20);
            }
        })
    });

    c.bench_function("simple 50", |b| {
        b.iter(|| {
            for _ in 0..50 {
                engine.on_key_press(20);
            }
        })
    });

    c.bench_function("simple 500", |b| {
        b.iter(|| {
            for _ in 0..500 {
                engine.on_key_press(20);
            }
        })
    });
}

criterion_group!(benches, simple);
criterion_main!(benches);