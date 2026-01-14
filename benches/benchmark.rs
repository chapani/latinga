use criterion::{Criterion, black_box, criterion_group, criterion_main};
use latinga::{Oegirgich, Sozlama, Tartib};
use std::time::Duration;

// --- 1. CONVERSION BENCHMARK ---
fn conversion_benchmark(c: &mut Criterion) {
    let base_sentence = "Oʻzbekiston - kelajagi buyuk davlat! Maʼno va mantiq. \n";
    let input = base_sentence.repeat(10_000);

    let mut group = c.benchmark_group("conversion_group");
    group
        .sample_size(20)
        .measurement_time(Duration::from_secs(30));

    // PRE-INITIALIZE THE FACADE (Crucial!)
    // This builds the engine once, so you only measure the TRANSFORMATION speed.
    let ogirgich_kelgusi = Oegirgich::yangi(Sozlama::yangi(Tartib::Kelgusi));

    group.bench_function("convert_kelgusi", |b| {
        b.iter(|| {
            // Use the public .ogir() method
            ogirgich_kelgusi.oegir(black_box(&input))
        })
    });

    let ogirgich_joriy = Oegirgich::yangi(Sozlama::yangi(Tartib::Joriy));
    group.bench_function("convert_joriy", |b| {
        b.iter(|| ogirgich_joriy.oegir(black_box(&input)))
    });

    group.finish();
}

// --- 2. VALIDATION BENCHMARK ---
fn validation_benchmark(c: &mut Criterion) {
    let base_text = "Oʻzbekiston - kelajagi buyuk davlat! \n\
                     Yaxshi ma'no va mantiq. \n\
                     SHAXS va Shaxs. \n\
                     mashhur va olchoq patterns.\n";
    let input = base_text.repeat(10_000);

    let mut group = c.benchmark_group("validation_group");
    group
        .sample_size(20)
        .measurement_time(Duration::from_secs(30));

    // Kelgusi Mode Validation
    let config_kelgusi = Sozlama::yangi(Tartib::Kelgusi);
    let trans_kelgusi = Oegirgich::yangi(config_kelgusi);
    group.bench_function("validate_kelgusi", |b| {
        b.iter(|| trans_kelgusi.tekshir(black_box(&input), black_box(1000)))
    });

    // Joriy Mode Validation
    let config_joriy = Sozlama::yangi(Tartib::Joriy);
    let trans_joriy = Oegirgich::yangi(config_joriy);
    group.bench_function("validate_joriy", |b| {
        b.iter(|| trans_joriy.tekshir(black_box(&input), black_box(1000)))
    });

    group.finish();
}

// --- 3. REGISTRATION ---
criterion_group!(benches, conversion_benchmark, validation_benchmark);
criterion_main!(benches);
