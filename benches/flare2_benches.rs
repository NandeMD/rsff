use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::fs::File;
use std::io::{Read, Write};
use flate2::write::{DeflateEncoder, GzEncoder, ZlibEncoder};
use flate2::Compression;


pub fn test_gz(xml: &String) {
    let mut enc = GzEncoder::new(Vec::new(), Compression::best());
    enc.write_all(xml.as_bytes()).unwrap();
    enc.finish().unwrap();
}

pub fn test_deflate(xml: &String) {
    let mut enc = DeflateEncoder::new(Vec::new(), Compression::best());
    enc.write_all(xml.as_bytes()).unwrap();
    enc.finish().unwrap();
}

pub fn test_zlib(xml: &String) {
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::best());
    enc.write_all(xml.as_bytes()).unwrap();
    enc.finish().unwrap();
}


pub fn flare2_bench(c: &mut Criterion) {
    let mut xml = String::new();
    let mut f = File::open("test.sffx").unwrap();
    f.read_to_string(&mut xml).unwrap();

    let mut g = c.benchmark_group("flate2");
    g.sample_size(1000).measurement_time(std::time::Duration::from_secs(200));

    g.bench_function("flate2_gzip", |b| b.iter(|| test_gz(black_box(&xml.clone()))));
    g.bench_function("flate2_deflate", |b| b.iter(|| test_deflate(black_box(&xml.clone()))));
    g.bench_function("flate2_zlib", |b| b.iter(|| test_zlib(black_box(&xml.clone()))));
}

criterion_group!(benches, flare2_bench);
criterion_main!(benches);