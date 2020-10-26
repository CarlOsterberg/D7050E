fn a() -> i32 {
    5 + false
};
fn b() -> i32 {
    5 / false
};
fn c() -> i32 {
    5 * false
};
fn d() -> i32 {
    5 - false
};
fn e() -> i32 {
    -false
};
fn b() -> i32 {
    !5
};
fn f() -> i32 {
    5 || false
};
fn g() -> i32 {
    5 && false
};
fn h() -> i32 {
    5 > false
};
fn i() -> i32 {
    5 < false
};
fn j() -> i32 {
    5 == false
};
fn k() -> () {
    let b:i32 = false;
};
fn l() -> () {
    let b:bool = 5;
};
fn m() -> () {
    let mut b:i32 = 3;
    b = false;
};
fn n() -> () {
    let mut b:bool = true;
    b = 5;
};