# The Chonk programming language

This is the main source code repository for Chonk. It is a simple interpreted
programming language based on the book [Crafting Interpreters](https://craftinginterpreters.com/).

<details>

<summary>Table of Contents</summary>

- [About Chonk](#about-chonk)
- [Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [Acknowledgments](#acknowledgments)
- [Contact](#contact)
- [License](#license)

</details>

## About Chonk

Chonk's syntax is based on a mixture of Python and Golang. It is dynamically
typed and uses a tree-walk interpreter, which is pretty slow but simple to implement.
It doesn't support [OOP](https://en.wikipedia.org/wiki/Object-oriented_programming)
and the goal is to keep it that way for simplicity's sake.

## Getting Started

To install the Chonk interpreter, follow these steps.

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/rakin406/chonk.git
   ```

2. Install the package
   ```sh
   cargo install --path .
   ```

## Usage

* You can either run the REPL (with zero arguments)
  ```sh
  chonk
  ```

* Or run a script file
  ```sh
  chonk <path-to-file>
  ```

  Example:
  ```sh
  chonk dummy.ck
  ```

  You can use any file extension you want, I chose ".ck" as "chonk" starts with
  'c' and ends with 'k'.

## Syntax and Semantics

Comments start with a hashtag. They can be placed at the end of a line, and the
rest of the line will be ignored. Only single-line comments are supported in
Chonk.
```py
# This is a comment
a = 5;  # This is also allowed
```

### Expressions

Chonk supports 5 arithmetic operations: addition, subtraction, multiplication,
division and modulo.
```py
3 + 2;   # 5
10 - 3;  # 7
7 * 10;  # 70
70 / 7;  # 10
500 % 3; # 2
```

Expressions can be grouped inside parentheses.
```py
(5 * (2 + 4)); # 30
```

In Chonk, variables cannot be declared like in other programming languages.
Instead, to create a variable, you just assign a value to it.

Assignment is done with an equal sign (=).

```py
a = 5;
```

Augmented assignment is used to replace a variable's value. It can be done by
appending an equal sign to any of the arithmetic operators, like "+=", "-=" and
so on.
```py
a = 10;
a += 5; # 15
```

Chonk provides these unary operators: !, -, +
```py
!false; # true
a = 10;
-a; # -10
+a; # 10
```

Prefix increment(++) and decrement(--) operators are allowed on variables. They
can be suffixes too but are desugared(converted) into prefix expression.
```py
a = 5;
++a;    # 6
a++;    # 7
--a;    # 6
```

Comparison operators are used to compare two values.
```py
1 == 1; # true
1 != 1; # false
```

### Statements

The `echo` statement displays the specified message to the screen. The message
can be a string, or any other object.
```sh
echo "Hello World"; # Output: Hello World
echo 123;           # Output: 123
echo true;          # Output: true
echo null;          # Output: null
```

An "if statement" is written by using the `if` keyword. It is used to specify a
block of code to be executed if a condition is `true`.

`else if` is used to specify a new condition if the first condition is `false`.

Finally, `else` is used to execute a block if all of the previous conditions are
`false`.

```go
a = 5;
b = 10;

if a < b {
    echo "b is greater than a";
} else if a == b {
    echo "a and b are equal";
} else {
    echo "a is greater than b";
}
```

Unfortunately, Chonk only has one type of loop: `while` loops.

With the `while` loop, we can execute a set of statements as long as a condition
is `true`.

```rust
i = 1;
while i < 6 {
    echo i;
    ++i;
}
```

A function is defined using the `func` keyword. You can pass parameters into a
function.

The `return` keyword is used to exit a function and return a value.

A function can be called just like in other programming languages. The number of
arguments has to match the number of parameters of the function.

```go
func add(a, b) {
    return a + b;
}

echo add(3, 2);
```
Currently, there is only one native function named "clock", which displays the
current time in seconds.

The `del` keyword is used to delete variables.
```py
a = 5;
del a;
b = c = 10;
del b, c;
```

## Contributing

Contributions are what make the open source community such an amazing place to
learn, inspire, and create. Any contributions you make are **greatly appreciated**.

If you have a suggestion that would make this better, please fork the repo and
create a pull request. Don't forget to give the project a star! Thanks again!

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## Acknowledgments

* [Crafting Interpreters](https://craftinginterpreters.com/)
* [Choose an Open Source License](https://choosealicense.com)
* [Keep a Changelog](https://keepachangelog.com/en/1.0.0/)
* [Semantic Versioning](https://semver.org/spec/v2.0.0.html)

## Contact

Rakin Rahman - rakinrahman406@gmail.com

## License

Distributed under the [Apache License](https://opensource.org/license/apache-2-0/).
See [LICENSE](LICENSE) for more information.
