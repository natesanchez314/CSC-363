fn main() {
    let a: Box<u8> = Box::new(b'A');
    let b: Vec<String> = vec!["Hello".to_string(), "World".to_string()];

    println!("Box<u8>\n    Address: {:p}\n    Contents: {}", a, a);
    println!("Vec<String>\n    Address: {:p}\n    Contents: {:?}", b.as_ptr(), b);

    let c: Vec<[u8; 4]> = vec![
        [b'A', b'B', b'C', b'D'],
        [b'A', b'B', b'C', b'D'],
        [b'A', b'B', b'C', b'D'],
        [b'A', b'B', b'C', b'D']
    ];
    println!("{:p}, {:p}, {:p}", &c, &c[0], &c[0][0]);

}
