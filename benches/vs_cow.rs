use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maybe_path::MaybePathBuf;
use std::{borrow::Cow, path::Path};

fn maybe_path_read(p: &MaybePathBuf) {
    let _ = p.as_ref();
}

fn maybe_path_clone(p: &MaybePathBuf) {
    let _ = p.clone();
}

fn maybe_path_newpath() {
    let _ = MaybePathBuf::new_path("foo/bar/baz");
}

#[allow(clippy::ptr_arg)]
fn path_read(p: &Cow<Path>) {
    let _ = p.as_ref();
}

#[allow(clippy::ptr_arg)]
fn path_clone(p: &Cow<Path>) {
    let _ = p.clone();
}

fn new_path() {
    let _ = Cow::Borrowed(std::path::Path::new("foo/bar/baz"));
}

fn bench_maybepath(c: &mut Criterion) {
    let mut group = c.benchmark_group("MaybePathBuf");
    let maybe_path = MaybePathBuf::new_path("foo/bar/baz");

    group.bench_function("maybe_path_read", |b| {
        b.iter(|| maybe_path_read(black_box(&maybe_path)))
    });
    group.bench_function("maybe_path_clone", |b| {
        b.iter(|| maybe_path_clone(black_box(&maybe_path)))
    });
    group.bench_function("maybe_path_newpath", |b| b.iter(maybe_path_newpath));

    group.finish();
}

fn bench_path(c: &mut Criterion) {
    let path = std::path::Path::new("foo/bar/baz");
    let path = Cow::Borrowed(path);

    let mut group = c.benchmark_group("Path");
    group.bench_function("cow_path_read", |b| b.iter(|| path_read(black_box(&path))));
    group.bench_function("cow_path_clone", |b| {
        b.iter(|| path_clone(black_box(&path)))
    });
    group.bench_function("new_cow_path", |b| b.iter(new_path));

    group.finish();
}

criterion_group!(benches, bench_path, bench_maybepath);
criterion_main!(benches);
