fn main() {
    // this code implements a little 
    // Fibonacci sequence in a range of 15

    let sequence = fibonacci();

    println!("The first 17 FIBONACCI numbers in the sequence are {:?}", sequence);
}

fn fibonacci() -> Vec<u32> {
    let mut sequence = vec![0, 1];

    for i in 0..15 {
        let result = sequence[i] + sequence[i + 1];

        sequence.push(result);
    }

    sequence
}
