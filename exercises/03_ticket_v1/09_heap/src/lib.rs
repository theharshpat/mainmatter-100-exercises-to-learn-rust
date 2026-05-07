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
    use std::mem::size_of;

    #[test]
    fn string_size() {
        assert_eq!(size_of::<String>(), 24);
    }

    #[test]
    fn ticket_size() {
        // This is a tricky question!
        // The "intuitive" answer happens to be the correct answer this time,
        // but, in general, the memory layout of structs is a more complex topic.
        // If you're curious, check out the "Type layout" section of The Rust Reference
        // https://doc.rust-lang.org/reference/type-layout.html for more information.
        assert_eq!(size_of::<Ticket>(), 24 * 3);
    }

    #[test]
    fn checking_string_ref_size_and_value() {
        let mut s = String::with_capacity(5);

        println!("=== BEFORE PUSH ===");

        // Address of the String struct itself (stack location)
        println!("address of s (String struct): {:p}", &s);

        // Heap buffer pointer stored INSIDE String
        println!("s.as_ptr() (heap buffer):     {:p}", s.as_ptr());

        println!("len: {}, cap: {}", s.len(), s.capacity());

        s.push_str("Hey");

        println!("\n=== AFTER PUSH ===");

        println!("address of s (String struct): {:p}", &s);
        println!("s.as_ptr() (heap buffer):     {:p}", s.as_ptr());

        println!("len: {}, cap: {}", s.len(), s.capacity());

        let r: &String = &s;

        println!("\n=== REFERENCE ===");

        // r points TO s
        println!("r value (address it stores):  {:p}", r);

        // address of the reference variable itself
        println!("address of r variable:        {:p}", &r);

        // heap pointer through the reference
        println!("r.as_ptr() (heap buffer):     {:p}", r.as_ptr());

        println!("\n=== CHECKS ===");

        // r stores address of s
        assert_eq!(r as *const String, &s as *const String);

        // both see same heap buffer
        assert_eq!(r.as_ptr(), s.as_ptr());
    }
}
