pub fn example4()
{
	let age_old = 6;

	if age_old == 5
	{
		xprintln!("go to kindergarten");
	}
	else if age_old > 5 && age_old < 18
	{
		xprintln!("go to grade {}", age_old - 5);
	}
	else if age_old <= 25 && age_old > 18
	{
		xprintln!("go to college");
	}
	else
	{
		xprintln!("do whatever the fuck you want");
	}

	xprintln!("!true = {}", !true);
	xprintln!("true || false = {}", true || false);
	xprintln!("true != fasle = {}", true != false);

	let can_vote = if age_old >= 18 { true } else { false };

	xprintln!("Can vote: {}", can_vote);
}