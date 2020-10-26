fn main() -> i32 {
    let mut a:i32 = 5;
    let b:&mut i32 = &mut a;
    a(b);
    a
}; 
fn a(c:&mut i32) -> () {
    if true {
        *c=1;
    };
};