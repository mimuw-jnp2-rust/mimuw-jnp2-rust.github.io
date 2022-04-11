fn get_5() -> u32 {
    5 // we could also write "return 5;"
}

fn print_sum(a: u32, b: u32) {
    println!("a + b = {}", a + b);
}

fn main() {
    let a = 100;
    print_sum(a, get_5());
}
