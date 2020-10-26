fn i32_return(a:&mut i32, mut b:i32) -> i32 {
    while b<10 {
        b = b + 1;
    };
    *a = 3;
    -b
};
fn bool_return(a:&bool, b:bool) -> bool {
    if !(*a && b || false) == true {
        true
    }; else {
        false
    };
};

fn main() -> () {
    let mut a:i32 = 5;
    let b:bool = false;
    let d:bool =bool_return(&b, true);
    i32_return(&mut a, 1);
    let mut t:i32 = if d {3}; else {5};
};