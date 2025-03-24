# Eth-Mnemonic-to-Keystore (`emtk`)

A simple CLI utility for generated an encrypted keystore from a mnemonic.

## Features

- Based on [`alloy`](https://docs.rs/alloy/latest/alloy/).
- Minimal dependencies.
- Builds a stand-alone binary, ideal for air-gapped generation.

## Limitations

- Only supports English mnemonics. Submit a PR for more languages.

## Usage

### 1. Install the binary to your `~/.cargo/bin` (or equivalent)

```bash
cargo install --path emtk
```

### 2. Generate a keystore

```
emtk --index 0 \
    --password meow \
    --output-dir ./ \
    --mnemonic "wear crucial left gorilla zebra child similar salmon seed survey artwork invite digital giggle enjoy"
```

The keystore will be saved to `--output-dir` with a UUID filename (*without* a
`.json` extension).