fn main() {
    let logical: bool = true;

    let a_float: f64 = 1.0;
    let an_interger = 5i32;

    let default_float = 3.0;
    let default_interger = 7;

    let mut inferred_type = 12;
    println!("inferred_type: {}", inferred_type);
    inferred_type = 4249987296i64;
    println!("inferred_type: {}", inferred_type);

    let mut mutable = 12;
    println!("mutable: {}", mutable);

    mutable = 21;
    println!("mutable: {}", mutable);

    // mutable = true;

    // let mutable = true;
}