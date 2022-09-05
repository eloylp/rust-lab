use criterion::{black_box, Criterion, criterion_group, criterion_main};

use caesar::caesar::{Caesar, Mode};

fn criterion_benchmark(c: &mut Criterion) {
    let text = "The red fox was trying to hunt the white rabbit in a snow field.
    The white rabbit entered in a deep hole, hoping the fox would lost his track. But the fox
    has a very good sense of hearing.";

    c.bench_function("caesar", |b|
        b.iter(|| Caesar::exec(text, 15, Mode::Encrypt).unwrap()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);