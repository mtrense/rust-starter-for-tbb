# Rust Starter for the Tech Brownbag

## Getting started

### Installing rust

Mac OS

Install rustup using brew

```bash
brew install rustup-init
```

and run it

```bash
rustup-init
```

After that you should have a working rustup package manager. Use it to set the default toolchain to `stable`:

```bash
rustup default stable
```

This installs `rustc` and some nice tools like `cargo` and `cargo-clippy`.

If you already have installed rust, you can update to the latest version by running:

```bash
rustup update
```

#### VS Code

Recommended addons:

- `rust-lang.rust`
- `bungcip.better-toml`
- `serayuzgur.crates`
- `belfz.search-crates-io`

#### Crates used here

- Actix: https://actix.rs
- Clap: https://clap.rs
- Serde: https://serde.rs
