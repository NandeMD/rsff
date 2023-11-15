use criterion::{criterion_group, criterion_main, Criterion, black_box};
use rsff::*;
use rsff::balloon::Balloon;

pub fn create_document() -> Document {
    Document::default()
}

pub fn document_benches(c: &mut Criterion) {
    let mut doc = Document::default();

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

    for _ in 0..100 {
        doc.balloons.push(bln.clone());
    }

    let mut g = c.benchmark_group("flate2");

    let xml_case = String::from(r#"<Document><Metadata><Script>Scanlation Script File v0.2.0</Script><App></App><Info>Num</Info><TLLength>9</TLLength><PRLength>6</PRLength><CMLength>0</CMLength><BalloonCount>2</BalloonCount><LineCount>2</LineCount></Metadata><Balloons><Balloon type="OT"><TL>num</TL><TL>nam</TL><PR>numnam</PR></Balloon><Balloon type="Dialogue"><TL>num</TL></Balloon></Balloons></Document>"#);

    g.sample_size(10_000);

    g.bench_function("create_doc", |b| b.iter(|| create_document()));
    g.bench_function("tl_chars", |b| b.iter(|| doc.tl_chars()));
    g.bench_function("pr_chars", |b| b.iter(|| doc.pr_chars()));
    g.bench_function("cm_chars", |b| b.iter(|| doc.comment_chars()));
    g.bench_function("line_count", |b| b.iter(|| doc.line_count()));
    g.bench_function("len", |b| b.iter(|| doc.len()));
    g.bench_function("to_string", |b| b.iter(|| doc.to_string()));
    g.bench_function("to_xml", |b| b.iter(|| doc.to_xml()));
    g.bench_function("from_xml", |b| b.iter(|| doc.xml_to_doc(black_box(xml_case.clone()))));
}

criterion_group!(benches, document_benches);
criterion_main!(benches);