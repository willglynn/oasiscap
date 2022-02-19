use criterion::{
    criterion_group, criterion_main, AxisScale, BatchSize, BenchmarkId, Criterion,
    PlotConfiguration, Throughput,
};
use std::str::FromStr;

macro_rules! fixture {
    ( $name:literal ) => {
        ($name, include_str!(concat!("../fixtures/", $name, ".xml")))
    };
}

fn criterion_benchmark(c: &mut Criterion) {
    let fixtures = [
        fixture!("v1dot0_appendix_adot1"),
        fixture!("v1dot0_appendix_adot2"),
        fixture!("v1dot0_appendix_adot3"),
        fixture!("v1dot0_appendix_adot4"),
        fixture!("v1dot1_appendix_adot1"),
        fixture!("v1dot1_appendix_adot2"),
        fixture!("v1dot1_appendix_adot3"),
        fixture!("v1dot1_appendix_adot4"),
        fixture!("v1dot2_appendix_adot1"),
        fixture!("v1dot2_appendix_adot2"),
        fixture!("v1dot2_appendix_adot3"),
        fixture!("v1dot2_appendix_adot4"),
        fixture!("nws-5c2cf27b1f56885d61654dc47fa411d5"),
        fixture!("ipaws-5e6dd964023f1930ef638846"),
        fixture!("ipaws-5e7e0fc5023f1930efcf3deb"),
        fixture!("ipaws-5ea321f39fc226a7b44b6874"),
    ];

    {
        let mut group = c.benchmark_group("oasiscap::Alert::from_str");
        group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        for (name, data) in fixtures {
            group.throughput(Throughput::Bytes(data.len() as u64));
            group.bench_with_input(BenchmarkId::from_parameter(name), data, |b, data| {
                b.iter_batched(
                    || data,
                    |data| oasiscap::Alert::from_str(data).unwrap(),
                    BatchSize::LargeInput,
                );
            });
        }
        group.finish();
    }

    {
        let mut group = c.benchmark_group("oasiscap::Alert::to_string");
        group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        for (name, data) in fixtures {
            let alert = oasiscap::Alert::from_str(data).unwrap();
            group.throughput(Throughput::Bytes(alert.to_string().len() as u64));
            group.bench_with_input(BenchmarkId::from_parameter(name), &alert, |b, alert| {
                b.iter_batched(|| alert, |alert| alert.to_string(), BatchSize::LargeInput);
            });
        }
        group.finish();
    }

    protobuf(c, &fixtures);
}

#[cfg(not(feature = "prost"))]
fn protobuf(c: &mut Criterion, fixtures: &[(&str, &str)]) {}

#[cfg(feature = "prost")]
fn protobuf(c: &mut Criterion, fixtures: &[(&str, &str)]) {
    fn generate(alert: &oasiscap::protobuf::Alert) -> Vec<u8> {
        prost::Message::encode_to_vec(alert)
    }

    {
        let mut group = c.benchmark_group("oasiscap::protobuf::Alert::encode_to_vec");
        group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        for (name, data) in fixtures {
            let alert = data.parse::<oasiscap::Alert>().unwrap();
            let alert = oasiscap::protobuf::Alert::from(alert);
            group.throughput(Throughput::Bytes(generate(&alert).len() as u64));
            group.bench_with_input(BenchmarkId::from_parameter(name), &alert, |b, alert| {
                b.iter_with_large_drop(|| prost::Message::encode_to_vec(alert))
            });
        }
        group.finish();
    }

    {
        let mut group = c.benchmark_group("oasiscap::protobuf::Alert into oasiscap::Alert");
        group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        for (name, data) in fixtures {
            let alert = data.parse::<oasiscap::Alert>().unwrap();
            let alert = oasiscap::protobuf::Alert::from(alert);
            group.throughput(Throughput::Bytes(generate(&alert).len() as u64));
            group.bench_with_input(BenchmarkId::from_parameter(name), &alert, |b, alert| {
                b.iter_batched(
                    || alert.clone(),
                    |alert| oasiscap::Alert::try_from(alert).unwrap(),
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    {
        let mut group = c.benchmark_group("oasiscap::Alert into oasiscap::protobuf::Alert");
        group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        for (name, data) in fixtures {
            let alert = data.parse::<oasiscap::Alert>().unwrap();
            group.throughput(Throughput::Bytes(
                generate(&oasiscap::protobuf::Alert::from(alert.clone())).len() as u64,
            ));
            group.bench_with_input(BenchmarkId::from_parameter(name), &alert, |b, alert| {
                b.iter_batched(
                    || alert.clone(),
                    oasiscap::protobuf::Alert::from,
                    BatchSize::LargeInput,
                )
            });
        }
        group.finish();
    }

    {
        let mut group = c.benchmark_group("oasiscap::protobuf::Alert::decode");
        group.plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));
        for (name, data) in fixtures {
            let alert = data.parse::<oasiscap::Alert>().unwrap();
            let alert = oasiscap::protobuf::Alert::from(alert);
            let bytes = prost::Message::encode_to_vec(&alert);
            group.throughput(Throughput::Bytes(bytes.len() as u64));
            group.bench_with_input(BenchmarkId::from_parameter(name), &bytes, |b, bytes| {
                let bytes = bytes.as_slice();
                b.iter_with_large_drop(|| {
                    let alert: oasiscap::protobuf::Alert = prost::Message::decode(bytes).unwrap();
                    alert
                })
            });
        }
        group.finish();
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
