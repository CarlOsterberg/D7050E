fn a() -> () {
    let mut a:i32 = 5;
    let b:&mut i32 = &mut a;
    *b=7;
};