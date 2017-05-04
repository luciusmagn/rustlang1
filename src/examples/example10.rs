pub fn example10()
{
	let rand_tuple = ("Sergei", 40);
	let rand_tuple_2: (&str, i8) = ("Sergei Stokurev", 41);

	xprintln!("Name 1: {}", rand_tuple.0);
	xprintln!("Age  1: {}", rand_tuple.1);
	xprintln!("Name 2: {}", rand_tuple_2.0);
	xprintln!("Age  2: {}", rand_tuple_2.1);
}
