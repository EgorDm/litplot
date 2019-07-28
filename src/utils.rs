use rand::distributions::Alphanumeric;
use rand::Rng;

pub fn random_string(l: usize) -> String {
	let mut rng = rand::thread_rng();
	(0..l).map(|_| rng.sample(Alphanumeric)).collect::<String>()
}