proc macro so that

struct MyOption<T>(Option<T>) where T : Ord;

becomes transparent, and gets the implementation of Ord
where the None is bigger/smaller than all the Some(_)