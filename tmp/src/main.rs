struct Point {
	x: i32,
	y: i32,
}

struct PointRef<'a> {
	x: &'a mut i32,
	y: &'a mut i32,
}

struct Point3d {
	x: i32,
	y: i32,
	z: i32,
}

fn main() {
	let origin = Point3d { x: 0, y: 0, z: 0 };
	let point = Point3d { z: 1, x: 2, .. origin };
	println!("({}, {}, {})", point.x, point.y, point.z);
}

