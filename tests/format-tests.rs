//cargo test --features std

#[test]
fn format_f32() {
    use ufmt::{uwriteln};
    
    use ufmt_float::uFmt_f32;
    let pi = 3.14159234;

    let mut s = String::new();
    let pi_write = uFmt_f32::Zero(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3\n");

    let mut s = String::new();
    let pi_write = uFmt_f32::One(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.1\n");

    let mut s = String::new();
    let pi_write = uFmt_f32::Two(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.14\n");

    let mut s = String::new();
    let pi_write = uFmt_f32::Three(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.141\n");

    let mut s = String::new();
    let pi_write = uFmt_f32::Four(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.1415\n");

    let mut s = String::new();
    let pi_write = uFmt_f32::Five(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.14159\n");

    let number = 134539.0312;
    let mut s = String::new();
    let number_write = uFmt_f32::Two(number);
    uwriteln!(&mut s, "{}",number_write).unwrap();
    assert_eq!(s, "134539.03\n");

    let number = 1439.0034345;
    let mut s = String::new();
    let number_write = uFmt_f32::Three(number);
    uwriteln!(&mut s, "{}",number_write).unwrap();
    assert_eq!(s, "1439.003\n");

    let number = 13538.0006256;
    let mut s = String::new();
    let number_write = uFmt_f32::Four(number);
    uwriteln!(&mut s, "{}",number_write).unwrap();
    assert_ne!(s, "13539.0008\n");

}

#[test]
fn format_f64() {
    use ufmt::{uwriteln};
    
    use ufmt_float::uFmt_f64;
    let pi = 3.14159234;

    let mut s = String::new();
    let pi_write = uFmt_f64::Zero(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3\n");

    let mut s = String::new();
    let pi_write = uFmt_f64::One(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.1\n");

    let mut s = String::new();
    let pi_write = uFmt_f64::Two(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.14\n");

    let mut s = String::new();
    let pi_write = uFmt_f64::Three(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.141\n");

    let mut s = String::new();
    let pi_write = uFmt_f64::Four(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.1415\n");

    let mut s = String::new();
    let pi_write = uFmt_f64::Five(pi);
    uwriteln!(&mut s, "{}",pi_write).unwrap();
    assert_eq!(s, "3.14159\n");

    let number = 134539.037897;
    let mut s = String::new();
    let number_write = uFmt_f64::Two(number);
    uwriteln!(&mut s, "{}",number_write).unwrap();
    assert_eq!(s, "134539.03\n");

    let number = 134539.0033897;
    let mut s = String::new();
    let number_write = uFmt_f64::Three(number);
    uwriteln!(&mut s, "{}",number_write).unwrap();
    assert_eq!(s, "134539.003\n");

    let number = 134539.0003789;
    let mut s = String::new();
    let number_write = uFmt_f64::Four(number);
    uwriteln!(&mut s, "{}",number_write).unwrap();
    assert_eq!(s, "134539.0003\n");

}

