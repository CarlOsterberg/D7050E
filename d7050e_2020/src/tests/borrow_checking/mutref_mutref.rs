fn main() -> i32 {
    let mut a:i32 = 5;
    let mut b:&mut i32 = &mut a;
    let c:&mut &mut i32 = &mut b;
    **c
};