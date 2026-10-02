fn main() {
    borrowing_immutably_closure();
    borrowing_mutably_closure();
    moving_in_nonmutating_closure();
    moving_in_mutating_closure();
    moving_in_moving_out_closure();
}

fn borrowing_immutably_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    // This would not really only borrow... (it needs Vec by value).
    // let only_borrows = || std::mem::drop::<Vec<_>>(list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

fn borrowing_mutably_closure() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

fn moving_in_nonmutating_closure() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // This closure would just borrow the list, because it only prints it.
    // However, as spawning threads require passing `impl FnOnce + 'static`,
    // we need to use `move` keyword to force the closure to move `list`
    // into its captured environment.
    std::thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

fn moving_in_mutating_closure() {
    fn append_42(mut appender: impl FnMut(i32)) {
        appender(42);
    }

    let mut appender = {
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // The `move` keyword is necessary to prevent dangling reference to `list`.
        // Of course, the borrow checker protects us from compiling code without `move`.
        move |num| list.push(num)
    };

    append_42(&mut appender);
    append_42(&mut appender);
}

fn moving_in_moving_out_closure() {
    fn append_multiple_times(appender: impl FnOnce(&mut Vec<String>) + Clone) {
        let mut list = Vec::new();

        // We can clone this `FnOnce`, because we additionally require `Clone`.
        // If we didn't clone it, we couldn't call it more than *once*.
        appender.clone()(&mut list);
        appender(&mut list);
    }

    let appender = {
        let string = String::from("Ala");
        println!("Before defining closure: {:?}", string);

        // The `move` keyword is necessary to prevent dangling reference to `list`.
        // Of course, the borrow checker protects us from compiling code without `move`.
        move |list: &mut Vec<String>| list.push(string)
    };

    // As `appender` is only `FnOnce`, we need to clone before we consume it by calling it.
    append_multiple_times(appender.clone());
    append_multiple_times(appender);
}
