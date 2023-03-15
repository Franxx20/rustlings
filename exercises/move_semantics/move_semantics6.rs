// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.


fn main() {
    // let data = "Rust is great!".to_string()r
    // let data = String::from("Rust is great!");
    let data = String::new();

    let num = 1;
    
    copiaNumero(num);
    println!("num en main {}",num);
    dirNumero(&num);

    println!("{}", get_char(&data));

    string_uppercase(data);
}

// Should not take ownership
fn get_char(data: &String) -> char {
    // data.chars().last().unwrap_or_default()
    data.chars().last().unwrap_or('F')
}

// Should take ownership
fn string_uppercase( mut data:  String) {
    data = data.to_uppercase();

    println!("{}", data);
}

fn copiaNumero(num: i32){

    println!("{}",num);
}
fn dirNumero(num: &i32)
{
    println!("dir {:p} {}", num , *num);
}
