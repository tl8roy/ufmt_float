// cargo test --features std

#[test]
fn format_f32() {
	use ufmt_float::uFmt_f32;
	let pi = 3.14159234;

	let mut s = String::new();
	let pi_write = uFmt_f32::Zero(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3");

	let mut s = String::new();
	let pi_write = uFmt_f32::One(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.1");

	let mut s = String::new();
	let pi_write = uFmt_f32::Two(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.14");

	let mut s = String::new();
	let pi_write = uFmt_f32::Three(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.141");

	let mut s = String::new();
	let pi_write = uFmt_f32::Four(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.1415");

	let mut s = String::new();
	let pi_write = uFmt_f32::Five(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.14159");

	let number = 134539.0312;
	let mut s = String::new();
	let number_write = uFmt_f32::Two(number);
	ufmt::uwrite!(&mut s, "{}", number_write).unwrap();
	assert_eq!(s, "134539.03");

	let number = 1439.0034345;
	let mut s = String::new();
	let number_write = uFmt_f32::Three(number);
	ufmt::uwrite!(&mut s, "{}", number_write).unwrap();
	assert_eq!(s, "1439.003");

	let number = 13538.0006256;
	let mut s = String::new();
	let number_write = uFmt_f32::Four(number);
	ufmt::uwrite!(&mut s, "{}", number_write).unwrap();
	assert_ne!(s, "13539.0008");
}

#[test]
fn format_f64() {
	use ufmt_float::uFmt_f64;
	let pi = 3.14159234;

	let mut s = String::new();
	let pi_write = uFmt_f64::Zero(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3");

	let mut s = String::new();
	let pi_write = uFmt_f64::One(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.1");

	let mut s = String::new();
	let pi_write = uFmt_f64::Two(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.14");

	let mut s = String::new();
	let pi_write = uFmt_f64::Three(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.141");

	let mut s = String::new();
	let pi_write = uFmt_f64::Four(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.1415");

	let mut s = String::new();
	let pi_write = uFmt_f64::Five(pi);
	ufmt::uwrite!(&mut s, "{}", pi_write).unwrap();
	assert_eq!(s, "3.14159");

	let number = 134539.037897;
	let mut s = String::new();
	let number_write = uFmt_f64::Two(number);
	ufmt::uwrite!(&mut s, "{}", number_write).unwrap();
	assert_eq!(s, "134539.03");

	let number = 134539.0033897;
	let mut s = String::new();
	let number_write = uFmt_f64::Three(number);
	ufmt::uwrite!(&mut s, "{}", number_write).unwrap();
	assert_eq!(s, "134539.003");

	let number = 134539.0003789;
	let mut s = String::new();
	let number_write = uFmt_f64::Four(number);
	ufmt::uwrite!(&mut s, "{}", number_write).unwrap();
	assert_eq!(s, "134539.0003");
}

#[test]
fn format_heapless() {
	let pi32 = 3.14159234_f32;
	let pi64 = 3.14159234_f64;

	let mut s = heapless::String::<4>::new();
	let number_write = ufmt_float::uFmt_f32::Two(pi32);
	ufmt::uwrite!(&mut s, "{}", number_write).unwrap();
	assert_eq!(s, "3.14");

	let mut s = heapless::String::<4>::new();
	let number_write = ufmt_float::uFmt_f64::Two(pi64);
	ufmt::uwrite!(&mut s, "{}", number_write).unwrap();
	assert_eq!(s, "3.14");
}
