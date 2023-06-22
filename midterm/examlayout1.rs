fn main() {
    let a: Vec<String> = vec!["This".to_string(), "is".to_string(), "a".to_string(), "vec".to_string(), "of".to_string(), "String".to_string()];
    println!("{:?}", a);
    let b: &[u8] = "This is an array ref of u8".as_bytes();
    println!("{:?}", b);
    let c: Box<u8> = Box::new(b'C');
    println!("{:?}", c);
    let d: Box<&u8> = Box::new(&b'D');
    println!("{:?}", d);
    let e: Box<[u8]> = Box::new([b'E', b'E', b'E']);
    println!("{:?}", e);
    let f: Box<&[u8]> = Box::new("This is an array ref of u8".as_bytes());
    println!("{:?}", f);
    let g: Box<Box<u8>> = Box::new(Box::new(b'G'));
    println!("{:?}", g);
    let h: Box<[u8; 4]> = Box::new([b'H', b'H', b'H', b'H']);
    println!("{:?}", h);
    let i: [Box<u8>; 4] = [Box::new(b'I'), Box::new(b'I'), Box::new(b'I'), Box::new(b'I')];
    println!("{:?}", i);
    let j: [Option<u8>; 4] = [Some(b'J'), None, Some(b'J'), None];
    println!("{:?}", j);
    let k: Vec<[u8; 4]> = vec![[b'K', b'K', b'K', b'K'], [b'K', b'K', b'K', b'K']];
    println!("{:?}", k);
}
