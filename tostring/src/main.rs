use tostring_macro::ToString;

#[derive(ToString)]
pub struct Struct {}

fn main() {
    let a = Struct {};
    println!("{:?}", a);
}

// Print macro expanded result
// cargo rustc -- -Zunstable-options -Zunpretty=expanded
