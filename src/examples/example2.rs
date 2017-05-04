pub fn example2()
{
	let is_it_true: bool = true;
	let let_x: char = 'x';

	xprintln!("It is {0}, that {1} is {0}", is_it_true, let_x);
	xprintln!("{:.2}", 1.234);
	xprintln!("B: {:b} H: {:x} O: {:o}", 10, 10, 10);
	xprintln!("\"{ten:>ws$}\"", ten = 10, ws = 5);
	xprintln!("\"{ten:>0ws$}\"", ten = 10, ws = 5);
}