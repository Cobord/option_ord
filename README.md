proc macro so that

struct MyOption<T>(Option<T>) where T : Ord;

becomes transparent, and gets the implementation of Ord
where the None is bigger/smaller than all the Some(_)

and without generics as well
as in
#[transparent_option_ord(true)]
struct MyOption(Option<i32>);
which puts None as smaller than everything else