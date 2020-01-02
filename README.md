# A writer for f32 and f64 for ufmt

There are six levels of precision based on the number of places after the decimal point.
The accuracy depends a lot on the number of significant figures and the decimal selected.
Use the lowest precision possible for the best results.
 
How to use: 
* Build the ```uFmt_fx``` enum with the variant the level of precision required
* Send that variable to the ufmt macro
* The macro will then output the text representation of the float.
```
use ufmt::{uwriteln};    
use ufmt_float::uFmt_f32;
let pi = 3.14159234;
let mut s = String::new();
let pi_write = uFmt_f32::Zero(pi);
uwriteln!(&mut s, "{}",pi_write).unwrap();
```
Current accuary is about 4-5 significant figures depending on the value.
 
The only relevant feature is ```maths```. If this is set, the library will use the micromaths crate to perform the transformation on the f32 struct. If not selected it will use a method that works in theory but may intoduce errors.
 
Please add an issue or PR if you have a suggestion on how to increase this.
 
There is no rounding on the last digit nor is the seperator selectable at the moment
