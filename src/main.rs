extern crate ansi_term;

#[macro_use]
mod macros;

mod example1;
mod example2;
mod example3;
mod example4;
mod example5;
//mod example6;

use ansi_term::Colour::*;

fn main()
{
	println!("{}", Yellow.paint("Example 1 output:"));
    example1::example1();
	println!("{}", Yellow.paint("\nExample 2 output:"));
    example2::example2();
	println!("{}", Yellow.paint("\nExample 3 output:"));
	example3::example3();
	println!("{}", Yellow.paint("\nExample 4 output:"));
	example4::example4();
	println!("{}", Yellow.paint("\nExample 5 output:"));
	example5::example5();
}
