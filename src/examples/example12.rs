pub fn example12()
{
	let sum_nums = |x: i32, y: i32| x + y;
	xprintln!("7 + 8 = {}", sum_nums(7,8));

	let num_ten = 10;
	let add_ten = |x: i32| x + num_ten;
	xprintln!("5 + 10 = {}", add_ten(5));
}
