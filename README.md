### Learning resources

- **The Book** : https://doc.rust-lang.org/book/title-page.html
- **Rust by example** https://doc.rust-lang.org/rust-by-example/index.html
- **Rustlings** (exercises) : https://github.com/rust-lang/rustlings



### Ecosystem

- **Cargo** : build system and package manager (like npm, gradle, rake) for RUST
- **rustup** : CLI tool for managing RUST versions (like nvm, rvm)
- **Rustfmt** : code formatter (like prettier) for RUST
- **Rust Language Server** : for IDE integration for code completion, inline error messages



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

  

### Hello World !

1. Create a `hello_world.rs`

   ```rust
   fn main(){
     println!("Hello Rust ! You are my love at first sight.")
   }
   ```

2. Complile program

   ```bash
   rust c hello_world.rs
   ```

3. Run compiled binary

   ```
   ./hello_world
   ```

**Take note**: 

- **fn main()** returns nothing, and is the default entrypoiont (like c or like public static void main in Java)
- **println!** is a ***macro*** (hence ends with !). If you want to use function instead, use **println**
- it is a **compiled language**











| Ferris                                                       | Meaning                                          |
| ------------------------------------------------------------ | ------------------------------------------------ |
| <img src="https://doc.rust-lang.org/book/img/ferris/does_not_compile.svg" style="width: 100px; display: block; overflow: hidden"> | This code does not compile!                      |
| <img src="https://doc.rust-lang.org/book/img/ferris/panics.svg" style="width: 100px; display: block; overflow: hidden"> | This code panics!                                |
| <img src="https://doc.rust-lang.org/book/img/ferris/unsafe.svg" style="width: 100px; display: block; overflow: hidden"> | This code block contains unsafe code.            |
| <img src="https://doc.rust-lang.org/book/img/ferris/not_desired_behavior.svg" style="width: 100px; display: block; overflow: hidden"> | This code does not produce the desired behavior. |





**Misc notes**

- Rust’s stability guarantees : backward compatibilty guarnteed for future versions