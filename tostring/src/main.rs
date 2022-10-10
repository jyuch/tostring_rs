use tostring_macro::ToString;

#[derive(ToString)]
pub struct Struct {
    i: i32,
}

#[derive(ToString)]
pub struct Hoge(i32);

fn main() {
    let a = Struct { i: 1 };
    println!("{:?}", a);

    let b = Hoge(2);
    println!("{:?}", b)
}

// Print macro expanded result
// cargo rustc -- -Z unstable-options -Z unpretty=expanded -Z macro-backtrace
