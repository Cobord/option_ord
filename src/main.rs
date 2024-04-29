fn main() {

}

mod test {


    #[test]
    fn simple() {
        use std::cmp::Ordering;
        use my_macro::transparent_option_ord;
        
        #[transparent_option_ord(true)]
        struct MyOption(Option<i32>);

        let x = MyOption(Some(1));
        let y = MyOption(None);
        assert!((&y).option_cmp(&x) == Ordering::Less);
    }

    #[test]
    fn generic() {
        use std::cmp::Ordering;
        use my_macro::transparent_option_ord;
        
        #[transparent_option_ord(false)]
        struct MyOption<T>(Option<T>) where T : Ord;

        let x = MyOption(Some(1));
        let y = MyOption(None);
        assert!((&y).option_cmp(&x) == Ordering::Greater);
    }
}