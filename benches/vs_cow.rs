use criterion::{black_box, criterion_group, criterion_main, Criterion};
use maybe_path::MaybePathBuf;
use std::{borrow::Cow, path::Path};

fn maybe_path_read<'a>(p: &'a MaybePathBuf) -> &'a Path {
    p.as_ref()
}

fn maybe_path_clone<'a>(p: &MaybePathBuf<'a>) -> MaybePathBuf<'a> {
    p.clone()
}

fn maybe_path_newpath() -> MaybePathBuf<'static> {
    MaybePathBuf::new_path("foo/bar/baz")
}

#[allow(clippy::ptr_arg)]
fn path_read<'a>(p: &'a Cow<'a, Path>) -> &'a Path {
    p.as_ref()
}

#[allow(clippy::ptr_arg)]
fn path_clone<'a>(p: &Cow<'a, Path>) -> Cow<'a, Path> {
    p.clone()
}

fn new_path() -> Cow<'static, Path> {
    Cow::Borrowed(std::path::Path::new("foo/bar/baz"))
}

fn maybe_path_read_many<'a>(paths: &'a Vec<MaybePathBuf<'a>>) -> Vec<&'a Path> {
    let mut output = Vec::with_capacity(paths.len());
    for path in paths {
        output.push(path.as_ref());
    }

    output
}

fn cow_path_read_many<'a>(paths: &'a Vec<Cow<'a, Path>>) -> Vec<&'a Path> {
    let mut output = Vec::with_capacity(paths.len());
    for path in paths {
        output.push(path.as_ref());
    }

    output
}

fn bench_maybepath(c: &mut Criterion) {
    let mut group = c.benchmark_group("MaybePathBuf");
    let maybe_path = MaybePathBuf::new_path("foo/bar/baz");

    let paths = vec![maybe_path.clone(); 1000];
    group.bench_function("maybe_path_read_many", |b| {
        b.iter(|| maybe_path_read_many(black_box(&paths)))
    });

    group.bench_function("maybe_path_read", |b| {
        b.iter(|| black_box(maybe_path_read(&maybe_path)))
    });
    group.bench_function("maybe_path_clone", |b| {
        b.iter(|| black_box(maybe_path_clone(&maybe_path)))
    });
    group.bench_function("maybe_path_newpath", |b| {
        b.iter(|| black_box(maybe_path_newpath()))
    });

    group.finish();
}

fn bench_path(c: &mut Criterion) {
    let mut group = c.benchmark_group("Path");
    let path = Cow::Borrowed(std::path::Path::new("foo/bar/baz"));

    let paths = vec![path.clone(); 1000];
    group.bench_function("cow_path_read_many", |b| {
        b.iter(|| cow_path_read_many(black_box(&paths)))
    });

    group.bench_function("cow_path_read", |b| b.iter(|| black_box(path_read(&path))));
    group.bench_function("cow_path_clone", |b| {
        b.iter(|| black_box(path_clone(&path)))
    });
    group.bench_function("new_cow_path", |b| b.iter(|| black_box(new_path())));

    group.finish();
}

criterion_group!(benches, bench_path, bench_maybepath);
criterion_main!(benches);
