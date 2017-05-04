pub fn example13()
{
	let vect1 = vec![1, 2, 3];
	let vect2 = vect1;
	// this is invalid
	// xprintln!("vect1[0] = {}", vect1[0]);
	xprintln!("vect2[0] = {}", vect2[0]);

	let prim_val = 1;
	let prim_val2 = prim_val;
	xprintln!("prim_val: {}", prim_val);
	xprintln!("prim_val2: {}", prim_val2);

	xprintln!("sum of vect: {}", sum_vects(&vect2));
	xprintln!("vect: {:?}", vect2);
}

fn sum_vects(v1: &Vec<i32>) -> i32
{
	let sum = v1.iter()
		.fold(
			0, |mut sum, &x| {
				sum += x;
				sum
			}
		);
	sum
}