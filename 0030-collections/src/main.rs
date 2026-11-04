#![allow(unused_variables)]

use std::fmt::Debug;

fn main() {
    // Tuples
    let tuple = (1, 2, true, "abc", (1, 2));
    println!("{}", tuple.3);

    let x = 10;
    let y = 20;
    println!("{:p} {:p}", &x, &y);

    let position = (10, 20);
    println!("{:p} {:p} {:p}", &position.0, &position.1, &position);

    let void = ();
    let pair = (1, true);
    let reversed_pair = (pair.1, pair.0);
    let (x, y) = reversed_pair;

    // Arrays
    let mut my_numbers = [1, 2, 3, 4, 5];
    println!("The second element is {}", my_numbers[1]);
    my_numbers[1] = 10;
    println!("The second element is now {}", my_numbers[1]);

    let many_numbers = [0; 10];
    let matrix = [[0; 3]; 3];
    print_each_element_of_array(&many_numbers);
    print_each_element_of_array(&matrix);

    print_with_const_gen(&many_numbers);

    let few_numbers = [1, 2, 3];
    print_with_const_gen(&few_numbers);

    // Vectors
    let mut my_numbers = vec![1, 2, 3, 4, 5];
    my_numbers.push(6);
    println!("{:?}", my_numbers);

}

// Type parameter with trait bound for Debug
fn print_each_element_of_array<T: Debug>(arr: &[T]) {
    for n in 0..arr.len() {
        println!("{:?}", arr[n]);
    }
}

// Const generics for arrays with a minimum length requirement
fn print_with_const_gen<T: Debug, const N: usize>(arr: &[T; N]) {
    // The following line will be executed at compile time
    const { assert!(N >= 3, "Array must have at least 3 elements"); }
    for n in 0..N {
        println!("{:?}", arr[n]);
    }
}
