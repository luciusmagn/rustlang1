pub fn example3()
{
	xprintln!("5 + 4 = {}", 5 + 4);
	xprintln!("5 - 4 = {}", 5 - 4);
	xprintln!("5 * 4 = {}", 5 * 4);
	xprintln!("5 / 4 = {}", 5 / 4);
	xprintln!("5 % 4 = {}", 5 % 4);

	let neg_4 = -4i32;

	xprintln!("abs(-4) = {}", neg_4.abs());
	xprintln!("4 ^ 6 = {}", 4i32.pow(6));
	xprintln!("sqrt 9 = {}", 9f64.sqrt());
	xprintln!("cbrt 9 = {}", 27f64.cbrt());
	xprintln!("Round 1.45 = {}", 1.45f64.round());
	xprintln!("Floor 1.45 = {}", 1.45f64.floor());
	xprintln!("Ceiling 1.45 = {}", 1.45f64.ceil());
	xprintln!("e ^ 2 = {}", 2f64.exp());
	xprintln!("log(2) = {}", 2f64.ln());
	xprintln!("log10(2) = {}", 2f64.log10());
	xprintln!("90 to Radians = {}", 90f64.to_radians());
	xprintln!("PI to Degrees = {}", 3.14f64.to_degrees());
	xprintln!("Max 4, 5 = {}", 4f64.max(5f64));
	xprintln!("Min 4, 5 = {}", 4f64.min(5f64));
}
