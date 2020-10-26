fn main() -> () {
    let a:i32 = 5;
    let b:&i32 = &a;
    let c:&i32 = a(b);
};

fn a(a:&i32) -> &i32 {
    a
};