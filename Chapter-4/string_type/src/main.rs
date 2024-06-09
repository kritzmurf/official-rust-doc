fn main() {
    /*Rust has a type "String" that manages data allocated
    * on the heap and is able to store an amount of text that
    * is unknown to us at compile time.
    *
    * We create a String from a string literal using the `from`
    * function
    */
    let s = String::from("hello");
    /*
    * Just like in C/C++, `::` is a namespace operator
    *
    * Note that this String var CAN be mutated
    */   
    let mut s = String::from("hello");
    s.push_str(", world!"); //push_str() appends
    println!("{}", s); //should print `hello, world!`
    /*
    * Important to understand is that this is mutable because it is
    * not allocated on the stack like a string literal is. Instead,
    * this is allocated on the heap. As such:
    *   The memory must be requested form the memory allocator at runtime
    *   We need a way of returning this memory to the allocator when we're done with our `String`
    *
    *   Rather than rely on a garbage collector, or force the programmer to track and free
    *   memory allocations, rust utilzes the concept of `scope` to free allocated memory.
    */
    {
        let s = String::from("hello"); // s is valid from this point forward
        // s remains valid
    }   // scope terminated. s is no longer valid

// Variables and Data Interacting with Move

    /*
    * Multiple variables can interact with the sam3 data in different ways in Rust. 
    * Let's look at an example using an integer
    */   
    let x = 5;
    let y = x;
    
    /*
    * Bind the value of `5` to `x`
    * make a copy of the valu in x and bind it to y
    * We now have two variable3s, x and y. Both equal 5.
    *
    * This is indeed what is happening, because integers are simple values
    * with known, ifxed size, and these two `5` values are pushed onto the STACK.
    *
    * Lets look at the same behavior in a String
    *
    */

    let s1 = String::from("hello");
    let s2 = s1;
    /*
    * This looks very similar, so we might assume that the way it works would be the
    * same: that is, the second line would make a copy of hte value in `s1` and bind it to `s2`.
    * But that isn't<F3> quite waht happens.
    *
    * Unde the hood, String is made up of three parts, shown here:
    *   a pointer to the memory that holds the contents of hte string
    *   a length
    *   a capacity
    *
    *   This group ofdata is stored on the STACK
    *   The actual value of the string, that is, the 'hello', is stored on the 
    *   HEAP
    *
    *   Length is how much memory, in bytes, the contents of the 'String' are cvurrently using.
    *   The capacity is the total amount of memory, in bytes, that the String has received from
    *   the allocator. 
    *
    *   The key difference between length and capacity matters. This will be discussed further on.
    *
    *   When we assign s1 to s12, the Stringt data is copied, meaning we copy the pointer, the
    *   length, and the capacity that are on the stack. We do not copy the data on the heap that
    *   the pointer refers to. In other words, we create a reference to the exact same heap data.
    *
    *   This is NOT how things worked for our int values, which simply allocated another entry on
    *   the STACK
    *
    *   Earlier, we said that when a variable goes out of scope, Rust automatically calls `drop`
    *   and cleans up the heap memory for that variable. But as stated above, both data pointers 
    *   are pointing to the same location. This is a problem: when s2 and s1 go out of scope, they
    *   will both try to free the same memory. This is known as a "double free error", and is one
    *   of the memory safety bugs mentioned previously. Freeing memory twice can lead to memory
    *   corruption, which can potentially lead to security vulnerabilties.
    *
    *   To ensure memory safety, after the line "let s2 = s1;", Rust considres s1 as NO LONGER
    *   VALID.
    *   Therefore, Rust doesn't need to free anything when s1 goes out of scope. 
    */

    /* THIS WILL FAIL
    let s1 = String::from("hello");
    let s2 = s1;
    println!("This will fail: {}", s1);
   */ 

    /*
    * This will fail because Rust prevents you from using invalidated references.
    *
    * If you've heard the terms "shallow copy" and "deep copy" while working in 
    * other langauges, the concept of copying the pointer, length, and capacity
    * without copying the data probably sounds like a shallow copy. But beacuse
    * Rust also invalidates the first variable, instead of being called a shallow
    * copy, its known as a "move". So in the above example, we would say that s1
    * was moved into s2
    *
    * Implied in all of this is the realization that Rust will never automatically
    * create "deep cpies" of data. thererfore, any automatic copying can be assumed to be3
    * inexpensive in terms of runtime performance.
    *
    *
*/

    // VARIABLES AND DATA INTERACTING WITH CLONE

    /*
    * If we DO want to deeply copy the heap data of the "String", not just the stack data,
    * we can use a common method called "clone". We'll discuss method syntax later, but because
    * methods are a common feature in many programming languages, you've probably seen them before.
    *
    * an example of clone
*/
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("S1 = {}, S2 = {}", s1, s2);
    /*
    * This works just fine and explicitly produces the behavior shown above, where the heap data
    * actually DOES get copied.
    *
    * when you see a call to 'clone', you know htat some arbitrary code is being executed and
    * that code may be expnesive. Its a visual indicator that something different is going on.
    *
*/
}
