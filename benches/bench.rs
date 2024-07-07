use criterion::{criterion_group, criterion_main, Criterion};
use glob_lab::glob_match;

const GLOB: &'static str = "some/**/crazy/needle.txt";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";

fn mine_crate(b: &mut Criterion) {
  b.bench_function("mine", |b| b.iter(|| glob_match(GLOB, PATH)));
}

fn glob_match_crate(b: &mut Criterion) {
  b.bench_function("glob_match_crate", |b| {
    b.iter(|| fast_glob::glob_match(GLOB, PATH))
  });
}

criterion_group!(benches, glob_match_crate, mine_crate);
criterion_main!(benches);
