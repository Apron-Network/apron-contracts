# Ink! Contracts for Apron Network

## Install dependencies
reference [https://paritytech.github.io/ink-docs/getting-started/setup](https://paritytech.github.io/ink-docs/getting-started/setup).

## Clone code
Run
```
git clone --recursive https://github.com/Apron-Network/apron-contracts.git
```

## Compile 
Please **use cargo-contract version 0.15**!
```bash
cargo install cargo-contract --vers ^0.15 --force --locked
```
Run `bash ./build.sh`

or

Run
```bash
cargo +nightly contract build
```
in each contract folder
