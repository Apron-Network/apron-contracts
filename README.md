# Apron-contracts
contracts for Apron, using [ink!](https://github.com/paritytech/ink).

## install cargo-contract
reference [here]().

## compile contracts
```bash
cargo +nightly contract build
```

## install by polkadot.js apps
visit [polkadot.js apps](https://polkadot.js.org/apps/), and connect Apron node.
then `Develpoer`->`Contract`->`upload WASM`.

## generate a new contract
```bash
cargo contract new test
```

## build all contracts
```bash
./build.sh
```
the ABI and wasm copied in `./traget` dir.