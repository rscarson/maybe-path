use std::{borrow::Cow, path::Path};

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maybe_path::MaybePath;

fn maybe_path_read<'a>(p: &'a MaybePath<'a>) -> &'a Path {
    p.as_path()
}

#[allow(clippy::ptr_arg)]
fn path_read<'a>(p: &'a Cow<'a, Path>) -> &'a Path {
    p.as_ref()
}

#[allow(clippy::clone_on_copy)]
fn maybe_path_clone<'a>(p: &MaybePath<'a>) -> MaybePath<'a> {
    p.clone()
}

#[allow(clippy::ptr_arg)]
fn path_clone<'a>(p: &Cow<'a, Path>) -> Cow<'a, Path> {
    p.clone()
}

fn maybe_path_newpath() -> MaybePath<'static> {
    MaybePath::new_path("foo/bar/baz")
}

fn new_path() -> Cow<'static, Path> {
    Cow::Borrowed(std::path::Path::new("foo/bar/baz"))
}

fn bench_maybepath(c: &mut Criterion) {
    let mut group = c.benchmark_group("MaybePath");
    let maybe_path2 = black_box(MaybePath::new_path("foo/bar/baz"));

    group.bench_function("maybe_path_read", |b| {
        b.iter(|| black_box(maybe_path_read(&maybe_path2)))
    });
    group.bench_function("maybe_path_clone", |b| {
        b.iter(|| black_box(maybe_path_clone(&maybe_path2)))
    });
    group.bench_function("maybe_path_newpath", |b| {
        b.iter(|| black_box(maybe_path_newpath()))
    });

    group.finish();
}

fn bench_path(c: &mut Criterion) {
    let path = black_box(Cow::Borrowed(std::path::Path::new("foo/bar/baz")));

    let mut group = c.benchmark_group("Path");
    group.bench_function("path_read", |b| b.iter(|| black_box(path_read(&path))));
    group.bench_function("path_clone", |b| b.iter(|| black_box(path_clone(&path))));
    group.bench_function("new_path", |b| b.iter(|| black_box(new_path())));

    group.finish();
}

criterion_group!(benches, bench_path, bench_maybepath);
criterion_main!(benches);
