fn first_two(seq: &[u32]) -> &[u32] {
    if seq.len() < 2 {
        &seq[..]
    } else {
        &seq[..2]
    }
}

fn main() {
    let seq = [1, 2, 3, 4];
    
    println!("First two elements of the sequence: {:?}", first_two(&seq[..]));
}