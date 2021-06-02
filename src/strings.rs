//Primitive str = Immutable fixed-length string somewhere in memory
//String = Growable, heap-allocated data structure - Use when you need to modify or own string data

pub fn run() {
    //Fixed length and immutable
    let fix_hello = "hello";

    //Growable
    let mut hello = String::from("Hello ");

    //Get length
    println!("Length: {}", hello.len());

    //Pushing char only
    hello.push('W');

    //Pushing string
    hello.push_str("orld!");

    //Cannot do
    // fix_hello.push('W') or fix_hello.push_str('World')

    //Capacity in bytes
    println!("Capacity: {}", hello.capacity());

    //Check if empty
    println!("Is empty: {}", hello.is_empty());

    //Contains
    println!("Contains 'World':{}", hello.contains("World"));

    //Replace
    println!("Replace : {}", hello.replace("World", "There"));

    //Loop through string by whitespace
    for word in hello.split_whitespace() {
        println!("{}", word);
    }

    //Create a string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('p');

    //Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);

    println!("{:?}", (hello, fix_hello));
}
