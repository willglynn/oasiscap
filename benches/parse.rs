use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn parse(input: &str) -> oasiscap::Alert {
    input.parse().unwrap()
}

macro_rules! bench_parse {
    ( $c:expr, $name:literal ) => {
        $c.bench_function(concat!("parse ", $name), |b| {
            let input = include_str!(concat!("../fixtures/", $name));
            b.iter(|| black_box(parse(black_box(input))));
        })
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    bench_parse!(c, "v1dot0_appendix_adot1.xml");
    bench_parse!(c, "v1dot0_appendix_adot2.xml");
    bench_parse!(c, "v1dot0_appendix_adot3.xml");
    bench_parse!(c, "v1dot0_appendix_adot4.xml");

    bench_parse!(c, "v1dot1_appendix_adot1.xml");
    bench_parse!(c, "v1dot1_appendix_adot2.xml");
    bench_parse!(c, "v1dot1_appendix_adot3.xml");
    bench_parse!(c, "v1dot1_appendix_adot4.xml");

    bench_parse!(c, "v1dot2_appendix_adot1.xml");
    bench_parse!(c, "v1dot2_appendix_adot2.xml");
    bench_parse!(c, "v1dot2_appendix_adot3.xml");
    bench_parse!(c, "v1dot2_appendix_adot4.xml");

    bench_parse!(c, "nws-5c2cf27b1f56885d61654dc47fa411d5.xml");

    bench_parse!(c, "ipaws-5e6dd964023f1930ef638846.xml");
    bench_parse!(c, "ipaws-5e6dd9de023f1930ef6548d9.xml");
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
