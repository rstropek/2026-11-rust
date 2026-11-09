#![allow(dead_code, unused, unused_variables)]

#[derive(Clone)]
struct Customer {
    id: i32,
    name: String,
}

struct Order<'a> {
    customer: &'a Customer,
    product: String,
    quantity: i32,
}

struct Vec { x: f32, y: f32, }
fn get_longer_vec<'a>(v1: &'a Vec, v2: &'a Vec) -> &'a Vec {
    if (v1.x * v1.x + v1.y * v1.y) > (v2.x * v2.x + v2.y * v2.y) {
        v1
    } else {
        v2
    }
}

fn main() {
    let customers = vec![
        Customer {
            id: 1,
            name: String::from("Alice"),
        },
        Customer {
            id: 2,
            name: String::from("Bob"),
        },
    ];

    for i in 0..customers.len() {
        println!("Name of customer {i}: {}", customers[i].name);
    }

    let o1 = Order {
        customer: &customers[0],
        product: String::from("Laptop"),
        quantity: 1,
    };

    println!("Order is for customer: {}", o1.customer.name);
}
