use std::rc::Rc;

struct LoudInt(i32);

impl Drop for LoudInt {
    fn drop(&mut self) {
        println!("[{}] Farewell!", self.0);
    }
}

fn main() {
    let weak_ref;

    {
        let shared_ref = Rc::new(LoudInt(5));

        // weak_count keeps track of the non-owning reference to the data
        assert_eq!(Rc::weak_count(&shared_ref), 0);

        // `downgrade()` obtains a weak pointer to Rc's data
        weak_ref = Rc::downgrade(&shared_ref);

        assert_eq!(Rc::weak_count(&shared_ref), 1);
        assert_eq!(Rc::strong_count(&shared_ref), 1);

        // In order to use the the data underneath the weak pointer
        // we need to obtain a new shared pointer from it.
        // The `upgrade()` method returns `Option<Rc<T>>`.
        let temp = weak_ref.upgrade();
        assert_eq!(Rc::strong_count(&shared_ref), 2);
        println!("The value is {}", temp.unwrap().0);
    }

    println!("The value should be deallocated by now.");
    matches!(weak_ref.upgrade(), None);
}
