fn main() {
    // let s1 = String::from("hello");
    // let s2 = s1;

    // When assigning an unknown size variable to a new variable, it does not make a "Copy". It shifts the pointer and necessary info to the new variable and deems the old one as moved. Thus making s1 not usable anymore.
    // println!("{}, world!", s2);

    ///////////////////////////////////////////////////////////////////////////////////////

    // let s1 = String::from("hello");
    // This line hear actually makes  a copy of the data. So this will impact both the stack and the heap. Section 4.4 in the book of rust.
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    /////////////////////////////////////////////////////////////////////////////////////////

    // let s = String::from("hello");  // s comes into scope

    // Because the string type is a non known length, information related to this variable is stored both on the stack and the heap.
    // The stack maintains the pointer reference, where its located on the heap, as well as its length and capacity. Therefore it is inefficient and can't be copied. So once s is passed to the function it moves out of scope permanently.

    // takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    // But because x is immutable and is a known fixed length, it is stored solely on the stack. Thus, making it a copiable type by default. Because of this, when you pass it to the function makes_copy, it actually copies the value of x and passes it. So you can call x after the function and it will still be considered a valid call.                                
    // let x = 5;                      // x comes into scope

    // makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward



    // Using Slices For better tracking of information based off of a reference without having to keep track of its original status
    // let sliceExample = String::from("Hello World!");

    // The slices are always the starting position and then one more than the ending position for the reference. Which is why the string total length is 12 but world reference is 12
    // let hello = &sliceExample[0...5];
    // let world = &sliceExample[6...12];

    let s = String::from("Hello World!");
    let word = get_first_word(&s);

    println!("the first word is: {}", word);
}



// fn takes_ownership(some_string: String) { // some_string comes into scope
//     println!("{}", some_string);
// } // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

// fn makes_copy(some_integer: i32) { // some_integer comes into scope
//     println!("{}", some_integer);
// } // Here, some_integer goes out of scope. Nothing special happens.


// The return type str represents a string slice.
fn get_first_word(s: &String) -> &str {
    // We are attempting to find the last index of the first word that contains a space.
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}