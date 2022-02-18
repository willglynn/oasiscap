use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn generate(alert: &oasiscap::Alert) -> String {
    alert.to_string()
}

macro_rules! bench_generate {
    ( $c:expr, $name:literal ) => {
        $c.bench_function(concat!("generate ", $name), |b| {
            let input = include_str!(concat!("../fixtures/", $name));
            let alert = input.parse().unwrap();
            b.iter(|| black_box(generate(black_box(&alert))));
        })
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_generate!(c, "v1dot0_appendix_adot1.xml");
    bench_generate!(c, "v1dot0_appendix_adot2.xml");
    bench_generate!(c, "v1dot0_appendix_adot3.xml");
    bench_generate!(c, "v1dot0_appendix_adot4.xml");

    bench_generate!(c, "v1dot1_appendix_adot1.xml");
    bench_generate!(c, "v1dot1_appendix_adot2.xml");
    bench_generate!(c, "v1dot1_appendix_adot3.xml");
    bench_generate!(c, "v1dot1_appendix_adot4.xml");

    bench_generate!(c, "v1dot2_appendix_adot1.xml");
    bench_generate!(c, "v1dot2_appendix_adot2.xml");
    bench_generate!(c, "v1dot2_appendix_adot3.xml");
    bench_generate!(c, "v1dot2_appendix_adot4.xml");

    bench_generate!(c, "nws-5c2cf27b1f56885d61654dc47fa411d5.xml");

    bench_generate!(c, "ipaws-5e6dd964023f1930ef638846.xml");
    bench_generate!(c, "ipaws-5e6dd9de023f1930ef6548d9.xml");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
