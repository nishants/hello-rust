use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main(){

	loop{
		println!("Enter your guess");

		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input).expect("Failed to read value from console");
		println!("Your entered {}", user_input);

		let user_input : u32 = user_input.trim().parse().expect("Please enter a valid number");

		let secret = rand::thread_rng().gen_range(1, 11);

		match user_input.cmp(&secret){
			Ordering::Less => println!("Too small than ({})", secret),
			Ordering::Greater => println!("Too large than {}", secret),
			Ordering::Equal => {
				println!("Bingo !");
				break;
			},
		};
	}

}