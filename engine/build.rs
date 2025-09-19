//! Build script for the arbitrage engine
//!
//! This script generates Rust code from Protocol Buffer definitions
//! for the Geyser gRPC interface.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Temporarily disable proto compilation due to conflicts
    // TODO: Fix proto file conflicts and re-enable

    // let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    //
    // // Configure tonic-build
    // tonic_build::configure()
    //     .build_server(false) // We only need the client
    //     .build_client(true)
    //     .out_dir(&out_dir)
    //     .compile(
    //         &[
    //             "proto/geyser.proto",
    //             "proto/confirmed_block.proto",
    //             "proto/transaction.proto",
    //         ],
    //         &["proto"],
    //     )?
    //
    // // Tell cargo to recompile if proto files change
    // println!("cargo:rerun-if-changed=proto/geyser.proto");
    // println!("cargo:rerun-if-changed=proto/confirmed_block.proto");
    // println!("cargo:rerun-if-changed=proto/transaction.proto");

    Ok(())
}
