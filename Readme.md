# Maths Problem Gen

A maths problem generator aiming to be lightweight and fast. Creating it for maths revision and to
integrate into my friend's Discord bot

## Goals

- [x] Expressive system for representing maths in Rust
- [x] Automatic exact arithmetic solver
- [x] Render the maths problems to images (for Discord bot and usability)
- [ ] Generate simple maths problems
- [ ] Generate non-obvious multiple choice options for the problems
- [ ] Generate more complex maths problems (e.g. derivatives, integrals, exact trigonometric values
      etc.)

## Improvements

- [ ] Use a native maths renderer to avoid the performance issues with the current node-based maths
      rendering API that we're using.

## Dependencies

### Install on macOS

```sh
brew install pkg-config
brew install autoconf automake libtool
brew install icu4c
brew install graphite2
brew install libpng
brew install freetype2
```

### Other platforms

You're on your own.

## Running

### First times

```sh
git submodule update --init --recursive
```

### Start the rendering API

```sh
cd math-api
npm i
npm run start
```

### Run the program

```sh
cargo run
```
