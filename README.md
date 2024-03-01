## Implementing Custom Error Types in Rust

This repository contains a simple Rust binary example of implementing and using custom error types. It's for my own use in the first instance.

### Getting Started

1. **Clone the repository:**

```bash
git clone https://github.com/durbanlegend/home_grown_errors
cd home_grown_errors
```

2. **Install dependencies:**

```bash
cargo install
```

3. **Run the binary:**

```bash
cargo run
```

### Project Structure

This project is quite simple and consists of a single source file, `src/main.rs`.

### Understanding the Code

The `src/main.rs` file showcases defining a custom error type, `MyError`, implementing the `std::error::Error` and `std::fmt::Display` traits for it, and demonstrating its usage in different scenarios. It also includes error handling with the `Result` type and the `?` operator.

### Learning Resources

The usual learning resources: the Rust Book, the Rustlings course, Rust By Example and web search. This project makes no claim to originality and I must have adapted the code from somewhere! 

This repository is intended as a personal reference and can be further enhanced with additional examples and functionalities related to custom error types in Rust.
