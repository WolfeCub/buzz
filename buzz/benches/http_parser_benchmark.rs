use criterion::{black_box, criterion_group, criterion_main, Criterion};
use buzz::http_parse::parse_http;

pub fn criterion_benchmark(c: &mut Criterion) {
    let content = concat!(
        "GET /some/route HTTP/1.1\r\n",
        "Content-Length: 0\r\n",
        "Foo: Bar\r\n",
        "SomeHeader: blah\r\n",
    ).as_bytes();

    c.bench_function("parse_http GET", |b| b.iter(|| {
        parse_http(black_box(content))
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
