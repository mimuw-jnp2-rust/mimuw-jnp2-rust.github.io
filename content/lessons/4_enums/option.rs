#![allow(unused_assignments)]
#![allow(unused_variables)]

fn main() {
    let mut not_null: i32 = 42;
    not_null = 43;
    // not_null = None; // this won't compile because it's a different type!

    let mut nullable: Option<i32> = Some(42);
    nullable = None;
    nullable = Some(43);

    // such construction is rare, but it's possible
    let mut double_nullable: Option<Option<i32>> = Some(Some(42));
    double_nullable = None;
    double_nullable = Some(None);

    // None and Some(None) are different!
    assert_ne!(double_nullable, None);
}