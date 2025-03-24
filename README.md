# Eth-Mnemonic-to-Keystore (`emtk`)

A simple CLI utility for generated an encrypted keystore from a mnemonic.

This tool has not been audited by Sigma Prime. Use at your own risk.

## Features

- Based on [`alloy`](https://docs.rs/alloy/latest/alloy/).
- Minimal dependencies.
- Builds a stand-alone binary, ideal for air-gapped generation.
- Easy cross-compilation.

## Limitations

- Only supports English mnemonics. Submit a PR for more languages.

## Usage

### 1. Install the binary to your `~/.cargo/bin` (or equivalent)

```bash
make install
```

### 2. Generate a keystore

```
emtk --index 0 \
    --password meow \
    --output-dir ./ \
    --mnemonic "wear crucial left gorilla zebra child similar salmon seed survey artwork invite digital giggle enjoy"
```

The `--index`'th address will be derived from the `--mnemonic`. The resulting
keystore will be saved to `--output-dir` with a UUID filename (*without* a
`.json` extension). The keystore will be encrypted with `--password`.

## Cross-Compilation

Cross-compilation is provided via [`cross`](https://github.com/cross-rs/cross/)
and therefore uses Docker.

```bash
# x86_64 (Intel)
make build-x86-64
# aarch64 (ARM)
make build-aarch64
```

Binaries will be compiled to `./emtk/target/<PLATFORM>/release/emtk`.