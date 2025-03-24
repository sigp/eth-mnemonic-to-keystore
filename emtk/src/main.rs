use alloy_signer_local::{coins_bip39::English, MnemonicBuilder, PrivateKeySigner};
use clap::Parser;
use rand::thread_rng;
use std::path::PathBuf;
use std::process::exit;
use std::str::FromStr;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// CLI arguments
#[derive(Parser, Debug, Zeroize, ZeroizeOnDrop)]
#[command(version, about, long_about = None)]
struct Args {
    /// The mnemonic to decode.
    #[arg(short, long)]
    mnemonic: String,
    /// The index from which to derive the keystore.
    #[arg(short, long)]
    index: u32,
    /// The password to use to encrypt the resulting keystore.
    #[arg(short, long)]
    password: String,
    /// The directory in which to output the keystore file.
    #[arg(short, long)]
    output_dir: String,
}

fn main() {
    if let Err(e) = main_fallible() {
        eprintln!("{}", e);
        exit(-1);
    }
}

fn main_fallible() -> Result<(), String> {
    let args = Args::try_parse().map_err(|e| e.to_string())?;

    let output_dir = PathBuf::from_str(&args.output_dir)
        .map_err(|e| format!("Unable to parse output-dir: {e}"))?;
    if !output_dir.exists() {
        return Err(format!(
            "Output directory does not exist: {:?}",
            args.output_dir
        ));
    }

    let index = args.index;

    let wallet = MnemonicBuilder::<English>::default()
        .phrase(&args.mnemonic)
        .index(index)
        .map_err(|e| format!("Invalid index {index}: {e}"))?
        .build()
        .map_err(|e| format!("Unable to build keystore from mnemonic: {e}"))?;

    let mut rng = thread_rng();
    let (_keystore, path) = PrivateKeySigner::encrypt_keystore(
        &output_dir,
        &mut rng,
        wallet.to_bytes(),
        &args.password,
        None,
    )
    .map_err(|e| format!("Unable to build keystore: {e}"))?;

    print!("{path}");

    Ok(())
}
