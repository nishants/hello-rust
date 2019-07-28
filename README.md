[Find a cheatsheet here](./cheatsheet.md) 

### Learning resources

- **The Book** : https://doc.rust-lang.org/book/title-page.html
- **Rust by example** https://doc.rust-lang.org/rust-by-example/index.html
- **Rustlings** (exercises) : https://github.com/rust-lang/rustlings



### Other interesting resources

- **Fullstack with RUST :** https://thefullsnack.com/en/rust-for-the-web.html
- https://github.com/ctjhoa/rust-learning
- http://cosmic.mearie.org/2014/01/periodic-table-of-rust-types/



### Ecosystem

- **Cargo** : build system and package manager (like npm, gradle, rake) for RUST
- **rustup** : CLI tool for managing RUST versions (like nvm, rvm)
- **Rustfmt**  (Rust Format): code formatter (like prettier) for RUST
- **Rust Language Server** : for IDE integration for code completion, inline error messages
- **Crates** : modules (like node_modules) for RUST
- **[perludes](https://doc.rust-lang.org/std/prelude/index.html)**: default libs imported for a RUST program file



### Setup

- Install **rustup**

  ```sh
  curl https://sh.rustup.rs -sSf | sh
  ```

- Addd following to .bashrc file

  ```bash
  export PATH="$HOME/.cargo/bin:$PATH"
  ```

- You might still need to install a linker, which usually comes along with c compiler for the platform.

- Upading **rustup**

  ```bash
  rustup update	
  ```

- Verify installation of **rustc** (rust compiler)

  ```bash
  rustc --version	
  ```

- List rust docs

  ```bash
  rustup doc			
  ```

  

# 01 - Hello World !

1. Create a `hello_world.rs`

   ```rust
   fn main(){
     println!("Hello Rust ! You are my love at first sight.")
   }
   ```

2. Complile program

   ```bash
   rustc hello_world.rs
   ```

3. Run compiled binary

   ```
   ./hello_world
   ```

**Take note**: 

- **fn main()** returns nothing, and is the default entrypoiont (like c or like public static void main in Java)
- **println!** is a ***macro*** (hence ends with !). If you want to use function instead, use **println**
- it is a **compiled language**





# 02 - Hello Cargo

- Cargo is build tool and dependecy manger for RUST lang.

- Create a new project with cargo

  ```java
  cargo new hello_cargo
  ```

- This creates the directory `hello_cargo` and two files under it

  **Cargo.toml**

  ```toml
  [package]
  name = "hello_cargo"
  version = "0.1.0"
  authors = ["nishant <nishant.singh87@gmail.com>"]
  edition = "2018"
  
  # See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html
  
  [dependencies]
  ```

	**src/main.rs**
	
	```rust
	fn main() {
	    println!("Hello, world!");
	}
	```

- Build project

  ```
  cargo build
  ```

- Run built project

  ```bash
  cargo run
  ```

- Notice it creates a new file `Cargo.lock` which works just like package.lock.json 

- It also creates a **target** directory

- Binary code is generated in **target/debug**

- Check compile errors without building code using `cargo check`

  ```
  cargo check
  ```

- **Build for release**

  ```
  cargo build --release
  ```

- this creates a **target/release** directory to output a more optimized production code

**Notes**

- Cargo uses **Cargo.toml**, just like npm uses package.json
- [TOML](https://github.com/toml-lang/toml) is superb markup language (minimal that JSON, without sucking up like YAML)
- Cargo asks to keep all code in `src`
- Cargo follows philosophy of *place for everything, everything in its place*
- **cargo check** runs much faster than cargo build, hence suitable to quickly checking compile errors
- **carto build** creates binary quickly for dev mode, **cargo build —release** performs extra optmizations for produced binary





# 03 - Guessing game



- Add the rand crate in Cargo.toml to generate random numbers

  ```toml
  [dependencies]
  rand = "0.7.0"
  ```

  and run 

  ```bash
  cargo build
  ```

  

- Create a guessing game

  ```rust
  use std::io;
  use rand::Rng;
  use std::cmp::Ordering;
  
  fn main(){
  
  	loop{
  		println!("Enter your guess");
  
  		let mut user_input = String::new();
  		io::stdin().read_line(&mut user_input).expect("Failed to read value from console");
  		
  		// we could do this too, but will crash program when input is invalid number
  		// let user_input : u32 = user_input.trim().parse().expect("Please enter a valid number");
  		
  		let user_input : u32 = match user_input.trim().parse(){
  			Ok(value) => value,
  			Err(_) => {
  				println!("\"{}\" is not a valid number", user_input.trim());
  				continue;
  			}
  		};
  
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
  ```

**Notes**

- random number generation is not a core part for RUST API (too bad)
- we used **rand crate** for random number generation
- we used the **std::io**  for reading standard input stream (console)
- `io::stdin.read_line()` : returns **enum** of type **io::Result**, which can be **Ok** or **Err**
- `io::Result#expect` returns value (`Ok` type) or exits programs (`Err` type), based on what was returned from `read_line`
- `use rand::Rng;` adds the `Rng` trait in scope of program
- `rand::thread_rng()` : creates **random number generator specific to current thread** 
- `std::cmp::Ordering` is another enum, whose values can be `Greater`, `Less` or `Equal`
- `use std::cmp::Ordering;` brings the `Ordering` enum into the scope
- `cmp` can be called on anything that can be compared, part of `std` scope, it returns an enum of type `std::cmp::Ordering`
- `match ` expression works just like `switch`, the `cases` here are called `arms`. 
- `match user_input.cmp(&secret)` works like `switch(user_input.cmp(&secret))` and the arms work like the `case Ordering::Less :`though
- `u32` is an unsigned integer. RUST auto infers this type based on annotation `let user_input: u32` while processing `user_input.trim().parse()`
- 



**Misc notes**

- Rust’s stability guarantees : backward compatibilty guarnteed for future versions