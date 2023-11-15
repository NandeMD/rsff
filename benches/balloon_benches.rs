use criterion::{criterion_group, criterion_main, Criterion};
use rsff::balloon::*;

pub fn create_balloon() -> Balloon {
    Balloon::default()
}

pub fn create_balloon_img() -> BalloonImage {
    let v: Vec<u8> = Vec::with_capacity(1000000);
    BalloonImage { img_type: String::from("jpg"), img_data: v }
}

pub fn balloon_benches(c: &mut Criterion) {
    let mut bln = Balloon::default();
    bln.tl_content.push(String::from("asdfasdf"));
    bln.tl_content.push(String::from("asdfasdf"));
    bln.tl_content.push(String::from("asdfasdf"));
    bln.pr_content.push(String::from("ieieieiieieieieie"));
    bln.pr_content.push(String::from("ieieieiieieieieie"));
    bln.pr_content.push(String::from("ieieieiieieieieie"));
    bln.pr_content.push(String::from("ieieieiieieieieie"));
    bln.comments.push(String::from("tutututututut"));
    bln.comments.push(String::from("tutututututut"));
    bln.comments.push(String::from("tutututututut"));
    bln.comments.push(String::from("tutututututut"));
    bln.comments.push(String::from("tutututututut"));

    let mut g = c.benchmark_group("flate2");
    g.sample_size(100_000);

    g.bench_function("create_balloon", |b| b.iter(|| create_balloon()));
    g.bench_function("create_balloon_img", |b| b.iter(|| create_balloon_img()));
    g.bench_function("b_tl_chars", |b| b.iter(|| bln.tl_chars()));
    g.bench_function("b_pr_chars", |b| b.iter(|| bln.pr_chars()));
    g.bench_function("b_cm_chars", |b| b.iter(|| bln.comments_chars()));
    g.bench_function("b_line_count", |b| b.iter(|| bln.line_count()));
    g.bench_function("b_to_string", |b| b.iter(|| bln.to_string()));
    g.bench_function("b_to_xml", |b| b.iter(|| bln.to_xml()));
}

criterion_group!(benches, balloon_benches);
criterion_main!(benches);