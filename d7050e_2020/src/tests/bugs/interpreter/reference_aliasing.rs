fn main() -> i32 {
    let a:i32 = 0;
    let b:&i32 = &a;
    let a:i32 = 9;
    *b
};