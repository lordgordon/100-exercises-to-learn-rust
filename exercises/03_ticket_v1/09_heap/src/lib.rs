pub struct Ticket {
    title: String,
    description: String,
    status: String,
}

#[cfg(test)]
mod tests {
    use super::Ticket;
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
        // usize a 64bit arch, so u64 ->    8 bytes
        // len is u64 ->                    8 bytes
        // capacity is u64 ->               8 bytes
        // Total                            24 bytes
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Data layout" section of the Rustonomicon
        // https://doc.rust-lang.org/nomicon/data.html for more information.
        assert_eq!(size_of::<Ticket>(), 72);
        // string * 3 = 24 * 3 = 72 bytes
    }
}
