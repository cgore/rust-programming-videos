fn main() {
    let x = 123;
    println!("x is {}", x);
    {
        let x = 456;
        println!("x is {}", x);
    }
    println!("x is {}", x);

    let mut y = 123;
    println!("y is {}", y);
    {
        y = 456;
        println!("y is {}", y);
    }
    println!("y is {}", y);

    // DOESN'T WORK: must be `mut` mutable to re-assign.
    //
    // let z = 123;
    // println!("z is {}", z);
    // {
    //     z = 456;
    //     println!("z is {}", z);
    // }
    // println!("z is {}", z);

    const C:i32 = 123;
    println!("C is {}", C);
    {
        // Shadowing constants actually works if it's another scope,
        // I was surprised by this.
        // It doesn't work if it's in the same scope though,
        // which will work with normal variables.
        const C:i32 = 456;
        println!("C is {}", C);
    }
    println!("C is {}", C);
}
