use rand::Rng;

static REAL_PI: f64 = 3.141592653589793238462643383279;

fn main() {
	let mut total = 0.0;
	let mut inside = 0.0;
	let loops = 1000000000;
	for _i in 1..(loops+1){
		if inside_circle(distance_origo(rand_float(),rand_float())){
			inside+=1.0;
		}
		total+=1.0;
	}
	let aprox_pi: f64 = 4.0*(inside/total);
	// Relation between the imaginary circle inside a 1x1 wil have the same as points outside and inside the corner circle
    println!("
    	The RealPI =\t{:.15}\n
    	Approx. PI \u{2248}\t{:.15}\n
    	Deviasion  :\t{:.15}\n
    	Equal      :\t{:.14}%\n
    	Rounds     :\t{}",
    	REAL_PI, aprox_pi, (REAL_PI-aprox_pi), 100.0*(aprox_pi.min(REAL_PI)/aprox_pi.max(REAL_PI)), loops);
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

