use maybe_path::MaybePath;

#[test]
fn test_create() {
    let path1 = MaybePath::new_path("foo/bar/baz");
    const PATH2: MaybePath<'_> = MaybePath::new_str("foo/bar/baz");

    assert!(path1.is_path());
    assert!(!PATH2.is_path());
    assert_eq!(path1.as_ref(), PATH2.as_ref());
}

#[test]
fn test_as() {
    let path1 = MaybePath::new_path("foo/bar/baz");
    let path2 = MaybePath::new_str("foo/bar/baz");

    assert_eq!(path1.as_path(), path2.as_path());
    assert_eq!(path1.as_str(), path2.as_str());
    assert_eq!(path1.to_owned(), path2.to_owned());
}
