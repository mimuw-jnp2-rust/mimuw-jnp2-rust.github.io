use std::rc::Rc;

struct LoudInt(i32);

impl Drop for LoudInt {
    fn drop(&mut self) {
        println!("[{}] Farewell!", self.0);
    }
}

fn main() {
    {
        let outer_ref;

        {
            let inner_ref = Rc::new(LoudInt(5));

            // strong_count represents the number of owning references pointing
            // to data
            assert_eq!(Rc::strong_count(&inner_ref), 1);

            outer_ref = Rc::clone(&inner_ref);

            assert_eq!(Rc::strong_count(&inner_ref), Rc::strong_count(&outer_ref));
            assert_eq!(Rc::strong_count(&inner_ref), 2);
        }

        println!("The {} still lives!", outer_ref.0);
        assert_eq!(Rc::strong_count(&outer_ref), 1);
    }
}
