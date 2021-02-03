use std::fs::{read_dir, File};
use std::io::Read;
use std::path::PathBuf;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};

use rust_lexer::parse;
use std::time::Duration;

fn parse_fixtures(c: &mut Criterion) {
    let fixtures: Vec<(PathBuf, u64, String)> = read_dir("fixtures")
        .unwrap()
        .map(|entry| {
            let entry = entry.unwrap();

            let mut file = File::open(entry.path()).unwrap();
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();

            let size = file.metadata().unwrap().len();

            (entry.path(), size, contents)
        })
        .collect();

    let mut group = c.benchmark_group("parse_fixtures");
    group.measurement_time(Duration::from_secs(20));

    for (path, size, content) in fixtures {
        group.throughput(Throughput::Bytes(size));
        group.bench_with_input(
            BenchmarkId::from_parameter(path.to_str().unwrap()),
            &content,
            |b, content| b.iter(|| parse(content)),
        );
    }
    group.finish()
}

criterion_group!(benches, parse_fixtures);
criterion_main!(benches);
