pub fn example14()
{
	let circle1 = Circle {
		x: 10.0,
		y: 10.0,
		radius: 10.0,
	};

	xprintln!(
		"X: {} Y: {} R: {}",
		circle1.x,
		circle1.y,
		circle1.radius
	);
	xprintln!("Circle Radius: {}", get_radius(&circle1));
	xprintln!("Circle X: {}", circle1.get_x());
	xprintln!("Circle Area: {}", circle1.area());

	let rect1 = Rectangle {
		height: 10.0,
		width: 10.0,
	};

	xprintln!("Rectangle Area: {}", rect1.area());
}

struct Circle
{
	x: f64,
	y: f64,
	radius: f64,
}


fn get_radius(circle: &Circle) -> f64
{
	circle.radius
}

impl Circle
{
	pub fn get_x(&self) -> f64
	{
		self.x
	}
}

struct Rectangle
{
	height: f64,
	width: f64,
}

trait HasArea
{
	fn area(&self) -> f64;
}

impl HasArea for Circle
{
	fn area(&self) -> f64
	{
		3.14159 * (self.radius) * (self.radius)
	}
}

impl HasArea for Rectangle
{
	fn area(&self) -> f64
	{
		self.height * self.width
	}
}