use std::mem;

struct Data {
    values: Vec<i32>,  // Heap allocation
}

fn move_ownership(data: Data) {
    println!("Inside function, length of values: {}", data.values.len());
} // `data` is dropped here (memory automatically freed)

fn borrow_data(data: &Data) {
    println!("Inside function (borrowed), length of values: {}", data.values.len());
}

fn main() {
    let data = Data { values: vec![1, 2, 3, 4, 5] };
    
    println!("Size of struct: {} bytes", mem::size_of::<Data>());

    // Move Ownership
    move_ownership(data);
    // println!("{}", data.values.len()); // ERROR: Ownership moved, data is no longer valid

    let borrowed_data = Data { values: vec![10, 20, 30] };
    borrow_data(&borrowed_data); // Borrowing allows reuse without transfer of ownership

    println!("Borrowed data is still accessible: {}", borrowed_data.values.len()); // No issue
}
