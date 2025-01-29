use std::{
    io::{Read, Write},
    path::Path,
};

use multiversx_sc_snippets::imports::Bech32Address;
use serde::{Deserialize, Serialize};

const STATE_FILE: &str = "state.toml";

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
    pair_address_egld_mex: Option<Bech32Address>,
    pair_address_egld_usdc: Option<Bech32Address>,
    pair_address_egld_utk: Option<Bech32Address>,
    energy_factory_address: Option<Bech32Address>,
    farm_address: Option<Bech32Address>,
    farm_staking_address: Option<Bech32Address>,
    permission_hub_address: Option<Bech32Address>,
}

impl State {
    // Deserializes state from file
    pub fn load_state() -> Self {
        if Path::new(STATE_FILE).exists() {
            let mut file = std::fs::File::open(STATE_FILE).unwrap();
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            toml::from_str(&content).unwrap()
        } else {
            Self::default()
        }
    }

    /// Sets the contract address
    pub fn set_address(&mut self, address: Bech32Address) {
        self.contract_address = Some(address);
    }

    pub fn set_pair_address_mex(&mut self, address: Bech32Address) {
        self.pair_address_egld_mex = Some(address);
    }

    pub fn set_pair_address_usdc(&mut self, address: Bech32Address) {
        self.pair_address_egld_usdc = Some(address);
    }

    pub fn set_pair_address_utk(&mut self, address: Bech32Address) {
        self.pair_address_egld_utk = Some(address);
    }

    pub fn set_energy_factory_address(&mut self, address: Bech32Address) {
        self.energy_factory_address = Some(address);
    }

    pub fn set_farm_address(&mut self, address: Bech32Address) {
        self.farm_address = Some(address);
    }

    pub fn set_farm_staking_address(&mut self, address: Bech32Address) {
        self.farm_staking_address = Some(address);
    }

    pub fn set_permission_hub_address(&mut self, address: Bech32Address) {
        self.permission_hub_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }

    pub fn current_pair_address_mex(&self) -> &Bech32Address {
        self.pair_address_egld_mex
            .as_ref()
            .expect("no known pair address, deploy first")
    }

    pub fn current_pair_address_usdc(&self) -> &Bech32Address {
        self.pair_address_egld_usdc
            .as_ref()
            .expect("no known pair address, deploy first")
    }

    pub fn current_pair_address_utk(&self) -> &Bech32Address {
        self.pair_address_egld_utk
            .as_ref()
            .expect("no known pair address, deploy first")
    }

    pub fn current_energy_factory_address(&self) -> &Bech32Address {
        self.energy_factory_address
            .as_ref()
            .expect("no known energy factory address, deploy first")
    }

    pub fn current_farm_address(&self) -> &Bech32Address {
        self.farm_address
            .as_ref()
            .expect("no known lp farm address, deploy first")
    }

    pub fn current_farm_staking_address(&self) -> &Bech32Address {
        self.farm_staking_address
            .as_ref()
            .expect("no known farm staking address, deploy first")
    }

    pub fn current_permission_hub_address(&self) -> &Bech32Address {
        self.permission_hub_address
            .as_ref()
            .expect("no known permission hub address, deploy first")
    }
}

impl Drop for State {
    // Serializes state to file
    fn drop(&mut self) {
        let mut file = std::fs::File::create(STATE_FILE).unwrap();
        file.write_all(toml::to_string(self).unwrap().as_bytes())
            .unwrap();
    }
}
