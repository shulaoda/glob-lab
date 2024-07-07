use criterion::{criterion_group, criterion_main, Criterion};

const GLOB: &'static str = "some/**/{bbb,crazy}/needle.{pngg,txt}";
const PATH: &'static str = "some/a/bigger/path/to/the/crazy/needle.txt";

fn mine(c: &mut Criterion) {
  c.bench_function("mine", |b| {
    b.iter(|| assert!(glob_lab::glob_match_with_brace(GLOB, PATH)))
  });
}

fn globset(c: &mut Criterion) {
  c.bench_function("globset", |b| {
    b.iter(|| {
      let pat = globset::Glob::new(GLOB).unwrap().compile_matcher();
      assert!(pat.is_match(PATH));
    })
  });
}

fn glob_match(c: &mut Criterion) {
  c.bench_function("glob_match", |b| {
    b.iter(|| assert!(fast_glob::glob_match(GLOB, PATH)))
  });
}

criterion_group!(benches, mine, globset, glob_match);
criterion_main!(benches);
