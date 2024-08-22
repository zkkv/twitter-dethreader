# Twitter Dethreader
Twitter dethreader (/de-thread-er/, not /deth-read-er/) is a command-line tool that converts Twitter threads into markdown format. This project was developed mostly for fun, educational value and because it is something I would use myself.

## How to Run
Currently, there is no way to install this program globally as a binary.

To install, you need to have Rust language installed as well as `cargo` package manager. Clone this repository and run:

```shell
cargo run -- [OPTIONS] <tweet-id> 
```

## Contributing
I certainly have some things that are missing or could be improved. Thus, feel free to contribute to this project. Best way to start is to open an issue. 

Contribution is not just about adding features. This project can also benefit from:

- Documentation
- Testing
- GUI
- Discovering bugs

This list is not exhaustive.

## Motivation

This was developed mainly for two reasons:

- When Twitter API stopped being free, all existing tools for unwrapping threads broke. I decided that I could make one myself and it can be free.
- I wanted to practice Rust.

## Developer
Developed by zkkv, 08/2024