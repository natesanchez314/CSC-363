fn find_num_occurrence_array (n : u8, arr : [u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array "arr".
    let mut occurrences = 0;
    for num in arr {
        if num == n {
            occurrences += 1;
        }
    }
    occurrences
}

fn find_num_occurrence_array_ref (n : u8, arr_ref : &[u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array referenced by "arr_ref".
    let mut occurrences = 0;
    for &num in arr_ref {
        if num == n {
            occurrences += 1;
        }
    }
    occurrences
}

fn find_num_occurrence_slice (n : u8, slice : &[u8]) -> usize {
    // TODO: find and return the number of occurrences of "n" in slice reference "slice".
    let mut occurrences = 0;
    for &num in slice {
        if num == n {
            occurrences += 1;
        }
    }
    occurrences
}

fn increment_array (mut arr : [u8; 10]) -> [u8; 10] {
    // TODO: add one to every element of the array "arr", then return the array.
    for i in 0..arr.len() {
        arr[i] += 1;
    }
    arr
}

fn increment_array_ref (arr_ref : &mut [u8; 10]) {
    // TODO: add one to every element of the array "arr".  Nothing to return.
    for num in arr_ref {
        *num += 1;
    }
}

fn increment_slice (slice : &mut [u8]) {
    // TODO: add one to every element of the array "arr".  Nothing to return.
    for num in slice {
        *num += 1;
    }
}

fn string_to_vector (s : &str) -> Vec<u8> {
    s.bytes ().collect::<Vec<_>> ()
}

fn vector_to_string (v : &[u8]) -> String {
    String::from_utf8_lossy (v).into_owned ()
}

fn main() {
    /*
    let mut i = 0;
    loop {
        println!("{i}");
        i += 1;
    }
    */

    let array = [4,5,6,7,8,9,5,5,6,10];

    // TODO: call find_num_occurrence_array in a loop (with every from 0 to 9 inclusive).
    for i in 0..=9 {
        let occurrences = find_num_occurrence_array(i, array);
        println!("n: {i}, occurrences: {occurrences}");
    }

    // TODO: call find_num_occurrence_array_ref in a loop (with every from 0 to 9 inclusive).
    for i in 0..=9 {
        let occurrences = find_num_occurrence_array_ref(i, &array);
        println!("n: {i}, occurrences: {occurrences}");
    }

    // TODO: call find_num_occurrence_slice in a loop (with every from 0 to 9 inclusive).
    for i in 0..=9 {
        let occurrences = find_num_occurrence_slice(i, &array);
        println!("n: {i}, occurrences: {occurrences}");
    }
    
    let mut mut_array: [u8; 10] = [4,5,6,7,8,9,5,5,6,10];
    dbg! (mut_array);
    dbg! (increment_array (mut_array));
    dbg! (mut_array);
    dbg! (increment_array_ref (&mut mut_array));
    dbg! (mut_array);
    dbg! (increment_slice (&mut mut_array));
    dbg! (mut_array);

    let mut vec_a: Vec<u8> = vec![1, 2, 3];
    let mut len_a = vec_a.len();
    let mut cap_a = vec_a.capacity();
    println!("Length: {len_a}, capacity: {cap_a}, {:?}", vec_a);
    vec_a.push(4);
    len_a = vec_a.len();
    cap_a = vec_a.capacity();
    println!("Length: {len_a}, capacity: {cap_a}, {:?}", vec_a);

    let mut vec_b = Vec::<u8>::new();
    let mut len_b = vec_b.len();
    let mut cap_b = vec_b.capacity();
    println!("Length: {len_b}, capacity: {cap_b}, {:?}", vec_b);
    vec_b.push(1);
    vec_b.push(2);
    len_b = vec_b.len();
    cap_b = vec_b.capacity();
    println!("Length: {len_b}, capacity: {cap_b}, {:?}", vec_b);

    let mut vec_c = Vec::<u8>::with_capacity(3);
    let mut len_c = vec_c.len();
    let mut cap_c = vec_c.capacity();
    println!("Length: {len_c}, capacity: {cap_c}, {:?}", vec_c);
    vec_c.push(1);
    vec_c.push(2);
    vec_c.push(3);
    vec_c.push(4);
    len_c = vec_c.len();
    cap_c = vec_c.capacity();
    println!("Length: {len_c}, capacity: {cap_c}, {:?}", vec_c);

    // TODO: try calling dbg!(string_to_vector(...)) and dbg!(vector_to_string(...))
    dbg!(string_to_vector("Hello world"));
    dbg!(vector_to_string(&vec_a));
}