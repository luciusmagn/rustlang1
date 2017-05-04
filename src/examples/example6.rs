pub fn example6()
{
	let rand_string = "I am random string";
	xprintln!("Length: {}", rand_string.len());

	let (first, second) = rand_string.split_at(6);

	xprintln!("First: {} Second: {}", first, second);

	let mut chars = rand_string.chars();
	let mut indiv_char = chars.next();

	loop
	{
		match indiv_char
		{
			Some(x) => xprintln!("{}", x),
			None => break,
		}
		indiv_char = chars.next();
	}

	let mut iter = rand_string.split_whitespace();
	let mut indiv_word = iter.next();

	loop
	{
		match indiv_word
		{
			Some(x) => xprintln!("{}", x),
			None => break,
		}
		indiv_word = iter.next();
	}

	let rand_string2 = "I am a random string\nThere are other string like it\nThis string is the best";
	let mut lines = rand_string2.lines();
	let mut indiv_line = lines.next();

	loop
	{
		match indiv_line
		{
			Some(x) => xprintln!("{}", x),
			None => break,
		}
		indiv_line = lines.next();
	}

	xprintln!("Find best: {}", rand_string2.contains("best"));
}
