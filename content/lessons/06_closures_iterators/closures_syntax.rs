fn main() {
    #[rustfmt::skip]
    {
        // This is formatted so that with rust-analyzer it renders as well-aligned.

        fn  add_one_v1                        (x: u32) -> u32 { x + 1 }  // This is an ordinary function.
        let add_one_v2 = |x: u32| -> u32 { x + 1 }; // Closures use pipes instead of parentheses.
        let add_one_v3 = |x|        { x + 1 }; // Both parameters and return value can have their types inferred.
        let add_one_v4 = |x|          x + 1  ; // If the body is a single expression, braces can be omitted.

        let _res = add_one_v1(0_u32);
        let _res = add_one_v2(0_u32);
        let _res = add_one_v3(0_u32);
        let _res = add_one_v4(0_u32);
        
        // This does not compile, because closures are not generic.
        // Their type is inferred once and stays the same.
        // let _res = add_one_v4(0_i32);
    };
}
