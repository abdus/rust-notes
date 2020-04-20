---
title: Guessing Game
date: 2020-04-20 10:51:15.518646319 +0530
tags: [guessing-game]
---

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number!!");

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 200);

        io::stdin()
            .read_line(&mut guess)
            .expect("Faild to Read Line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid Input.");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
```

## Explanation

### The `use` statement

```rs
use rand::Rng;
use std::cmp::Ordering;
use std::io;
```

Rust only brings a few types into global scope, by default. The rest of the types a program needs, should be brought to scope by using the `use` keyword.

Here, `rand` is an external package, `Ordering` is an `enum` for comparing two variables of same type. `Ordering` provides `Ordering:Less`, `Ordering:Greater` and `Ordering:Equal`. `io` provides methods required for input and output.

### Loop

```sh
loop {

}
```

`loop` is keyword for creating infinite loops. In the above program, it allows user to keep-on guessing until they make the correct choice.

### `let`, `mut` and `String::new()`

```sh
let mut guess = String::new();
```

`let` is a keyword for **defining variables** and assign a value to it. By default, Rust variables are immutable. They can't be changed once decleared. The `mut` keyword is used for **mutating a variable** so that it could be modified in a later stage.

The `String::new()` creates an empty value of type `string`. The **`new()` after `::` is known as an associated function** of the type `String`.

> An associated function is implemented on a type, rather than on a particular instance of a Type. Some languages call this a static method.

