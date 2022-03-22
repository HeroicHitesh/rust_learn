# rust_learn

## 1.1 Installation

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

## [1.2 Hello World](./1.2%20hello_world/)

Compile Rust programs using

```shell
rustc main.rs
```

execute using

```shell
./main
```

## [1.3 Cargo](./1.3%20hello_cargo/)

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

## [2. Guessing Game](./2.0%20guessing_game/)

A classic number guessing game

## [2.1 Customized Guessing Game](./2.1%20customized_guessing_game/)

Customized lowercase alphabet guessing game

## [3.1 Variables and Mutability](./3.1%20variables/)

- By default, variables are immutable
- Constants are always immutable,
- Shadowing, in simple terms, means we reuse name of variable by using `let` again

## [3.2 Data Types](./3.2%20data_types/)

- Scalar Types
  - Integer Types
    - Signed (Negative numbers stored using 2's complement)
      - i8 / i16 / i32 / i64 / i128 / isize (depends on computer architecture)
    - Unsigned (Always Positive)
      - u8 / u16 / u32 / u64 / u128 / usize (depends on computer architecture)
  - Floating-point Types
    - f32 / f64
  - Boolean Type (`bool`)
    - True / False
  - Character Type (`char`)
    - `'c'` - stored in single quotes
- Compound Types
  - Tuple Type (`tup`)
    - A comma-separated list of values (can be of different types) inside parentheses
  - Array Type
    - Every element in an array should be of same type

## [5.1 Defining and Instantiating Structs](./5.1%20defining_and_instantiating_structs/)

In a struct you’ll name each piece of data so it’s clear what the values mean, rest all is same as tuples
