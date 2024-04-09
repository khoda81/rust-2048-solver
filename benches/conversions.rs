use criterion::{black_box, criterion_group, Criterion};

fn conversions(c: &mut Criterion) {
    let mut group = c.benchmark_group("f32<->i32");
    group.bench_function("float2uint", |b| b.iter(|| black_box(0.0_f32) as u32));
    group.bench_function("float2int", |b| b.iter(|| black_box(0.0_f32) as i32));
    group.bench_function("uint2float", |b| b.iter(|| black_box(0_u32) as f32));
    group.bench_function("int2float", |b| b.iter(|| black_box(0_i32) as f32));
    group.finish();
}

criterion_group!(bench_conversions, conversions);
