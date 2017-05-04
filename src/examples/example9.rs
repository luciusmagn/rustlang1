pub fn example9()
{
	let mut vect1 = vec![1,2,3,4,5];
	xprintln!("Item 2 {}", vect1[1]);

	for i in &vect1
	{
		xprintln!("Vect: {}", i);
	}

	vect1.push(6);
	vect1.pop();
}
