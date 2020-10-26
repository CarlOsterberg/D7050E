fn main() -> () {
    let a:i32 = 5;
    let b:& & &i32 = & & & a;
    let c:& &i32 = *b;
    let d:&i32 = *c;
    let e:&i32 = &a;
};