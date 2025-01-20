#![allow(non_snake_case)]

mod config;
use proxies::*;

use config::Config;
use multiversx_sc_snippets::imports::*;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    path::Path,
};

const STATE_FILE: &str = "state.toml";
const PAIR_CONTRACT_CODE: &str = "../../dex/pair-mock/output/pair-mock.mxsc.json";
const ENERGY_FACTORY_CONTRACT_CODE: &str =
    "../../locked-asset/energy-factory/output/energy-factory.mxsc.json";
const LP_FARM_CONTRACT_CODE: &str =
    "../../dex/farm-with-locked-rewards/output/farm-with-locked-rewards.mxsc.json";
const FARM_STAKING_CONTRACT_CODE: &str = "../farm-staking/output/farm-staking.mxsc.json";
const CONTRACT_CODE: &str = "output/farm-staking-proxy.mxsc.json";
const PERMISSION_HUB_CONTRACT_CODE: &str =
    "../../dex/permissions-hub/output/permissions-hub.mxsc.json";
const ENERGY_FACTORY_MOCK_CONTRACT_CODE: &str =
    "../../energy-integration/energy-factory-mock/output/energy-factory-mock.mxsc.json";
const MIN_FARMING_EPOCHS: u64 = 2;
const PENALTY_PERCENTAGE: u64 = 10;
const LP_FARM_PER_BLOCK_REWARD_AMOUNT: u64 = 5_000;
const STAKING_FARM_PER_BLOCK_REWARD_AMOUNT: u64 = 1_000;
const EPOCHS_IN_YEAR: u64 = 360;
const LOCK_OPTIONS: &[u64] = &[EPOCHS_IN_YEAR, 5 * EPOCHS_IN_YEAR, 10 * EPOCHS_IN_YEAR]; // 1, 5 or 10 years
const USER_REWARDS_BASE_CONST: u64 = 10;
const USER_REWARDS_ENERGY_CONST: u64 = 3;
const USER_REWARDS_FARM_CONST: u64 = 2;
const MIN_ENERGY_AMOUNT_FOR_BOOSTED_YIELDS: u64 = 1;
const MIN_FARM_AMOUNT_FOR_BOOSTED_YIELDS: u64 = 1;
const PENALTY_PERCENTAGES: &[u64] = &[4_000, 6_000, 8_000];
const LEGACY_LOCKED_TOKEN_ID: &[u8] = b"LEGACY-123456";
const BASE_ASSET_TOKEN_ID: &[u8] = b"BASST-123456";
pub async fn farm_staking_proxy_cli() {
    env_logger::init();

    let mut args = std::env::args();
    let _ = args.next();
    let cmd = args.next().expect("at least one argument required");
    let mut interact = ContractInteract::new().await;
    match cmd.as_str() {
        "deploy" => {
            interact
                .deploy_metastaking(String::new(), String::new(), String::new(), String::new())
                .await
        }
        "upgrade" => interact.upgrade().await,
        "registerDualYieldToken" => interact.register_dual_yield_token().await,
        "getDualYieldTokenId" => interact.dual_yield_token().await,
        "getLpFarmAddress" => interact.lp_farm_address().await,
        "getStakingFarmAddress" => interact.staking_farm_address().await,
        "getPairAddress" => interact.pair_address().await,
        "getStakingTokenId" => interact.staking_token_id().await,
        "getFarmTokenId" => interact.staking_farm_token_id().await,
        "getLpTokenId" => interact.lp_token_id().await,
        "getLpFarmTokenId" => interact.lp_farm_token_id().await,
        "setPermissionsHubAddress" => interact.set_permissions_hub_address().await,
        "setEnergyFactoryAddress" => interact.set_energy_factory_address().await,
        "getEnergyFactoryAddress" => interact.energy_factory_address().await,
        "addSCAddressToWhitelist" => interact.add_sc_address_to_whitelist().await,
        "removeSCAddressFromWhitelist" => interact.remove_sc_address_from_whitelist().await,
        "isSCAddressWhitelisted" => interact.is_sc_address_whitelisted().await,
        "stakeFarmTokens" => interact.stake_farm_tokens().await,
        "claimDualYield" => interact.claim_dual_yield_endpoint().await,
        "unstakeFarmTokens" => interact.unstake_farm_tokens().await,
        //"stakeFarmOnBehalf" => interact.stake_farm_on_behalf().await,
        "claimDualYieldOnBehalf" => interact.claim_dual_yield_on_behalf().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
    pair_address: Option<Bech32Address>,
    energy_factory_address: Option<Bech32Address>,
    lp_farm_address: Option<Bech32Address>,
    farm_staking_address: Option<Bech32Address>,
    permission_hub_address: Option<Bech32Address>,
    energy_factory_mock_address: Option<Bech32Address>,
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

    pub fn set_pair_address(&mut self, address: Bech32Address) {
        self.pair_address = Some(address);
    }

    pub fn set_energy_factory_address(&mut self, address: Bech32Address) {
        self.energy_factory_address = Some(address);
    }

    pub fn set_lp_farm_address(&mut self, address: Bech32Address) {
        self.lp_farm_address = Some(address);
    }

    pub fn set_farm_staking_address(&mut self, address: Bech32Address) {
        self.farm_staking_address = Some(address);
    }

    pub fn set_permission_hub_address(&mut self, address: Bech32Address) {
        self.permission_hub_address = Some(address);
    }

    pub fn set_energy_factory_mock_address(&mut self, address: Bech32Address) {
        self.energy_factory_mock_address = Some(address);
    }

    /// Returns the contract address
    pub fn current_address(&self) -> &Bech32Address {
        self.contract_address
            .as_ref()
            .expect("no known contract, deploy first")
    }

    pub fn current_pair_address(&self) -> &Bech32Address {
        self.pair_address
            .as_ref()
            .expect("no known pair address, deploy first")
    }

    pub fn current_energy_factory_address(&self) -> &Bech32Address {
        self.energy_factory_address
            .as_ref()
            .expect("no known energy factory address, deploy first")
    }

    pub fn current_lp_farm_address(&self) -> &Bech32Address {
        self.lp_farm_address
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

    pub fn current_energy_factory_mock_address(&self) -> &Bech32Address {
        self.energy_factory_mock_address
            .as_ref()
            .expect("no known energy factory mock address, deploy first")
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

pub struct ContractInteract {
    interactor: Interactor,
    pub wallet_address: Address,
    pub bob_address: Address,
    contract_code: String,
    pair_contract_code: String,
    energy_factory_contract_code: String,
    energy_factory_mock_contract_code: String,
    lp_farm_contract_code: String,
    farm_staking_contract_code: String,
    permission_hub_contract_code: String,
    state: State,
}

impl ContractInteract {
    pub async fn new() -> Self {
        let config = Config::new();
        let mut interactor = Interactor::new(config.gateway_uri())
            .await
            .use_chain_simulator(config.use_chain_simulator());

        interactor.set_current_dir_from_workspace("farm-staking/farm-staking-proxy");
        let wallet_address = interactor.register_wallet(test_wallets::alice()).await;
        let bob_address = interactor.register_wallet(test_wallets::bob()).await;

        // Useful in the chain simulator setting
        // generate blocks until ESDTSystemSCAddress is enabled
        interactor.generate_blocks_until_epoch(1).await.unwrap();

        ContractInteract {
            interactor,
            wallet_address,
            bob_address,
            contract_code: CONTRACT_CODE.to_string(),
            pair_contract_code: PAIR_CONTRACT_CODE.to_string(),
            energy_factory_contract_code: ENERGY_FACTORY_CONTRACT_CODE.to_string(),
            energy_factory_mock_contract_code: ENERGY_FACTORY_MOCK_CONTRACT_CODE.to_string(),
            lp_farm_contract_code: LP_FARM_CONTRACT_CODE.to_string(),
            farm_staking_contract_code: FARM_STAKING_CONTRACT_CODE.to_string(),
            permission_hub_contract_code: PERMISSION_HUB_CONTRACT_CODE.to_string(),
            state: State::load_state(),
        }
    }

    pub async fn deploy_metastaking(
        &mut self,
        staking_token: String,
        lp_token: String,
        staking_farm: String,
        lp_farm: String,
    ) {
        let code_path = MxscPath::new(&self.contract_code);
        let energy_factory_address = self.state.current_energy_factory_address();
        let lp_farm_address = self.state.current_lp_farm_address();
        let staking_farm_address = self.state.current_farm_staking_address();
        let pair_address = self.state.current_pair_address();
        let staking_token_id = TokenIdentifier::from_esdt_bytes(staking_token);
        let lp_farm_token_id = TokenIdentifier::from_esdt_bytes(lp_farm);
        let staking_farm_token_id = TokenIdentifier::from_esdt_bytes(staking_farm);
        let lp_token_id = TokenIdentifier::from_esdt_bytes(lp_token);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(120_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .init(
                energy_factory_address,
                lp_farm_address,
                staking_farm_address,
                pair_address,
                staking_token_id,
                lp_farm_token_id,
                staking_farm_token_id,
                lp_token_id,
            )
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state.set_address(Bech32Address::from_bech32_string(
            new_address_bech32.clone(),
        ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn deploy_pair(
        &mut self,
        first_token: String,
        second_token: String,
        lp_token: String,
    ) {
        let code_path = MxscPath::new(&self.pair_contract_code);
        let token_a = OptionalValue::Some(TokenIdentifier::from_esdt_bytes(first_token));
        let token_b = OptionalValue::Some(TokenIdentifier::from_esdt_bytes(second_token));
        let lp_token = OptionalValue::Some(TokenIdentifier::from_esdt_bytes(lp_token));

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(pair_proxy::PairMockProxy)
            .init(
                token_a,
                token_b,
                lp_token,
                OptionalValue::Some(self.wallet_address.clone()),
                OptionalValue::Some(true),
                OptionalValue::Some(false),
            )
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_pair_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn deploy_energy_factory_mock(&mut self) {
        let code_path = MxscPath::new(&self.energy_factory_mock_contract_code);
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(energy_factory_mock_proxy::EnergyFactoryMockProxy)
            .init()
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_energy_factory_mock_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn deploy_energy_factory(&mut self) {
        let code_path = MxscPath::new(&self.energy_factory_contract_code);
        let mut lock_options = MultiValueEncoded::new();
        for (option, penalty) in LOCK_OPTIONS.iter().zip(PENALTY_PERCENTAGES.iter()) {
            lock_options.push((*option, *penalty).into());
        }
        let old_locked_address = self.state.current_energy_factory_mock_address();
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(160_000_000u64)
            .typed(energy_factory_proxy::SimpleLockEnergyProxy)
            .init(
                TokenIdentifier::from_esdt_bytes(BASE_ASSET_TOKEN_ID.to_vec()),
                TokenIdentifier::from_esdt_bytes(LEGACY_LOCKED_TOKEN_ID.to_vec()),
                old_locked_address,
                0u64,
                lock_options,
            )
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_energy_factory_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_energy_factory_address())
            .gas(30_000_000u64)
            .typed(energy_factory_proxy::SimpleLockEnergyProxy)
            .unpause_endpoint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn deploy_farm_with_locked_rewards(
        &mut self,
        farming_token: String,
        rewards_token: String,
    ) {
        let code_path = MxscPath::new(&self.lp_farm_contract_code);
        let rewards_token_id = TokenIdentifier::from_esdt_bytes(rewards_token);
        let farming_token_id = TokenIdentifier::from_esdt_bytes(farming_token);
        let pair_address = self.state.current_pair_address();
        let admins = MultiValueEncoded::new();
        let division_constant = BigUint::from(10_000_000_000u64);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(200_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .init(
                rewards_token_id,
                farming_token_id,
                division_constant,
                pair_address,
                &self.wallet_address,
                admins,
            )
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_lp_farm_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn deploy_farm_staking(&mut self, staking_token: String) {
        let code_path = MxscPath::new(&self.farm_staking_contract_code);
        let farming_token_id = TokenIdentifier::from_esdt_bytes(staking_token);
        let division_constant = BigUint::from(10_000_000_000u64);
        let admins = MultiValueEncoded::new();

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(140_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .init(
                farming_token_id,
                division_constant,
                50u64,
                5u64,
                &self.wallet_address,
                admins,
            )
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_farm_staking_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn deploy_permission_hub(&mut self) {
        let code_path = MxscPath::new(&self.permission_hub_contract_code);

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(permission_hub_proxy::PermissionsHubProxy)
            .init()
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_permission_hub_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn add_sc_address_to_whitelist_lp(&mut self, sc_address: Address) {
        let address_to_whitelist = ManagedAddress::from_address(&sc_address);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_permission_hub_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .add_sc_address_to_whitelist(address_to_whitelist)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn upgrade(&mut self) {
        let contract_code = MxscPath::new(&self.contract_code);
        let response = self
            .interactor
            .tx()
            .to(self.state.current_address())
            .from(&self.wallet_address)
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .upgrade()
            .code(contract_code)
            .code_metadata(CodeMetadata::UPGRADEABLE)
            .returns(ReturnsNewAddress)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn register_dual_yield_token(&mut self) {
        let egld_amount = BigUint::<StaticApi>::from(0u128);

        let token_display_name = ManagedBuffer::new_from_bytes(&b""[..]);
        let token_ticker = ManagedBuffer::new_from_bytes(&b""[..]);
        let num_decimals = 0u32;

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .register_dual_yield_token(token_display_name, token_ticker, num_decimals)
            .egld(egld_amount)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn dual_yield_token(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .dual_yield_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn lp_farm_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .lp_farm_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn staking_farm_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .staking_farm_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn pair_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .pair_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn staking_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .staking_token_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn staking_farm_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .staking_farm_token_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn lp_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .lp_token_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn lp_farm_token_id(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .lp_farm_token_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn set_permissions_hub_address(&mut self) {
        let address = self.state.current_permission_hub_address();

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .set_permissions_hub_address(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_energy_factory_address(&mut self) {
        let sc_address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .set_energy_factory_address(sc_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn energy_factory_address(&mut self) {
        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .energy_factory_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn add_sc_address_to_whitelist(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .add_sc_address_to_whitelist(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn remove_sc_address_from_whitelist(&mut self) {
        let address = bech32::decode("");

        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .remove_sc_address_from_whitelist(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn remove_sc_address_from_whitelist_lp(&mut self, sc_address: Address) {
        let address_to_remove = ManagedAddress::from_address(&sc_address);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_permission_hub_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .remove_sc_address_from_whitelist(address_to_remove)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn is_sc_address_whitelisted(&mut self) {
        let address = bech32::decode("");

        let result_value = self
            .interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .is_sc_address_whitelisted(address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {result_value:?}");
    }

    pub async fn stake_farm_tokens(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let opt_orig_caller = OptionalValue::Some(bech32::decode(""));

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .stake_farm_tokens(opt_orig_caller)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn claim_dual_yield_endpoint(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let opt_orig_caller = OptionalValue::Some(bech32::decode(""));

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .claim_dual_yield_endpoint(opt_orig_caller)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn unstake_farm_tokens(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        let pair_first_token_min_amount = BigUint::<StaticApi>::from(0u128);
        let pair_second_token_min_amount = BigUint::<StaticApi>::from(0u128);
        let opt_orig_caller = OptionalValue::Some(bech32::decode(""));

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .unstake_farm_tokens(
                pair_first_token_min_amount,
                pair_second_token_min_amount,
                opt_orig_caller,
            )
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn stake_farm_on_behalf(
        &mut self,
        caller: Address,
        original_owner: Address,
        token_id: String,
        token_nonce: u64,
        token_amount: BigUint<StaticApi>,
        error_message: Option<&str>,
    ) {
        let response = self
            .interactor
            .tx()
            .from(caller)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .stake_farm_on_behalf(original_owner)
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsHandledOrError::new())
            .run()
            .await;

        if let Err(error) = response {
            assert_eq!(error_message, Some(error.message.as_str()))
        }
    }

    pub async fn claim_dual_yield_on_behalf(&mut self) {
        let token_id = String::new();
        let token_nonce = 0u64;
        let token_amount = BigUint::<StaticApi>::from(0u128);

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .claim_dual_yield_on_behalf()
            .payment((
                TokenIdentifier::from(token_id.as_str()),
                token_nonce,
                token_amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn issue_fungible_token(
        &mut self,
        from_address: Address,
        token_display_name: &[u8],
        token_ticker: &[u8],
        initial_supply: RustBigUint,
        num_decimals: usize,
    ) -> String {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);

        self.interactor
            .tx()
            .from(from_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_fungible(
                issue_cost,
                token_display_name,
                token_ticker,
                initial_supply,
                FungibleTokenProperties {
                    num_decimals,
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_mint: true,
                    can_burn: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                },
            )
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await
    }

    pub async fn issue_and_set_all_roles(
        &mut self,
        from_address: Address,
        token_display_name: &[u8],
        token_ticker: &[u8],
    ) -> String {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);

        self.interactor
            .tx()
            .from(from_address)
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .issue_non_fungible(
                issue_cost,
                token_display_name,
                token_ticker,
                NonFungibleTokenProperties {
                    can_freeze: true,
                    can_wipe: true,
                    can_pause: true,
                    can_change_owner: true,
                    can_upgrade: true,
                    can_add_special_roles: true,
                    can_transfer_create_role: true,
                },
            )
            .returns(ReturnsNewTokenIdentifier)
            .run()
            .await
    }

    pub async fn register_farm_token(&mut self) {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(100_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .register_farm_token("LPFarming", "LPF", 18u8)
            .egld(issue_cost)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_special_roles_lp_farm(&mut self, farm_token: String, rewards_token: String) {
        let farm_token_roles = [EsdtLocalRole::Burn];
        self.interactor
            .tx()
            .from(self.wallet_address.clone())
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .set_special_roles(
                self.state.current_lp_farm_address().clone(),
                TokenIdentifier::from_esdt_bytes(farm_token),
                farm_token_roles.into_iter(),
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        let reward_token_roles = [EsdtLocalRole::Mint];
        self.interactor
            .tx()
            .from(self.wallet_address.clone())
            .to(ESDTSystemSCAddress)
            .gas(100_000_000u64)
            .typed(ESDTSystemSCProxy)
            .set_special_roles(
                self.state.current_lp_farm_address().clone(),
                TokenIdentifier::from_esdt_bytes(rewards_token),
                reward_token_roles.into_iter(),
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn send_tokens_to_other_wallet(
        &mut self,
        receiver: Address,
        token_id: String,
        amount: u64,
    ) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(receiver.clone())
            .single_esdt(
                &TokenIdentifier::from_esdt_bytes(token_id),
                0,
                &amount.into(),
            ) // .transfer()
            .run()
            .await;
    }

    pub async fn check_user_balance(&self, user_address: Address) {
        let res = self
            .interactor
            .proxy
            .get_account_esdt_tokens(&user_address)
            .await;

        println!("User balance: {:?}", res);
    }

    pub async fn add_proxy_to_whitelist(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .add_sc_address_to_whitelist(self.state.current_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn blacklist_address(&mut self, address: Address) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_permission_hub_address())
            .gas(30_000_000u64)
            .typed(permission_hub_proxy::PermissionsHubProxy)
            .blacklist(ManagedAddress::from_address(&address))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn whitelist_address(&mut self, address: Address) {
        let addresses = vec![ManagedAddress::from_address(&address)];
        let mut whitelisted_adresses = MultiValueEncoded::new();
        for address in addresses {
            whitelisted_adresses.push(address);
        }
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_permission_hub_address())
            .gas(30_000_000u64)
            .typed(permission_hub_proxy::PermissionsHubProxy)
            .whitelist(whitelisted_adresses)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn remove_whitelist_address(&mut self, address: Address) {
        let addresses = vec![ManagedAddress::from_address(&address)];
        let mut whitelisted_adresses = MultiValueEncoded::new();
        for address in addresses {
            whitelisted_adresses.push(address);
        }
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_permission_hub_address())
            .gas(30_000_000u64)
            .typed(permission_hub_proxy::PermissionsHubProxy)
            .remove_whitelist(whitelisted_adresses)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn enter_farm_endpoint(&mut self, token_id: String, amount: u64) {
        let original_caller: OptionalValue<[u8; 32]> = OptionalValue::None;
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .enter_farm_endpoint(original_caller)
            .esdt(EsdtTokenPayment::new(
                TokenIdentifier::from_esdt_bytes(token_id),
                0,
                amount.into(),
            ))
            .returns(ReturnsResult)
            .run()
            .await;
    }

    pub async fn set_minimum_farming_epochs(&mut self, epochs: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_minimum_farming_epochs(epochs)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_penalty_percent(&mut self, percentage_wanted: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_penalty_percent(percentage_wanted)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_active_state_lp(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .resume()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_per_block_rewards(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_per_block_rewards_endpoint(BigUint::from(LP_FARM_PER_BLOCK_REWARD_AMOUNT))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_produce_rewards_enabled(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .start_produce_rewards_endpoint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_lock_epochs(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_lock_epochs(LOCK_OPTIONS[2])
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_locking_sc_address(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_locking_sc_address(ManagedAddress::from_address(
                &self.state.current_energy_factory_address().to_address(),
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_energy_factory_address_lp(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_energy_factory_address(self.state.current_energy_factory_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_boosted_yields_factors_lp(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_boosted_yields_factors(
                BigUint::from(USER_REWARDS_BASE_CONST),
                BigUint::from(USER_REWARDS_ENERGY_CONST),
                BigUint::from(USER_REWARDS_FARM_CONST),
                BigUint::from(MIN_ENERGY_AMOUNT_FOR_BOOSTED_YIELDS),
                BigUint::from(MIN_FARM_AMOUNT_FOR_BOOSTED_YIELDS),
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_energy_factory_address_farm_staking(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .set_energy_factory_address(self.state.current_energy_factory_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn register_farm_token_farm_staking(&mut self) {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(90_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .register_farm_token("StakingFarm", "STKF", 18u8)
            .egld(issue_cost)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_state_active_farm_staking(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .resume()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_per_block_rewards_farm_staking(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .set_per_block_rewards(BigUint::from(STAKING_FARM_PER_BLOCK_REWARD_AMOUNT))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_produce_rewards_enabled_farm_staking(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .start_produce_rewards_endpoint()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_boosted_yields_factors_farm_staking(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .set_boosted_yields_factors(
                BigUint::from(USER_REWARDS_BASE_CONST),
                BigUint::from(USER_REWARDS_ENERGY_CONST),
                BigUint::from(USER_REWARDS_FARM_CONST),
                BigUint::from(MIN_ENERGY_AMOUNT_FOR_BOOSTED_YIELDS),
                BigUint::from(MIN_FARM_AMOUNT_FOR_BOOSTED_YIELDS),
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn get_farm_token_id(&mut self) -> TokenIdentifier<StaticApi> {
        self.interactor
            .query()
            .to(self.state.current_lp_farm_address())
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .farm_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_farm_token_id_staking(&mut self) -> TokenIdentifier<StaticApi> {
        self.interactor
            .query()
            .to(self.state.current_farm_staking_address())
            .typed(farm_staking_proxy::FarmStakingProxy)
            .farm_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn issue_locked_token(&mut self) -> TokenIdentifier<StaticApi> {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_energy_factory_address())
            .gas(90_000_000u64)
            .typed(energy_factory_proxy::SimpleLockEnergyProxy)
            .issue_locked_token("LockedToken", "LCKD", 18u8)
            .egld(issue_cost)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        self.interactor
            .query()
            .to(self.state.current_energy_factory_address())
            .typed(energy_factory_proxy::SimpleLockEnergyProxy)
            .locked_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_locked_token_id_wanted(&mut self) -> TokenIdentifier<StaticApi> {
        self.interactor
            .query()
            .to(self.state.current_lp_farm_address())
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .get_locked_token_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn get_storage_keys_energy_factory(&mut self) {
        let result = self
            .interactor
            .proxy
            .get_account_storage_keys(&self.state.current_energy_factory_address().to_address())
            .await;
        println!("Storage keys: {:?}", result);
    }

    pub async fn get_energy_factory_address_lp(&mut self) {
        let result = self
            .interactor
            .query()
            .to(self.state.current_lp_farm_address())
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .energy_factory_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
        assert_eq!(
            result,
            self.state.current_energy_factory_address().to_address()
        );
    }

    pub async fn setup_lp_farm(&mut self) {
        self.register_farm_token().await;
        self.set_minimum_farming_epochs(MIN_FARMING_EPOCHS).await;
        self.set_penalty_percent(PENALTY_PERCENTAGE).await;
        self.set_active_state_lp().await;
        self.set_per_block_rewards().await;
        self.set_produce_rewards_enabled().await;
        self.set_lock_epochs().await;
        self.set_locking_sc_address().await;
        self.set_energy_factory_address_lp().await;
        self.set_boosted_yields_factors_lp().await;
    }

    pub async fn setup_farm_staking(&mut self) {
        self.register_farm_token_farm_staking().await;
        self.set_energy_factory_address_farm_staking().await;
        self.set_state_active_farm_staking().await;
        self.set_per_block_rewards_farm_staking().await;
        self.set_produce_rewards_enabled_farm_staking().await;
        self.set_boosted_yields_factors_farm_staking().await;
    }

    pub async fn setup_tests(&mut self) -> String {
        let farm_amount = 100_000_000u64;
        let WEGLD = self
            .issue_fungible_token(
                self.wallet_address.clone(),
                "WrappedEGLD".as_bytes(),
                "WEGLD".as_bytes(),
                RustBigUint::from(1000u64),
                18,
            )
            .await;
        let RIDE = self
            .issue_fungible_token(
                self.wallet_address.clone(),
                "RideToken".as_bytes(),
                "RIDE".as_bytes(),
                RustBigUint::from(1000u64),
                18,
            )
            .await;
        let LP = self
            .issue_fungible_token(
                self.wallet_address.clone(),
                "LpFarm".as_bytes(),
                "LPTST".as_bytes(),
                RustBigUint::from(farm_amount * 2 + 1),
                18,
            )
            .await;
        let MEX = self
            .issue_fungible_token(
                self.wallet_address.clone(),
                "MexToken".as_bytes(),
                "MEX".as_bytes(),
                RustBigUint::from(1000u64),
                18,
            )
            .await;
        self.deploy_energy_factory_mock().await;
        self.deploy_energy_factory().await;
        self.issue_locked_token().await;
        self.deploy_pair(WEGLD.clone(), RIDE.clone(), LP.clone())
            .await;
        self.deploy_farm_with_locked_rewards(LP.clone(), MEX.clone())
            .await;
        self.setup_lp_farm().await;
        let LP_FARM = self.get_farm_token_id().await;
        self.set_special_roles_lp_farm(LP.clone(), RIDE.clone())
            .await;
        self.deploy_farm_staking(RIDE.clone()).await;
        self.setup_farm_staking().await;
        let STAKING_FARM = self.get_farm_token_id_staking().await;
        self.deploy_metastaking(
            RIDE.clone(),
            LP.clone(),
            STAKING_FARM.to_string(),
            LP_FARM.to_string(),
        )
        .await;
        self.deploy_permission_hub().await;
        self.add_proxy_to_whitelist().await;
        self.send_tokens_to_other_wallet(self.bob_address.clone(), LP.clone(), 1u64)
            .await;
        self.set_permissions_hub_address().await;

        LP
    }
    pub async fn set_boosted_yields_rewards_percentage(&mut self, percentage_wanted: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .set_boosted_yields_rewards_percentage(percentage_wanted)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }

    pub async fn set_boosted_yields_rewards_percentage_lp(&mut self, percentage_wanted: u64) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_lp_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_boosted_yields_rewards_percentage(percentage_wanted)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        println!("Result: {response:?}");
    }
}
