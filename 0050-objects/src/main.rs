#![allow(unused_assignments)]

struct Vector2D {
    x: f32,
    y: f32,
}

fn main() {
    // Ownership
    let vector = Vector2D { x: 1.0, y: 2.0 };
    print_vector(&vector); // Borrowing the vector, no transfer of ownership
    println!("Vector: ({:p} {}, {})", &vector.x, vector.x, vector.y);
    
    let vector /* on the heap */ = Box::new(Vector2D { x: 3.0, y: 4.0 });
    println!("Vector on the heap: ({:p} {}, {})", &vector.x, vector.x, vector.y);
    
    let another_vector = vector; // Transfer of ownership from `vector` to `another_vector`
    println!("Another vector on the heap: ({:p} {}, {})", &another_vector.x, another_vector.x, another_vector.y);

    let new_vector = create_vector(); // Transfer of ownership from the function to `new_vector`
    drop(new_vector);

    // Borrowing
    let mut vector = Vector2D { x: 7.0, y: 8.0 };
    let borrowed_vector = &vector;
    println!("Borrowed vector: ({:p} {}, {})", &borrowed_vector.x, borrowed_vector.x, borrowed_vector.y);
    vector.x += 1.0;

    vector.x *= 2.0;
    let mut_borrowed_vector = &mut vector;
    mut_borrowed_vector.x *= 2.0;

    enlarge_vector(2.0, &mut vector); // No transfer or ownership!

}

fn print_vector(vector: &Vector2D) {
    println!("Vector: ({:p} {}, {})", &vector.x, vector.x, vector.y);
}

fn create_vector() -> Vector2D {
    Vector2D { x: 5.0, y: 6.0 }
}

fn enlarge_vector(scale: f32, vector: &mut Vector2D) {
    vector.x *= scale;
    vector.y *= scale;
}