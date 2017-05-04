use std::io::stdin;

pub fn example7()
{
	'outer: loop
	{
		let number: i32 = 10;
		xprintln!("Pick a number");

		loop
		{
			let mut line = String::new();
			let input = stdin().read_line(&mut line);

			let guess: Option<i32> = input
				.ok()
				.map_or(None, |_| line.trim().parse().ok());

			match guess
			{
				None => xprintln!("Enter a number"),
				Some(n) if n == number =>
				{
					xprintln!("You guessed it");
					break 'outer;
				}
				Some(n) if n < number => xprintln!("Too low"),
				Some(n) if n > number => xprintln!("Too big"),
				Some(_) => xprintln!("Error"),
			}
		}
	}
}