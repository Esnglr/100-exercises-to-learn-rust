// TODO: Implement the `From` trait for the `WrappingU32` type to make `example` compile.

pub struct WrappingU32 {
    value: u32,
}

// From is an existing trait of core rust which is std::convert::From
impl From<u32> for WrappingU32 {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

fn example() {
    // both are doing the same thing , which is converting the 42 into u32
    // into function is automatically assigned when we implemented the From trait
    let wrapping: WrappingU32 = 42.into();
    let wrapping = WrappingU32::from(42);
}