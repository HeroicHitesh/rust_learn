# rust_learn

## Installation

Install `rustup` using

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

update using

```shell
rustup update
```

uninstall using

```shell
rustup self uninstall
```

## [Hello World](./hello_world)

Compile Rust programs using

```shell
rustc main.rs
```

execute using

```shell
./main
```

## [Cargo](./hello_cargo)

A Rust package manager. Create new Rust projects using

```shell
cargo new <project_name>
```

Example

```shell
cargo new hello_cargo
```

Check if your code compiles successfully using

```shell
cargo check
```

Compile you code using

```shell
cargo build
```

_Execute compiled code by running the binary file of project present in `target/debug`_

By default, builds are created in `dev` mode which are **_unoptimized + debuginfo_**, use `release` mode to get **_optimized_** builds

```shell
cargo build --release
```

_Execute compiled code by running the binary file of project present in `target/release`_

Compile code and run binary file in one go using

```shell
cargo run
```

## [Guessing Game](./guessing_game)

A classic number guessing game

## [Customized Guessing Game](./customized_guessing_game)

Customized lowercase alphabet guessing game
