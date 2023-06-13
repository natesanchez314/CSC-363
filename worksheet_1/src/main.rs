fn main() {
    println!("Hello, world!");

    for i in 1..11 {
        println!("{i}");
    }

    let mut sum = 0;
    for i in 1..11 {
        sum += i;
    }
    println!("{sum}");
}
