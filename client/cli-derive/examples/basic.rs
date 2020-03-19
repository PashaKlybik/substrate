#![allow(unused_variables, dead_code)]

use sc_cli::{SharedParams, CliConfiguration, ImportParams, KeystoreParams};
use sc_cli_derive::{spec_factory, substrate_cli_params};
use serde::{Deserialize, Serialize};
use sp_runtime::BuildStorage;
use sp_storage::Storage;

#[derive(Serialize, Deserialize)]
struct MyGenesisConfig;
struct MyExtension;
struct MyCli {
	shared: SharedParams,
	import: ImportParams,
	keystore: KeystoreParams,
}

impl BuildStorage for MyGenesisConfig {
	fn assimilate_storage(&self, _: &mut Storage) -> Result<(), String> {
		unimplemented!()
	}
}

#[spec_factory(cli = MyCli, support_url = "http://example.org", copyright_start_year = 2020)]
fn spec_factory(id: &str) -> Result<Option<sc_service::ChainSpec<MyGenesisConfig>>, String> {
	Ok(None)
}

#[substrate_cli_params(shared_params = shared, import_params = import, keystore_params = keystore)]
impl CliConfiguration for MyCli {
}

fn main() {}
