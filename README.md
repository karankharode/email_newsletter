# Rust Book Notes

## Project Setup

1. Create new project:
   ```
   cargo new zero2prod
   ```

2. Initialize repository:
   ```
   cd zero2prod
   git add .
   git commit -am "Project skeleton"
   ```

3. Publish branch:
   ```
   git remote add origin git@github.com:YourGitHubNickName/zero2prod.git
   git push -u origin main
   ```

## Configuration

### Faster Linking
Add to `.cargo/config.toml`:


Faster Linking:# .cargo/config.toml

## Additional Tools

- `cargo install cargo-expand`
- `cargo +nightly expand`: This command is used to expand macros in your Rust code. It shows the result of macro expansion, which can be helpful for debugging and understanding how macros work. Note that this requires the nightly toolchain.

  Example usage:
  ```
  cargo +nightly expand
  ```
  This will expand all macros in your project's main file.

  To expand macros in a specific file or module:
  ```
  cargo +nightly expand --bin your_binary_name
  ```
  or
  ```
  cargo +nightly expand --lib
  ```

# On MacOS, `brew install michaeleisel/zld/zld`
[target.x86_64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]
[target.aarch64-apple-darwin]
rustflags = ["-C", "link-arg=-fuse-ld=/usr/local/bin/zld"]


## Development Tools

- **Cargo Watch**: `cargo watch -x check -x test -x run`

## CI Pipeline

1. **Tests**: `cargo test`
2. **Coverage**: `cargo tarpaulin --ignore-tests`
3. **Linting**: `cargo clippy -- -D warnings`
4. **Formatting**:
   ```
   rustup component add rustfmt
   cargo fmt
   cargo fmt -- --check
   ```
5. **Security**: `cargo audit`

## Additional Tools

- `cargo install cargo-expand`

## Notes on Creating an Email Newsletter

- Use of tokio:
  - In Rust, the main function cannot be async.
  - We use `tokio::main` to give an illusion that it is async.
  - Under the hood, it takes our main asynchronous body and writes the necessary boilerplate to make it run on top of tokio's runtime

- Use of `tokio::spawn`:
  - Spawns a new asynchronous task and returns a handle to it.
  - This handle can be used to interact with the spawned task, such as waiting for it to complete or sending it signals.
  - The `spawn` function is used to start a new asynchronous task in Rust. It takes a future as an argument and returns a `JoinHandle` which can be used to manage the spawned task.
  - The `JoinHandle` is a type that represents a handle to a task that can be used to wait for the task to complete or get its result.
  - The `tokio::spawn` function is used to start a new asynchronous task in Rust. It takes a future as an argument and returns a `JoinHandle` which can be used to manage the spawned task.
  - The `JoinHandle` is a type that represents a handle to a task that can be used to wait for the task to complete or get its result.
  - The `tokio::spawn` function is used to start a new asynchronous task in Rust. It takes a future as an argument and returns a `JoinHandle` which can be used to manage the spawned task.
  - The `JoinHandle` is a type that represents a handle to a task that can be used to wait for the task to complete or get its result.

-  use of tcplistener
    - used to listen for the port that was assigned to our application by the OS
    - it takes in the address to bind to, and returns a `TcpListener`
    
