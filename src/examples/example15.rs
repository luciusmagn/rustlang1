pub fn example15()
{
	let hulk = Hero::Strong(100);
	let quicksilver = Hero::Fast;
	let spiderman = Hero::Info
	{
		name: "Spiderman".to_string(),		//can also use .to_owned()
		secret: "Peter Parker".to_string()  //performance is virtually identical
	};

	get_info(hulk);
	get_info(quicksilver);
	get_info(spiderman);
}

enum Hero
{
	Fast,
	Strong(i32),
	Info{name: String, secret: String}
}

fn get_info(h: Hero)
{
	match h
	{
		Hero::Fast => xprintln!("Fast"),
		Hero::Strong(i) => xprintln!("Lifts {} tons", i),
		Hero::Info {name, secret} => xprintln!("{} is {}", name, secret),
	}
}
