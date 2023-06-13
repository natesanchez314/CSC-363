fn f (s : String) -> String {
    let mut new_str = s.clone();
    new_str.push_str(" World!");
    new_str
  }
  
  fn g (_s : String) -> &'static str {
    //let mut new_str = s.clone();
    //new_str.push_str(" World!");
    //&new_str
    "Hello World!"
  }

  fn h (r : &str) -> &str {
    // needs a str ref not a string ref
    &r[2..]
  }

  fn f1 (r : &str) -> [&str; 3] {
    [&r[0..4], &r[4..8], &r[8..]]
  }
  
  //fn f2 (r : &str) -> &[&str] {
  //  ...
  //}
  
  fn f3 (r : &str) -> Vec<&str> {
    vec![&r[0..4], &r[4..8], &r[8..]]
  }
  
  fn g1 (r : &str) -> [String; 3] {
    [r[0..4].to_string(), r[4..8].to_string(), r[8..].to_string()]
  }
  
  //fn g2 (r : &str) -> &[String] {
  //  ...
  //}
  
  fn g3 (r : &str) -> Vec<String> {
    vec![r[0..4].to_string(), r[4..8].to_string(), r[8..].to_string()]
  }

  fn join_string_slice () {
    let array : [&str; 4] = ["the", "rain", "in", "Spain"];
    let slice : &[&str] = &array;
    let s : String = slice.join (" ");
    println! ("{s}");
  }
  
  fn join_vector_slice () {
    let array : [&[i32]; 3] = [ &[1,2], &[3,4,5], &[6,7,8,9] ];
    let slice : &[&[i32]] = &array;
    let v : Vec<i32> = slice.join::<&[i32]> (&[-2, -1]);
    println! ("{v:?}");
  }
  
  fn main () {
    let orig = String::from ("hello");
    dbg! (f(orig.clone ()));
    dbg! (g(orig.clone ()));
    dbg! (h("Hello"));
    dbg! (f1 ("the rain in Spain"));
    //dbg! (f2 ("the rain in Spain"));
    dbg! (f3 ("the rain in Spain"));
    dbg! (g1 ("the rain in Spain"));
    //dbg! (g2 ("the rain in Spain"));
    dbg! (g3 ("the rain in Spain"));

    join_string_slice ();
    join_vector_slice ();
  }