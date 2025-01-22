#[test]
fn simple() {
    use option_ord::transparent_option_ord;
    use std::cmp::Ordering;

    #[transparent_option_ord(true)]
    struct MyOption(Option<i32>);

    let x = MyOption(Some(1));
    let y = MyOption(None);
    assert!(y.option_cmp(&x) == Ordering::Less);
    assert!(y < x);
}

#[test]
fn generic() {
    use option_ord::transparent_option_ord;
    use std::cmp::Ordering;

    #[transparent_option_ord(false)]
    struct MyOption<T, U, V>(Option<(T, U, V)>)
    where
        T: Ord,
        U: Ord,
        V: Ord;

    let x = MyOption(Some((1,24,true)));
    let y = MyOption(None);
    assert!(y.option_cmp(&x) == Ordering::Greater);
    assert!(y > x);
}
