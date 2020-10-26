fn main() -> i32 {
    sum(9)
};

fn sum(var:i32) -> i32 {
    if var==0 {
        var
    }; else {
        sum(var - 1) + var
    };
};