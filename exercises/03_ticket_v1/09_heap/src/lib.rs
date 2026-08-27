pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

// TODO: based on what you learned in this section, replace `todo!()` with
//  the correct **stack size** for the respective type.
#[cfg(test)]
mod tests {
    use super::Ticket;
    // size_of brings the byte of that thing , not the bits of it.
    // and size_of doesn't bring the heap contents, but only stack.
    use std::mem::size_of;
    // for strings, we have 2 different memory space .
    // first one (stack) contains the information about that string,
    // and second (heap) contains the chars of that string


    // in stack, string is kept with 3 things
    // pointer(u8) , length(usize) and capacity(usize) which is all adds up to 8 x 3 = 24 bytes
    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    // in heap, strings are kept dynamically
    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 72);
    }
}
