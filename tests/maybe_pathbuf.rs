use maybe_path::MaybePathBuf;

#[test]
fn test_create() {
    let path1 = MaybePathBuf::new_path("foo/bar/baz");
    const PATH2: MaybePathBuf<'_> = MaybePathBuf::new_str("foo/bar/baz");

    assert!(path1.is_borrowed());
    assert!(PATH2.is_borrowed());
    assert_eq!(&path1, &PATH2);
}

#[test]
fn test_as() {
    let path1 = MaybePathBuf::new_path("foo/bar/baz");
    let path2 = MaybePathBuf::new_str("foo/bar/baz");

    assert_eq!(path1.as_ref(), path2.as_ref());
    assert_eq!(path1.to_owned(), path2.to_owned());
}
