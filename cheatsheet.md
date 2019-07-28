# Cheatsheet

**Ecosystem**: 

|                          |                                                           |
| ------------------------ | --------------------------------------------------------- |
| **Cargo**                | npm equivalent for RUST                                   |
| **rustup**               | cli tool for managing RUST (superset of nvm like feature) |
| **Rustfmt**              | Rust code formatter                                       |
| **Crates**               | node_module equivalent for RUST                           |
| **Rust Language Server** | IDE integration and inline errors                         |
| **perludes**             | equivalent to java.lang (default imports) but configrable |
| **Cargo.toml**           | cargo/RUST equivalent for package.json                    |
| **Cargo.lock**           | cargo/RUST equivalent for package.lock                    |



**Cargo**: 

|                                |                                                            |
| ------------------------------ | ---------------------------------------------------------- |
| **cargo new <project-name>**   | creates new cargo project                                  |
| **cargo build**                | create debug binary (**for dev**) in **target/debug**      |
| **cargo build —release**       | creates **optimized** release binary in **target/release** |
| **cargo check**                | quickly check for compile error without creating binary    |
| **cargo run**                  | compile and run                                            |
| **RUST_BACKTRACE=1 cargo run** | run with stacktrace for errors                             |



**Keywords and symbols**: 

|           |      |      |      |      |        |
| --------- | ---- | ---- | ---- | ---- | ------ |
| **let**   |      |      |      |      | **;**  |
| **mut**   |      |      |      |      | **&**  |
| **fn**    |      |      |      |      | **::** |
| **use**   |      |      |      |      | **.**  |
| **match** |      |      |      |      |        |

**Variables**:

|                     |                                                   |
| ------------------ | ------------------------------------------------- |
| `let name = 5;`     | auto type deduction                               |
| `let name : i32;`   | explicit type declaration, without initialization |
| `let mut name = 5;` | create mutable variable with `mut` keyword      |
|                     |                                                   |
|                     |                                                   |

**Numbers**

|                   |                                    |
| ----------------- | ---------------------------------- |
| `let name : i32;` | create an integer for size 32 bits |
|                   |                                    |
|                   |                                    |



**String**

|                                           |                                                              |
| ----------------------------------------- | ------------------------------------------------------------ |
| `"strings use double quotes"`             | string use double quotes                                     |
| `let my_str = String::new();`             | Create a string                                              |
| `println!("Your name is  {}", user_name)` | String templates in rust with {}                             |
| `str.trim()`                              | strip leading/trailing whitespace (**including line breaks**) |
| `let value : u32 = str.parse()`           | parse string to a u32 number                                 |
|                                           |                                                              |



**Misc language features** : 

|                         |                                               |
| ----------------------- | --------------------------------------------- |
| `use std::io`           | import module                                 |
| `String::new()`         | invoke type function (static function)        |
| `println!("hello";`     | Macro for print line                          |
| `println("hello");`     | Function for print line                       |
| `read_line(&mut myvar)` | pass mutable variable reference to a function |
|                         |                                               |
|                         |                                               |



# Recipes 

- **Loop**

  ```rust
  loop {
  	// do things
  	break; // when done
  }
  ```
  

  
- **Compare values**

  ```rust
  use std::cmp::Ordering;
  fn main(){
  		let value_one = 32;
    	let value_two = 23;
    
      match value_one.cmp(&value_two){
      Ordering::Less => println!("one is less thant two"),
        Ordering::Greater => println!("one is greter than two"),
        Ordering::Equal => println!("one is equal to two")
    }
  }
  ```
  
  
  
- **Random value** 

  ```rust
  use rand:Rng;
  
  fn main(){
    let random_value = rand::thread_rng().gen_range(1,11); 
  }
  ```

- **Read input from console**

  ```rust
  use std::io;
  
  fn main(){
  	let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Caught unknown error");
  }
  ```

  