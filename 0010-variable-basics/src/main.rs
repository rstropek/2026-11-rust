#![allow(unused_variables)]
#![allow(unused_assignments)]

fn add(a: u16, b: u16) -> u16 {
    a + b
}

// i32, i16, i64...
// u32, u16, u64...
// f32, f64...

fn main() {
    let x1: u16 = 42;
    let mut x2 = { 
        let a = 21;
        let b = 21;
        a + b
    };

    x2 += 1;

    let x2 = x2;
    {
        let mut x2 = x2; // Copying the int value
        x2 += 1;
    }

    println!("Hello, world {}!", add(x1, x2));

    //let my_range = 0..10;
    let my_rand_number;
    my_rand_number = rand::random_range(0..10);
    println!("Hello, world {}!", add(x1, my_rand_number));

}
