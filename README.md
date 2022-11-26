
# dg

A CLI tool that outputs definitions.

## How to use

Run `./dg <your-word>` and the CLI tool will output it's phonetic, part of speech and three definitions.

## Example

```shell
./dg computer
```

Output 
```
Phonetic: "/kəmˈpjuːtə/"
Part of speech: "noun"

Definitions

Definition -> "A person employed to perform computations; one who computes."
Definition -> "(by restriction) A male computer, where the female computer is called a computress."
Definition -> "A programmable electronic device that performs mathematical calculations and logical operations, especially one that can process, store and retrieve large amounts of data very quickly; now especially, a small one for personal or home use employed for manipulating text or graphics, accessing the Internet, or playing games or media."
```

## Compiling

- First clone the repository.
- Make sure you have the latest version of Rust install
- Change directory into the project and run `cargo build --release` for a release build.
- If you just want to run without optimizations, then run `cargo run <your-word>`

## TODO

- [ ] Add caching so that it will read from a local database if the word has already been searched.
