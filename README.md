# How to run

- Install rust https://rustup.rs/
- Go to https://www.powerlanguage.co.uk/wordle/
- `git clone https://github.com/jeteve/wordle-solver.git`
- `cd wordle-solver`
- `cargo run`
- Do what it says.

# Other games covered

If you use the `-s 6` option, you can also solve https://www.wordle2.in/

This seems to also be able to solve https://qntm.org/files/wordle/index.html


# About `cargo install`

The included executable will not run with cargo install as it
needs a file asset to load its dictionary. So until I change
the packaging, please use `git clone` and `cargo run` to run this.

# Developing

This is best developed in VS Code, using the provided `.devcontainer` with Docker.

# Releasing

Use `cargo release`