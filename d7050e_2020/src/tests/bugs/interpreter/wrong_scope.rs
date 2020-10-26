fn main() -> i32 {
    let mut b:i32 = 0;
    let c:&mut i32 = &mut b;
    if true {
        a(c);
    }
    b
};
fn a(b:&mut i32) -> () {
    *b = 7;
};