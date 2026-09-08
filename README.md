# SLL (Singly Linked List) in Rust

A simple, interactive command-line implementation of a singly linked list in Rust. Built as a learning project to explore ownership, `Box`, and `Option` for building linked data structures in safe Rust.

## Features

- **Insert at front** – add a new node at the head of the list
- **Insert at end** – add a new node at the tail of the list
- **Delete from front** – remove the head node
- **Delete from end** – remove the tail node
- **Display** – print the full list

## How it works

The list is built from a generic `Node<T>` struct:

```rust
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}
```

Each node owns the rest of the list through `Option<Box<Node<T>>>`, which is the idiomatic way to represent a nullable, owned link in safe Rust. Operations like `ins_f` and `del_f` use `std::mem::replace` and `Option::take` to shuffle data and ownership without needing `unsafe` code.

## Requirements

- [Rust and Cargo](https://www.rust-lang.org/tools/install) (edition 2024 toolchain)

## Getting started

Clone the repo and run it with Cargo:

```bash
git clone https://github.com/blazermega/sll_rust.git
cd sll_rust
cargo run
```

## Usage

On start, you'll be prompted to enter the first element of the list (an integer). Then you'll see a menu:

```
1. ins first
2. ins last
3. del first
4. del last
5. display
6. exit
```

Enter the number corresponding to the action you want, and follow any additional prompts (e.g. the value to insert).

### Example session

```
enter the first element of the list
5
1.ins first
2.ins last
3.del first
4.del last
5.display
6.exit
2
10
1.ins first
...
5
5 -> 10 ->
6
```

## Notes / limitations

- Currently hardcoded to `i32` values entered via `stdin`.
- Input is unvalidated beyond `parse()`'s own error handling — non-integer input will panic.
- This is a learning/demo project rather than a production-ready data structure crate.

## Future plans

- [ ] Support generic types beyond `i32` (accept strings, floats, etc.)
- [ ] Add input validation instead of panicking on bad input
- [ ] Add insert/delete at an arbitrary position (by index)
- [ ] Add a search/find operation
- [ ] Track list length instead of walking the list each time
- [ ] Add unit tests for each operation
- [ ] Turn `Node`/`List` into a reusable library (`lib.rs`) instead of only a CLI demo
- [ ] Add a `License` file

## License

No license specified yet — add one (e.g. MIT or Apache-2.0) if you plan to share or accept contributions.
