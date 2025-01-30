#![allow(clippy::approx_constant)]

use fortran_descriptor_parser::descriptor_parser;

#[test]
fn simple_integer() {
    let input = "         1".as_bytes();
    let x = descriptor_parser!("I10")(input).unwrap();
    assert_eq!(x, 1);
}

#[test]
fn simple_float() {
    let input = "  0.3141590E+01".as_bytes();
    let x = descriptor_parser!("F15")(input).unwrap();
    assert_eq!(x, 3.14159);
}

#[test]
fn simple_double() {
    let input = "  0.3141590D+01".as_bytes();
    let x = descriptor_parser!("D15")(input).unwrap();
    assert_eq!(x, 3.14159);
}

#[test]
fn simple_string() {
    let input = "This is a test".as_bytes();
    let x = descriptor_parser!("S14")(input).unwrap();
    assert_eq!(x, "This is a test");
}

#[test]
fn simple_repetitions() {
    let input = "         1         2".as_bytes();
    let (x0, x1) = descriptor_parser!("2I10")(input).unwrap();
    assert_eq!(x0, 1);
    assert_eq!(x1, 2);
}

#[test]
fn different_types() {
    let input = "         1      Test -0.31415E+01".as_bytes();
    let (x0, x1, x2) = descriptor_parser!("I10,S10,F13")(input).unwrap();
    assert_eq!(x0, 1);
    assert_eq!(x1, "Test");
    assert_eq!(x2, -3.1415);
}

#[test]
fn simple_nested() {
    let input = "    1 Test    2 test".as_bytes();
    let (x0, x1, x2, x3) = descriptor_parser!("2(I5,S5)")(input).unwrap();
    assert_eq!(x0, 1);
    assert_eq!(x1, "Test");
    assert_eq!(x2, 2);
    assert_eq!(x3, "test");
}

#[test]
fn multi_nested() {
    let input = "    1 Test    2 test".as_bytes();
    let (x0, x1, x2, x3) = descriptor_parser!("2(1(I5,S5))")(input).unwrap();
    assert_eq!(x0, 1);
    assert_eq!(x1, "Test");
    assert_eq!(x2, 2);
    assert_eq!(x3, "test");
}

#[test]
fn missing_bytes() {
    let input = "   1".as_bytes();
    let x = descriptor_parser!("I5")(input);
    match x {
        Ok(_) => panic!(),
        Err(e) => {
            assert_eq!(e.to_string(), "Found 4 bytes, expected at least 5")
        }
    }
}

#[test]
fn invalid_conversion_i32() {
    let input = "    A".as_bytes();
    let x = descriptor_parser!("I5")(input);
    match x {
        Ok(_) => panic!(),
        Err(e) => {
            assert_eq!(e.to_string(), "Can't convert '    A' into i32")
        }
    }
}

#[test]
fn invalid_conversion_f32() {
    let input = "    A".as_bytes();
    let x = descriptor_parser!("F5")(input);
    match x {
        Ok(_) => panic!(),
        Err(e) => {
            assert_eq!(e.to_string(), "Can't convert '    A' into f32")
        }
    }
}

#[test]
fn invalid_conversion_f64() {
    let input = "    A".as_bytes();
    let x = descriptor_parser!("D5")(input);
    match x {
        Ok(_) => panic!(),
        Err(e) => {
            assert_eq!(e.to_string(), "Can't convert '    A' into f64")
        }
    }
}
