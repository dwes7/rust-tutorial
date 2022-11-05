fn main() {
    let s1 = String::from("hello"); // copies a string literal into s1, which keeps the memory on the heap and keeps the pointer
                                    // capacity and size to that memory on the stack
    let s2 = s1; // both s1 and s2 have pointers pointing to the same location on the heap. No copies on the heap are made

    // if you do
    // println!("{}, world!", s1); // this will throw an error because rust considers s1 invalid after the s2=s1 assignment

    // let s2 = s1.clone(); will copy the heap contents of s1 so that s2 has it's own memory location of the data on the heap

    // this code is stack-only data because they are immutable and a known size. 
    // x will remain valid after the y=x assignment. DataTypes in Rust with the Copy trait where variables that use the trait
    // do not Move, but are trivially copied. We can't annotate a type with Copy if a Drop trait is already implemented on that type.
    let x = 5;
    let y = x;

    let some_string = String::from("hello"); // some_string is 'moved' into the function and becomes no longer valid
    takes_ownership(some_string);            // because it is a String type and unknown size at compile time
    // if we use some_string past this point, rust will throw a compilor error.

    let some_int = 5; // some_int is copied because it is a known size. 
    makes_copy(some_int);

    let s1 = String::from("Rust!");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

// string comes into scope and then drop is called at the end of the function freeing it from the heap
fn takes_ownership(some_string: String){
    println!("{}", some_string);
} // drop is called

// some_int comes into scope and nothings special happens after the the function returns.
fn makes_copy(some_integer: i32){
    println!("{}", some_integer);
} // northing special is done. some_integer just goes out of scope.

fn gives_ownership() -> String {
    let some_string = String::from("yours");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

// a reference is like a pointer in that it points to a memory address,
// but a reference is guaranteed to point to a VALID value of a partiuclar
// type for the life of that reference

fn calculate_length(s: &String) -> usize {
    s.len()
}