pub fn example8()
{
	let rand_array = [1,2,3];

	xprintln!("First element: {}", rand_array[0]);
	xprintln!("Array length: {}", rand_array.len());
	xprintln!("Second 2 : {:?}", &rand_array[1..3]);
}
