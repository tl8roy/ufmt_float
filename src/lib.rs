//! There are six levels of precision based on the number of places after the decimal point.
//! The accuracy depends a lot on the number of significant figures and the decimal selected.
//! Use the lowest precision possible for the best results.
//!
//! How to use:
//! # Build the ```uFmt_fx``` enum with the variant the level of precision required
//! # Send that variable to the ufmt macro
//! # The macro will then output the text representation of the float.
//! ```
//! use ufmt::{uwriteln};    
//! use ufmt_float::uFmt_f32;
//! let pi = 3.14159234;
//! let mut s = String::new();
//! let pi_write = uFmt_f32::Zero(pi);
//! uwriteln!(&mut s, "{}",pi_write).unwrap();
//! ```
//! Current accuary is about 4-5 significant figures depending on the value.
//!
//! The only relevant feature is ```maths```. If this is set, the library will use the micromaths crate to perform the transformation on the f32 struct. If not selected it will use a method that works in theory but may intoduce errors.
//!
//! Please add an issue or PR if you have a suggestion on how to increase this.
//!
//! There is no rounding on the last digit nor is the seperator selectable at the moment
//use ufmt;

#![no_std]
use ufmt::{uDisplay, uWrite, uwrite, Formatter};

///Select the variant with the level of precision required
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum uFmt_f32 {
	/// ```3```
	Zero(f32),
	/// ```3.1```
	One(f32),
	/// ```3.14```
	Two(f32),
	/// ```3.141```
	Three(f32),
	/// ```3.1415```
	Four(f32),
	/// ```3.14159```
	Five(f32),
}

impl uDisplay for uFmt_f32 {
	///Output the top digits, decimal point and the requested decimals based on the precision selected
	fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
	where
		W: uWrite + ?Sized,
	{
		use uFmt_f32::*;
		let (number, decimals) = match self {
			Zero(x) => float_to_int_f32(*x, 0),
			One(x) => float_to_int_f32(*x, 1),
			Two(x) => float_to_int_f32(*x, 2),
			Three(x) => float_to_int_f32(*x, 3),
			Four(x) => float_to_int_f32(*x, 4),
			Five(x) => float_to_int_f32(*x, 5),
		};
		uwrite!(f, "{}", number)?;

		match self {
			One(_) => uwrite!(f, ".{}", decimals),
			Two(_) if decimals >= 10 => uwrite!(f, ".{}", decimals),
			Two(_) if decimals < 10 => uwrite!(f, ".0{}", decimals),
			Three(_) if decimals >= 100 => uwrite!(f, ".{}", decimals),
			Three(_) if (10..100).contains(&decimals) => uwrite!(f, ".0{}", decimals),
			Three(_) if decimals < 10 => uwrite!(f, ".00{}", decimals),
			Four(_) if decimals >= 1000 => uwrite!(f, ".{}", decimals),
			Four(_) if (100..1000).contains(&decimals) => uwrite!(f, ".0{}", decimals),
			Four(_) if (10..100).contains(&decimals) => uwrite!(f, ".00{}", decimals),
			Four(_) if decimals < 10 => uwrite!(f, ".000{}", decimals),
			Five(_) if decimals >= 10000 => uwrite!(f, ".{}", decimals),
			Five(_) if (1000..10_000).contains(&decimals) => uwrite!(f, ".0{}", decimals),
			Five(_) if (100..1000).contains(&decimals) => uwrite!(f, ".00{}", decimals),
			Five(_) if (10..100).contains(&decimals) => uwrite!(f, ".000{}", decimals),
			Five(_) if decimals < 10 => uwrite!(f, ".0000{}", decimals),
			_ => Ok(()),
		}
	}
}

///Select the variant with the level of precision required
#[allow(non_camel_case_types)]
#[derive(Debug, Clone, Copy)]
pub enum uFmt_f64 {
	/// ```3```
	Zero(f64),
	/// ```3.1```
	One(f64),
	/// ```3.14```
	Two(f64),
	/// ```3.141```
	Three(f64),
	/// ```3.1415```
	Four(f64),
	/// ```3.14159```
	Five(f64),
}

impl uDisplay for uFmt_f64 {
	///Output the top digits, decimal point and the requested decimals based on the precision selected
	fn fmt<W>(&self, f: &mut Formatter<'_, W>) -> Result<(), W::Error>
	where
		W: uWrite + ?Sized,
	{
		use uFmt_f64::*;
		let (number, decimals) = match self {
			Zero(x) => float_to_int_f64(*x, 0),
			One(x) => float_to_int_f64(*x, 1),
			Two(x) => float_to_int_f64(*x, 2),
			Three(x) => float_to_int_f64(*x, 3),
			Four(x) => float_to_int_f64(*x, 4),
			Five(x) => float_to_int_f64(*x, 5),
		};
		uwrite!(f, "{}", number)?;

		match self {
			One(_) => uwrite!(f, ".{}", decimals),
			Two(_) if decimals >= 10 => uwrite!(f, ".{}", decimals),
			Two(_) if decimals < 10 => uwrite!(f, ".0{}", decimals),
			Three(_) if decimals >= 100 => uwrite!(f, ".{}", decimals),
			Three(_) if (10..100).contains(&decimals) => uwrite!(f, ".0{}", decimals),
			Three(_) if decimals < 10 => uwrite!(f, ".00{}", decimals),
			Four(_) if decimals >= 1000 => uwrite!(f, ".{}", decimals),
			Four(_) if (100..1000).contains(&decimals) => uwrite!(f, ".0{}", decimals),
			Four(_) if (10..100).contains(&decimals) => uwrite!(f, ".00{}", decimals),
			Four(_) if decimals < 10 => uwrite!(f, ".000{}", decimals),
			Five(_) if decimals >= 10000 => uwrite!(f, ".{}", decimals),
			Five(_) if (1000..10_000).contains(&decimals) => uwrite!(f, ".0{}", decimals),
			Five(_) if (100..1000).contains(&decimals) => uwrite!(f, ".00{}", decimals),
			Five(_) if (10..100).contains(&decimals) => uwrite!(f, ".000{}", decimals),
			Five(_) if decimals < 10 => uwrite!(f, ".0000{}", decimals),
			_ => Ok(()),
		}
	}
}

/*
use core::ops::{Mul,Sub};

fn float_to_int<T,U,V>(orginal: T,precision: u8) -> (U,V)
	where
		T: From<U> + From<f32> + Copy + Mul + Sub + From<<T as Sub>::Output> + From<<T as Mul>::Output>,
		U: From<T> + Copy,
		V: From<T> + Copy
	{
		let prec = match precision {
			1 => T::from(10.0),
			2 => T::from(100.0),
			3 => T::from(1000.0),
			4 => T::from(10000.0),
			5 => T::from(100000.0),
			_ => T::from(0.0),
		};
		let base = U::from(orginal);
		let decimal = V::from(T::from(T::from(orginal - T::from(base)) * prec));
		(base,decimal)
	}*/

///Split the float into the integer and the fraction with the correct precision
#[cfg(feature = "maths")]
fn float_to_int_f32(orginal: f32, precision: u8) -> (i32, u32) {
	#[allow(unused_imports)]
	use micromath::F32Ext;
	let prec = match precision {
		1 => 10.0,
		2 => 100.0,
		3 => 1000.0,
		4 => 10000.0,
		5 => 100000.0,
		_ => 0.0,
	};
	let base = orginal.trunc() as i32;
	let decimal = (orginal.fract() * prec) as u32;
	(base, decimal)
}

///Split the float into the integer and the fraction with the correct precision
#[cfg(not(feature = "maths"))]
fn float_to_int_f32(orginal: f32, precision: u8) -> (i32, u32) {
	let prec = match precision {
		1 => 10.0,
		2 => 100.0,
		3 => 1000.0,
		4 => 10000.0,
		5 => 100000.0,
		_ => 0.0,
	};
	let base = orginal as i32;
	let decimal = ((orginal - (base as f32)) * prec) as u32;
	(base, decimal)
}

///Split the float into the integer and the fraction with the correct precision
fn float_to_int_f64(orginal: f64, precision: u8) -> (i32, u32) {
	let prec = match precision {
		1 => 10.0,
		2 => 100.0,
		3 => 1000.0,
		4 => 10000.0,
		5 => 100000.0,
		_ => 0.0,
	};
	let base = orginal as i32;
	let decimal = ((orginal - (base as f64)) * prec) as u32;
	(base, decimal)
}
