#[cfg(test)]
use fortran_descriptor_parser::FromSlice;

#[test]
fn to_float_f() {
    let input = " 3.1416".as_bytes();
    let f: f32 = input.to_type().unwrap();
    assert_eq!(3.1416, f);
}

#[test]
fn to_float_e() {
    let input = "  0.3141590E+01".as_bytes();
    let f: f32 = input.to_type().unwrap();
    assert_eq!(3.141_59, f);
}

#[test]
fn to_float_d() {
    let input = "  0.3141590D+01".as_bytes();
    let f: f32 = input.to_type().unwrap();
    assert_eq!(3.141_59, f);
}

#[test]
fn to_float_i() {
    let input = "  -1".as_bytes();
    let f: f32 = input.to_type().unwrap();
    assert_eq!(-1.0, f);
}

#[test]
fn to_double_f() {
    let input = "  3.1416".as_bytes();
    let f: f64 = input.to_type().unwrap();
    assert_eq!(3.1416, f);
}

#[test]
fn to_double_e() {
    let input = "  0.3141590E+01".as_bytes();
    let f: f64 = input.to_type().unwrap();
    assert_eq!(3.141_59, f);
}

#[test]
fn to_double_d() {
    let input = "  0.3141590D+01".as_bytes();
    let f: f64 = input.to_type().unwrap();
    assert_eq!(3.141_59, f);
}
#[test]
fn to_double_i() {
    let input = "  -1".as_bytes();
    let f: f64 = input.to_type().unwrap();
    assert_eq!(-1.0, f);
}
