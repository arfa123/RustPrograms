## First Hello World Rust program build with cargo
>https://www.rust-lang.org/
>https://doc.rust-lang.org/book/ch00-00-introduction.html
### Creating a Project with Cargo:
Let’s create a new project using Cargo and look at how it differs from our original Hello, world! project. Then, on any operating system, run the following:
`$ cargo new hello_cargo`
`$ cd hello_cargo`
The first command creates a new directory called  _hello_cargo_. We’ve named our project  _hello_cargo_, and Cargo creates its files in a directory of the same name.

### Building and Running a Cargo Project
From your  _hello_cargo_  directory, build your project by entering the following command:
`$ cargo build`
This command creates an executable file in _target/debug/hello_cargo_ (or _target\debug\hello_cargo.exe_on Windows) rather than in your current directory. You can run the executable with this command:
`$ ./target/debug/hello_cargo # or .\target\debug\hello_cargo.exe on Windows Hello, world!`
We just built a project with  `cargo build`  and ran it with  `./target/debug/hello_cargo`, but we can also use  `cargo run`  to compile the code and then run the resulting executable all in one command:
`$ cargo run`
Cargo also provides a command called  `cargo check`. This command quickly checks your code to make sure it compiles but doesn’t produce an executable:
`$ cargo check`
### Building for Release
When your project is finally ready for release, you can use `cargo build --release` to compile it with optimizations. This command will create an executable in _target/release_ instead of _target/debug_.
