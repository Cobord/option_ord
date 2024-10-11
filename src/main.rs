fn main() {}

#[allow(clippy::non_canonical_partial_ord_impl)]
mod test {

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
        struct MyOption<T>(Option<T>)
        where
            T: Ord;

        let x = MyOption(Some(1));
        let y = MyOption(None);
        assert!(y.option_cmp(&x) == Ordering::Greater);
        assert!(y > x);
    }
}
