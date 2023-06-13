use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

fn chop_1(s: String) -> Vec<char> {
    let mut res = Vec::new();
    for c in s.chars() {
        res.push(c);
    }
    res
}

fn chop_3(s: String) -> Vec<[char; 3]> {
    let mut res = Vec::<[char; 3]>::new();
    let mut c = s.chars();
    for _i in 0..s.len() / 3 {
        let arr: [char; 3] = [
            c.next().unwrap(),
            c.next().unwrap(),
            c.next().unwrap(),
        ];
        res.push(arr);
    }
    res
}

fn reverse_string(s: String) -> String {
    let mut v = chop_1(s);
    let mut res = String::new();
    while v.len() > 0 {
        let c = v.pop().unwrap();
        res.push(c);
    }
    res
}

fn main() -> Result<(), Box<dyn Error>> {
    let s: String = String::from("Hello, world!");
    println!("{:?}", chop_1(s.clone()));
    println!("{:?}", chop_3(s.clone()));
    println!("{}", reverse_string(s));

    let args : Vec<String> = env::args().skip(1).collect();
    let input_filename : &str = &args[0];
    /*let data : String = std::fs::read_to_string(input_filename)?;
    println!("Length: {}", data.len());
    for line in data.lines().rev() {
        println!("Line length: {}", line.len());
        println! (">>>{}<<<", line);
    }
    for line in data.lines() {
        println!("Line length: {}", line.len());
        println! (">>>{}<<<", reverse_string(line.to_string()));
    }*/
    let input : File = File::open (input_filename)?;
    for b in input.bytes () {
        let b : u8 = b?;
        println! (">>>{}<<<", b);
    }
    Ok(())
}
