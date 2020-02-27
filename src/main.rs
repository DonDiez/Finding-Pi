use rand::Rng;

fn main() {
	let mut total = 0.0;
	let mut inside = 0.0;
	let loops = 100000000;
	for _i in 1..(loops+1){
		if inside_circle(distance_origo(rand_float(),rand_float())){
			inside+=1.0;
		}
		total+=1.0;
	}
	// Relation between the imaginary circle inside a 1x1 wil have the same as points outside and inside the corner circle
    println!("PI (Rounds: {}) = {:?}",loops, 4.0*(inside/total));
}

// Returns a random float between 0-1
fn rand_float() -> (f64){
	let mut rng = rand::thread_rng();
	return rng.gen::<f64>();
}

//Finds the distance from origo to point by using its x and y value
fn distance_origo(x: f64, y: f64) -> f64{
	return  (x.powi(2)+y.powi(2)).sqrt();
}

//Boolean for if its inside circle or not (Radius = 1)
fn inside_circle(x: f64) -> bool{
	return x<1.0;
}

