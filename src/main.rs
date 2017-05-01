extern crate ansi_term;

#[macro_use]
mod macros;
use macros::*;

mod example1;
mod example2;
mod example3;
mod example4;
mod example5;

use ansi_term::Colour::*;

fn main()
{
	println!("{}", Yellow.paint("Example 1 output:\n"));
    example1::example1();
	println!("{}", Yellow.paint("Example 2 output:\n"));
    example2::example2();
	println!("{}", Yellow.paint("Example 3 output:\n"));
	example3::example3();
}
