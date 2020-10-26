fn main() -> i32 {
    let a:i32 = 5;
    let b:&i32 = &a;
    a(b)

}; 
fn a(c:&i32) -> i32 {
    *c
};