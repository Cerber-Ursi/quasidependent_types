use qd_vect::vect;

fn main() {
    let mut v1 = vect!{ vec![] };
    let mut v2 = vect!{ vec![] };
    v1 = v2;
    v2 = v1;
}
