// This function is going to be used only in the tests, so we add the `#[cfg(test)]` attribute.
// It means that it won't be compiled in the final executable.
#[cfg(test)]
fn return_42() -> i32 {
    42
}

fn frobnicate(x: i32) -> i32 {
    println!("frobicating...!");
    x + 40
}

fn main() {
    frobnicate(2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(return_42(), 42);
    }

    #[test]
    fn test_frobnicate() {
        assert_eq!(frobnicate(2), 42);
    }
}
