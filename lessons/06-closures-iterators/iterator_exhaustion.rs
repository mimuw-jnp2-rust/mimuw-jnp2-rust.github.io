use std::collections::HashSet;

fn main() {
    // Various ways to create a String.
    let mut strings = [
        String::new(),
        String::from("a"),
        "b".into(),
        "c".to_owned(),
        "d".to_string(),
        "e".chars().collect(),
    ];

    // `iter()` is a usual method that creates an iterator over immutable references to the collection's items.
    let _all_len_0_or_1 = strings
        .iter()
        .filter(|s| !s.is_empty())
        .all(|s| s.len() == 1);

    // `iter_mut()` is a usual method that creates an iterator over mutable references to the collection's items.
    for s in strings.iter_mut().map_while(|s| match s.as_str() {
        "c" => None,
        _ => Some(s),
    }) {
        *s = s.replace("b", "aba");
    }

    // This code is equivalent to the `for` above.
    // `for` is usually more idiomatic, but `for_each` is sometimes cleaner and sometimes faster.
    strings
        .iter_mut()
        .map_while(|s| match s.as_str() {
            "c" => None,
            _ => Some(s),
        })
        .for_each(|s| *s = s.replace("b", "aba"));

    // `into_iter()` is a method from `IntoIterator` trait that converts a collection to an iterator
    let mut empty_strings_iter = strings.into_iter().map(|mut s| {
        s.clear();
        s
    });

    // This is a set of empty Strings...
    let empty_strings_set = empty_strings_iter.clone().collect::<HashSet<_>>();

    // And this is a Vec of immutable references to empty Strings.
    let empty_string_refs_vec = empty_strings_set.iter().collect::<Vec<_>>();

    // equivalent to `empty_string_refs_vec.into_iter()`
    for s in empty_string_refs_vec {
        println!("{}", s)
    }

    while let Some(s) = empty_strings_iter.next_back() {
        assert!(s.is_empty());
    }
}
