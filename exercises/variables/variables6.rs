// variables6.rs
// Make me compile! Execute the command `rustlings hint variables6` if you want a hint :)

//1- you aren’t allowed to use mut with constants
//2- Constants can be declared in any scope
//3- constants may be set only to a constant expression, not a result
//4- the type of the value must be annotated
const NUMBER: u32 = 3;
fn main() {
    println!("Number {}", NUMBER);
}
