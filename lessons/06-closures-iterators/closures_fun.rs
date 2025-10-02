fn main() {
    fn some_function() -> String {
        String::new()
    }

    let v1 = String::from("v1");
    let mut borrowing_immutably_closure = || v1.clone();

    let mut v2 = String::from("v2");
    let mut borrowing_mutably_closure = || {
        v2.push('.');
        v2.clone()
    };

    let v3 = String::from("v3");
    let mut moving_in_nonmutating_closure = move || v3.clone();

    let mut v4 = String::from("v4");
    let mut moving_in_mutating_closure = move || {
        v4.push('.');
        v4.clone()
    };
    let v5 = String::from("v5");
    let moving_in_moving_out_closure = || v5;

    let fn_once_callables: [&dyn FnOnce() -> String; 5] = [
        &some_function,
        &borrowing_immutably_closure,
        &borrowing_mutably_closure,
        &moving_in_nonmutating_closure,
        &moving_in_moving_out_closure,
    ];

    #[allow(unused_variables)]
    for fn_once_callable in fn_once_callables {
        // Cannot move a value of type `dyn FnOnce() -> String`.
        // The size of `dyn FnOnce() -> String` cannot be statically determined.
        // println!("{}", fn_once_callable());

        // So, for FnOnce, we need to be their owners to be able to call them,
        // and we can't have a `dyn` object owned on stack.
        // We will solve this problem soon with smart pointers (e.g., Box).
        // This will give us `std::function` -like experience.
    }

    // Mutable reference to FnMut is required to be able to call it.
    let fn_mut_callables: [&mut dyn FnMut() -> String; 4] = [
        &mut borrowing_immutably_closure,
        &mut borrowing_mutably_closure,
        &mut moving_in_nonmutating_closure,
        &mut moving_in_mutating_closure,
    ];

    for fn_mut_callable in fn_mut_callables {
        println!("{}", fn_mut_callable());
    }

    let fn_callables: &[&dyn Fn() -> String] =
        &[&borrowing_immutably_closure, &moving_in_nonmutating_closure];

    for fn_callable in fn_callables {
        println!("{}", fn_callable());
    }
}
