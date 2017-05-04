pub fn example11()
{
	say_hello("potato");
	xprintln!("5 + 4 = {}", get_sum(5, 4));

	let sum = get_sum;

	xprintln!("6 + 4 = {}", sum(6, 4));
}

fn say_hello(name: &str)
{
	xprintln!("Hello {}", name);
}

fn get_sum(num1: i32, num2: i32) -> i32
{
	num1 + num2
}