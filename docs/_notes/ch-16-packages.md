---
date: Tue 09 Jun 2020 07:45:17 PM +0530
title: "Package Management"
tags: ["package"]
---

Package management comes into play when a project gets bigger and bigger.
Dividing codes into multiple files and grouping them make a project more
readable and maintainable.

A package can contain multiple binary crate and optionally one library crate.

Additionally, a package can help encapsulate codes that are not meant for
others to see. It can also enable scoping.

Rust's Module System:

- Packages: Cargo feature that let user build, test and share crates
- Crates: A tree of modules that produces a library or executable
- Modules and `use`: Let user contron the organization, scope and encapsulation.
- Paths: A way of naming an item, such as `struct`, function or module

## Understanding `cargo new project-name`

When user runs `cargo new project-name`, Cargo generates a couple of files
inside `project-name` directory. They are: config.toml and `src/main.rs`.

`config.toml` is the configuration file. This file contains dependancies and
other project related informations.

`src/main.rs` is the entry point for the Rust compiler. One does not have to
mention entry point in config file, as Rust treats `main.rs` as entry point
by convention. Similarly, `src/lib.rs` means the project contains a library
crate with the same name as package. There can be any number of binaries
inside `bin/` directory.

## The `mod`

`mod` token stands for module. It can be used to declare a module. A module
can host multiple modules nested inside it. Other than modules, it can
hold `struct`, functions etc.

An example:

```rs
mod front_of_the_house {
    mod hosting {
        fn add_to_wishlist() {};
    }
}
```

## Paths

Rust find the codes we want to use by path. A path can be relative or absolute.
Absolute path starts with crate name or literal `crate`.

A relative path, on the other hand, starts from the current module and uses
`self`, `super` or an indetifier in current module.

Both absolute and relative paths are followed by one or more indetifier
separated by `::`.

Example:

```rs
pub fn call_them() {
    // absolute path
    crate::front_of_the_house::hosting::add_to_wishlist();

    // relative path
    front_of_the_house::hosting::add_to_wishlist();
}
```

## Exposing Paths using `pub`

By default, a path is always private. In order to make it public, `pub` keyword
must be prepend.

Example:

```rs
mod front_of_the_house {
    mod hosting {
        fn add_to_wishlist() {};
    }
}
```

