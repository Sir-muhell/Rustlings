// This powerful wrapper provides the ability to store a positive integer value.
// TODO: Rewrite it using a generic so that it supports wrapping ANY type.
struct Wrapper {
    value: usize,
}

// TODO: Adapt the struct's implementation to be generic over the wrapped value.
impl Wrapper   {
    fn new(value: usize) -> Self {
        Wrapper { value }
    }
}

fn main() {
    // Wrapper::new(42);
    // Wrapper::new("hello".len());
    // println!("It worked");

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn store_u32_in_wrapper() {
        assert_eq!(Wrapper::new(42).value, 42);
    }

    #[test]
    fn store_str_in_wrapper() {
        let wrapped = Wrapper::new("hello".len());
        assert_eq!(wrapped.value, 5);
    }
}
