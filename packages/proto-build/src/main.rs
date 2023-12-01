//! Build Osmosis proto files. This build script clones the CosmosSDK and Osmosis version
//! specified in the COSMOS_SDK_REV and OSMOSIS_REV constant respectively and then
//! uses that to build the required proto files for further compilation.
//! This is based on the proto-compiler code in github.com/informalsystems/ibc-rs

use std::{env, path::PathBuf};

use proto_build::{
    code_generator::{CodeGenerator, CosmosProject},
    git,
};

/// The Cosmos SDK commit or tag to be cloned and used to build the proto files
const COSMOS_SDK_REV: &str = "v0.47.3-lsm5";

/// The persistence sdk commit or tag to be cloned and used to build the proto files
const PERSISTENCE_SDK_REV: &str = "v2.1.1";

const PSTAKE_NATIVE_REV: &str = "v2.5.0";

/// The wasmd commit or tag to be cloned and used to build the proto files
const WASMD_REV: &str = "v0.41.0";

// All paths must end with a / and either be absolute or include a ./ to reference the current
// working directory.

/// The directory generated cosmos-sdk proto files go into in this repo
const OUT_DIR: &str = "../persistence-std/src/types/";
/// Directory where the cosmos-sdk submodule is located
const COSMOS_SDK_DIR: &str = "../../dependencies/cosmos-sdk/";
/// Directory where the persistence sdk submodule is located
const PERSISTENCE_SDK_DIR: &str = "../../dependencies/persistence-sdk/";
/// Directory where the pstake native submodule is located
const PSTAKE_NATIVE_DIR: &str = "../../dependencies/pstake-native/";

/// Directory where the wasmd submodule is located
const WASMD_DIR: &str = "../../dependencies/wasmd/";

/// A temporary directory for proto building
const TMP_BUILD_DIR: &str = "/tmp/tmp-protobuf/";

pub fn generate() {
    let args: Vec<String> = env::args().collect();
    if args.iter().any(|arg| arg == "--update-deps") {
        git::update_submodule(COSMOS_SDK_DIR, COSMOS_SDK_REV);
        git::update_submodule(PERSISTENCE_SDK_DIR, PERSISTENCE_SDK_REV);
        git::update_submodule(PSTAKE_NATIVE_DIR, PSTAKE_NATIVE_REV);
        git::update_submodule(WASMD_DIR, WASMD_REV);
    }

    let tmp_build_dir: PathBuf = TMP_BUILD_DIR.parse().unwrap();
    let out_dir: PathBuf = OUT_DIR.parse().unwrap();

    let persistence_sdk_project = CosmosProject {
        name: "persistence-sdk".to_string(),
        version: PERSISTENCE_SDK_REV.to_string(),
        project_dir: PERSISTENCE_SDK_DIR.to_string(),
        include_mods: vec![],
    };

    let pstake_native_project = CosmosProject {
        name: "pstake-native".to_string(),
        version: PSTAKE_NATIVE_REV.to_string(),
        project_dir: PSTAKE_NATIVE_DIR.to_string(),
        include_mods: vec![],
    };

    let wasmd_project = CosmosProject {
        name: "wasmd".to_string(),
        version: WASMD_REV.to_string(),
        project_dir: WASMD_DIR.to_string(),
        include_mods: vec![],
    };

    let cosmos_project = CosmosProject {
        name: "cosmos".to_string(),
        version: COSMOS_SDK_REV.to_string(),
        project_dir: COSMOS_SDK_DIR.to_string(),
        include_mods: vec![
            "auth".to_string(),
            "authz".to_string(),
            "bank".to_string(),
            "base".to_string(),
            "gov".to_string(),
            // "feegrant".to_string(),
            "staking/v1beta1/genesis.proto".to_string(),
            "staking/v1beta1/staking.proto".to_string(),
            "staking/v1beta1/tx.proto".to_string(),
            "staking/v1beta1/query.proto".to_string(),
        ],
    };

    let code_generator = CodeGenerator::new(
        out_dir,
        tmp_build_dir,
        persistence_sdk_project,
        vec![pstake_native_project, cosmos_project, wasmd_project],
    );

    code_generator.generate();
}

fn main() {
    pretty_env_logger::init();
    generate();
}
