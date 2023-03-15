// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.


fn main() {
    let mut x = 100;
    println!("x {}",x);
    let y = &mut x;
    *y += 100;
    println!("dir y a x {}",y);
    let z = &mut x;
    *z += 1000;
    println!("dir z a x {}",z);
    assert_eq!(x, 1200);
}
