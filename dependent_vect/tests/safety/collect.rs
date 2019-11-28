use dependent_vect::{vect, collect};

fn test_1() {
    let _ = collect(vec![]);
}

fn test_2() {
    let (_, mut v) = vect!(vec![]);
    v = collect(vec![1]).1;
}

fn main() {}
