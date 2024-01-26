# The Chonk programming language

This is the main source code repository for Chonk. It is a simple interpreted
programming language based on the book [Crafting Interpreters](https://craftinginterpreters.com/).

> [!WARNING]
> **Chonk is still under development.**

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

Chonk is based on a mixture of Bash and Python. It is dynamically typed and uses
a tree-walk interpreter, which is pretty slow but simple to implement. It
doesn't support [OOP](https://en.wikipedia.org/wiki/Object-oriented_programming)
and the goal is to keep it that way for simplicity's sake.

## Getting Started

To try out the experimental interpreter, follow these steps.

### Installation

1. Clone the repo
   ```sh
   git clone https://github.com/rakin406/chonk.git
   ```

2. Build the project
   ```sh
   cargo build --release
   ```

## Usage

* You can either run the REPL (with zero arguments)
  ```sh
  ./target/release/chonk
  ```
  or
  ```sh
  cargo run --release
  ```
* Or run a script file
  ```sh
  ./target/release/chonk <path-to-file>
  ```
  Example:
  ```sh
  ./target/release/chonk dummy.ck
  ```
  You can use any file extension you want, I chose ".ck" as "chonk" starts with
  'c' and ends with 'k'.

Sample code:
```sh
echo 6 + 4          # Output: 10
echo 9 < 20         # Output: true
echo "Hello World"  # Output: Hello World
echo !true          # Output: false
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
