fn main() {
    let mut res: Result<i32, i32> = Ok(5);

    // `map` allows us to transform the value inside `Ok()`,
    // while leaving the `Err` untouched
    assert_eq!(res.map(|v| v * v), Ok(25));

    res = Err(5);
    assert_eq!(res.map(|v| v * v), Err(5));

    // With most combinators there are mirrored ones that work on `Err`
    // variants instead of `Ok`s.
    assert_eq!(res.map_err(|v| v * v), Err(25));

    // We can swap an `Ok` value for a different one with `and()`.
    // Analogously for `Err` and `or()`.
    res = Ok(5);
    assert_eq!(res.and(Ok(100)), Ok(100));

    res = Err(5);
    assert_eq!(res.and(Ok(100)), Err(5));

    // `and_then()` and `or_else()` allow us to invoke functions
    // only when the result is either an `Ok` or an `Err` respectively.
    let sq = |x: i32| -> Result<i32, i32> { Ok(x * x) };
    let err = |x: i32| -> Result<i32, i32> { Err(x) };
    
    assert_eq!(Ok(2).and_then(sq).and_then(sq), Ok(16));
    assert_eq!(Ok(2).and_then(sq).and_then(err), Err(4));
    assert_eq!(Ok(2).and_then(err).and_then(sq), Err(2));
    assert_eq!(Err(3).and_then(sq).and_then(sq), Err(3));

    assert_eq!(Ok(2).or_else(sq).or_else(sq), Ok(2));
    assert_eq!(Ok(2).or_else(err).or_else(sq), Ok(2));
    assert_eq!(Err(3).or_else(sq).or_else(err), Ok(9));
    assert_eq!(Err(3).or_else(err).or_else(err), Err(3));
}