use tostring_macro::ToString;

#[derive(ToString)]
pub struct Struct {}

#[derive(ToString)]
pub struct Hoge {}

fn main() {
    let a = Struct {};
    println!("{:?}", a);

    let b = Hoge {};
    println!("{:?}", b)
}

// Print macro expanded result
// cargo rustc -- -Zunstable-options -Zunpretty=expanded
