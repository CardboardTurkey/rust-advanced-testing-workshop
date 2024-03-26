#[cfg(test)]
mod tests {
    use googletest::{assert_that, matchers::empty};

    // The default `#[test]` attribute is not enough if you want to use
    // `googletest`'s macros and matchers. You need to use `#[googletest::test]` instead.
    //
    // You'll learn how to write a custom test macro later in the workshop!
    #[googletest::test]
    fn is_empty() {
        let v: Vec<i32> = vec![];
        // The `assert_that!` macro is the equivalent of `assert!` from the standard library.
        // It takes two arguments: the value you want to assert on, and the **matcher** you want to use.
        // You can find all the built-in matchers in the [`matchers`](https://docs.rs/googletest/0.11.0/googletest/index.html#available-matchers)
        // module of the `googletest` crate. Find the right one!
        assert_that!(v, empty());
    }

    #[test]
    fn one_value() {
        let v: Vec<i32> = vec![-1];
        assert_that!(v, empty());
    }

    #[test]
    fn two_values() {
        let v: Vec<i32> = vec![-1, 1];
        assert_that!(v, empty());
    }
}
