fn main() {
    let x = 123;
    let y = 456;
    let z = x + y;
    println!("Hello rusty variables {} + {} = {}", x, y, z);

    let f = 123.456;
    let g = 789.012;
    let h = f + g;
    println!("Hello rusty floats {} + {} = {}", f, g, h);

    let a = f + x as f32;
    println!("Does this work? {}", a);
}
