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
const PAIR_CONTRACT_CODE: &str = "../../dex/pair/output/pair.mxsc.json";
const ENERGY_FACTORY_CONTRACT_CODE: &str =
    "../../locked-asset/energy-factory/output/energy-factory.mxsc.json";
const FARM_CONTRACT_CODE: &str =
    "../../dex/farm-with-locked-rewards/output/farm-with-locked-rewards.mxsc.json";
const FARM_STAKING_CONTRACT_CODE: &str = "../farm-staking/output/farm-staking.mxsc.json";
const CONTRACT_CODE: &str = "output/farm-staking-proxy.mxsc.json";
const PERMISSION_HUB_CONTRACT_CODE: &str =
    "../../dex/permissions-hub/output/permissions-hub.mxsc.json";
const LOCKED_ASSET_FACTORY_CONTRACT_CODE: &str =
    "../../legacy-contracts/factory-legacy/output/factory-legacy.mxsc.json";
const ROUTER_CONTRACT_CODE: &str = "../../dex/router/output/router.mxsc.json";
const MIN_FARMING_EPOCHS: u64 = 7;
const PENALTY_PERCENTAGE: u64 = 300;
const STAKING_FARM_PER_BLOCK_REWARD_AMOUNT: u64 = 4138000000000000000;
const EPOCHS_IN_YEAR: u64 = 360;
const LOCK_OPTIONS: &[u64] = &[EPOCHS_IN_YEAR, 5 * EPOCHS_IN_YEAR, 10 * EPOCHS_IN_YEAR]; // 1, 5 or 10 years
const USER_REWARDS_BASE_CONST: u64 = 2;
const USER_REWARDS_ENERGY_CONST: u64 = 1;
const USER_REWARDS_FARM_CONST: u64 = 0;
const MIN_ENERGY_AMOUNT_FOR_BOOSTED_YIELDS: u64 = 1;
const MIN_FARM_AMOUNT_FOR_BOOSTED_YIELDS: u64 = 1;
const PENALTY_PERCENTAGES: &[u64] = &[4_000, 6_000, 8_000];
const LEGACY_LOCKED_TOKEN_ID: &[u8] = b"LEGACY-123456";
const BASE_ASSET_TOKEN_ID: &[u8] = b"BASST-123456";
const DIVISION_SAFETY_CONSTANT: u64 = 1_000_000_000_000;
const BOOSTED_YIELDS_PERCENTAGE: u64 = 6000;
const BOOSTED_YIELDS_PERCENTAGE_STAKING: u64 = 4000;
const LOCK_EPOCHS: u64 = 1440;
const REWARDS_PER_BLOCK_FARM: u64 = 10000;

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
        } //     "upgrade" => interact.upgrade().await,
        //     "registerDualYieldToken" => interact.register_dual_yield_token().await,
        //     "getDualYieldTokenId" => interact.dual_yield_token().await,
        //     "getLpFarmAddress" => interact.lp_farm_address().await,
        //     "getStakingFarmAddress" => interact.staking_farm_address().await,
        //     "getPairAddress" => interact.pair_address().await,
        //     "getStakingTokenId" => interact.staking_token_id().await,
        //     "getFarmTokenId" => interact.staking_farm_token_id().await,
        //     "getLpTokenId" => interact.lp_token_id().await,
        //     "getLpFarmTokenId" => interact.lp_farm_token_id().await,
        //     "setPermissionsHubAddress" => interact.set_permissions_hub_address().await,
        //     "setEnergyFactoryAddress" => interact.set_energy_factory_address().await,
        //     "getEnergyFactoryAddress" => interact.energy_factory_address().await,
        //     "addSCAddressToWhitelist" => interact.add_sc_address_to_whitelist().await,
        //     "removeSCAddressFromWhitelist" => interact.remove_sc_address_from_whitelist().await,
        //     "isSCAddressWhitelisted" => interact.is_sc_address_whitelisted().await,
        //     "stakeFarmTokens" => interact.stake_farm_tokens().await,
        //     "claimDualYield" => interact.claim_dual_yield_endpoint().await,
        //     "unstakeFarmTokens" => interact.unstake_farm_tokens().await,
        //     "stakeFarmOnBehalf" => interact.stake_farm_on_behalf().await,
        //     "claimDualYieldOnBehalf" => interact.claim_dual_yield_on_behalf().await,
        _ => panic!("unknown command: {}", &cmd),
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    contract_address: Option<Bech32Address>,
    pair_address: Option<Bech32Address>,
    energy_factory_address: Option<Bech32Address>,
    farm_address: Option<Bech32Address>,
    farm_staking_address: Option<Bech32Address>,
    permission_hub_address: Option<Bech32Address>,
    locked_asset_factory_address: Option<Bech32Address>,
    router_address: Option<Bech32Address>,
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

    pub fn set_farm_address(&mut self, address: Bech32Address) {
        self.farm_address = Some(address);
    }

    pub fn set_farm_staking_address(&mut self, address: Bech32Address) {
        self.farm_staking_address = Some(address);
    }

    pub fn set_permission_hub_address(&mut self, address: Bech32Address) {
        self.permission_hub_address = Some(address);
    }

    pub fn set_locked_asset_factory_address(&mut self, address: Bech32Address) {
        self.locked_asset_factory_address = Some(address);
    }

    pub fn set_router_address(&mut self, address: Bech32Address) {
        self.router_address = Some(address);
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

    pub fn current_locked_asset_factory_address(&self) -> &Bech32Address {
        self.locked_asset_factory_address
            .as_ref()
            .expect("no known energy factory mock address, deploy first")
    }

    pub fn current_router_address(&self) -> &Bech32Address {
        self.router_address
            .as_ref()
            .expect("no known router address, deploy first")
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
    locked_asset_factory_contract_code: String,
    farm_contract_code: String,
    farm_staking_contract_code: String,
    permission_hub_contract_code: String,
    router_contract_code: String,
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
            locked_asset_factory_contract_code: LOCKED_ASSET_FACTORY_CONTRACT_CODE.to_string(),
            farm_contract_code: FARM_CONTRACT_CODE.to_string(),
            farm_staking_contract_code: FARM_STAKING_CONTRACT_CODE.to_string(),
            permission_hub_contract_code: PERMISSION_HUB_CONTRACT_CODE.to_string(),
            router_contract_code: ROUTER_CONTRACT_CODE.to_string(),
            state: State::load_state(),
        }
    }

    pub async fn deploy_pair_template(
        &mut self,
        first_token: String,
        second_token: String,
    ) -> Address {
        let code_path = MxscPath::new(&self.pair_contract_code);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .gas(120_000_000u64)
            .typed(pair_proxy::PairProxy)
            .init(
                TokenIdentifier::from_esdt_bytes(first_token),
                TokenIdentifier::from_esdt_bytes(second_token),
                ManagedAddress::zero(),
                ManagedAddress::zero(),
                0u64,
                0u64,
                ManagedAddress::zero(),
                MultiValueEncoded::new(),
            )
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await
    }

    pub async fn deploy_router(&mut self, pair_template: Address) {
        let pair_template_address = OptionalValue::Some(pair_template);
        let code_path = MxscPath::new(&self.router_contract_code);
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(120_000_000u64)
            .typed(router_proxy::RouterProxy)
            .init(pair_template_address)
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_router_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn deploy_pair(&mut self, first_token: String, second_token: String) {
        let token_a = TokenIdentifier::from_esdt_bytes(first_token);
        let token_b = TokenIdentifier::from_esdt_bytes(second_token);
        let total_fee_percent = 300u64;
        let special_fee_percent = 50u64;
        let admins = MultiValueEncoded::new();
        let optional_fees =
            OptionalValue::Some(MultiValue2::from((total_fee_percent, special_fee_percent)));

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_router_address())
            .gas(120_000_000u64)
            .typed(router_proxy::RouterProxy)
            .create_pair_endpoint(
                token_a,
                token_b,
                &self.wallet_address,
                optional_fees,
                admins,
            )
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_pair_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");
    }

    pub async fn issue_lp_token_and_set_roles(&mut self) -> TokenIdentifier<StaticApi> {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_router_address())
            .gas(120_000_000u64)
            .typed(router_proxy::RouterProxy)
            .issue_lp_token(self.state.current_pair_address(), "RIDEWEGLD", "RDWGLD")
            .egld(issue_cost)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_router_address())
            .gas(120_000_000u64)
            .typed(router_proxy::RouterProxy)
            .set_local_roles(self.state.current_pair_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        self.interactor
            .query()
            .to(self.state.current_pair_address())
            .typed(pair_proxy::PairProxy)
            .get_lp_token_identifier()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn add_initial_liquidity_pair(
        &mut self,
        first_token: String,
        amount_first: u64,
        second_token: String,
        amount_second: u64,
    ) {
        let mut payments = ManagedVec::new();
        payments.push(EsdtTokenPayment::new(
            TokenIdentifier::from_esdt_bytes(first_token),
            0,
            BigUint::from(amount_first),
        ));
        payments.push(EsdtTokenPayment::new(
            TokenIdentifier::from_esdt_bytes(second_token),
            0,
            BigUint::from(amount_second),
        ));
        let payment = EgldOrMultiEsdtPayment::MultiEsdt(payments);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_pair_address())
            .gas(30_000_000u64)
            .typed(pair_proxy::PairProxy)
            .add_initial_liquidity()
            .egld_or_multi_esdt(payment)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn deploy_locked_asset_factory(&mut self) {
        let code_path = MxscPath::new(&self.locked_asset_factory_contract_code);
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(90_000_000u64)
            .typed(locked_asset_factory_proxy::LockedAssetFactoryProxy)
            .init()
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_locked_asset_factory_address(Bech32Address::from_bech32_string(
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
        let old_locked_address = self.state.current_locked_asset_factory_address();
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

    pub async fn set_transfer_role_locked_token(&mut self) {
        let opt_address: OptionalValue<[u8; 32]> = OptionalValue::None;
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_energy_factory_address())
            .gas(90_000_000u64)
            .typed(energy_factory_proxy::SimpleLockEnergyProxy)
            .set_transfer_role(opt_address)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    // pub async fn set_local_roles_energy(&mut self) {
    //     let base_token = self
    //         .interactor
    //         .query()
    //         .to(self.state.current_energy_factory_address())
    //         .typed(energy_factory_proxy::SimpleLockEnergyProxy)
    //         .base_asset_token_id()
    //         .returns(ReturnsResultUnmanaged)
    //         .run()
    //         .await;

    //     let roles = vec![EsdtLocalRole::Burn, EsdtLocalRole::Mint];

    //     self.interactor
    //         .tx()
    //         .from(self.state.current_energy_factory_address())
    //         .to(ESDTSystemSCAddress)
    //         .gas(100_000_000u64)
    //         .typed(ESDTSystemSCProxy)
    //         .set_special_roles(
    //             self.state.current_energy_factory_address().clone(),
    //             base_token,
    //             roles.into_iter(),
    //         )
    //         .returns(ReturnsResultUnmanaged)
    //         .run()
    //         .await;
    // }

    pub async fn deploy_farm(&mut self, farming_token_id: String) {
        let code_path = MxscPath::new(&self.farm_contract_code);
        let admins = MultiValueEncoded::new();
        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(160_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .init(
                TokenIdentifier::from("MEX-123456"),
                TokenIdentifier::from_esdt_bytes(farming_token_id),
                DIVISION_SAFETY_CONSTANT,
                self.state.current_pair_address(),
                &self.wallet_address,
                admins,
            )
            .code(code_path)
            .returns(ReturnsNewAddress)
            .run()
            .await;
        let new_address_bech32 = bech32::encode(&new_address);
        self.state
            .set_farm_address(Bech32Address::from_bech32_string(
                new_address_bech32.clone(),
            ));

        println!("new address: {new_address_bech32}");

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .resume()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn register_farm_token(&mut self) -> TokenIdentifier<StaticApi> {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(90_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .register_farm_token("RIDEWEGLDFarm", "RDWGLDFL", 18u8)
            .egld(issue_cost)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        self.interactor
            .query()
            .to(self.state.current_farm_address())
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .farm_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn whitelist_farm_in_pool(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_pair_address())
            .gas(30_000_000u64)
            .typed(pair_proxy::PairProxy)
            .whitelist_endpoint(self.state.current_farm_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_energy_address_in_farm(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_energy_factory_address(self.state.current_energy_factory_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_locking_contract_in_farm(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_locking_sc_address(self.state.current_energy_factory_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn whitelist_farm_in_energy_factory(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_energy_factory_address())
            .gas(30_000_000u64)
            .typed(energy_factory_proxy::SimpleLockEnergyProxy)
            .add_sc_address_to_whitelist(self.state.current_farm_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_lock_epochs(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_lock_epochs(LOCK_EPOCHS)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_boosted_rewards_farm(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_boosted_yields_rewards_percentage(BOOSTED_YIELDS_PERCENTAGE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_boosted_yields_factor_farm(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
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

    pub async fn set_rewards_per_block_farm(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_per_block_rewards_endpoint(BigUint::from(REWARDS_PER_BLOCK_FARM))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_penalty_farm(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_penalty_percent(PENALTY_PERCENTAGE)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn set_min_farming_epochs_farm(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .set_minimum_farming_epochs(MIN_FARMING_EPOCHS)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn get_energy_address(&mut self) {
        let response = self
            .interactor
            .query()
            .to(self.state.current_farm_address())
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .energy_factory_address()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        assert_eq!(
            response,
            self.state.current_energy_factory_address().to_address()
        );
    }

    pub async fn get_storage_key(&mut self) {
        let x = self
            .interactor
            .proxy
            .get_account_storage_keys(&self.state.current_energy_factory_address().to_address())
            .await;

        println!("{:?}", x);
    }

    pub async fn get_locked_token_id(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .get_locked_token_id()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
        println!("LOCKED TOKEN ID: {:?}", response);
    }

    pub async fn deploy_farm_staking(&mut self, staking_token: String) {
        let code_path = MxscPath::new(&self.farm_staking_contract_code);
        let farming_token_id = TokenIdentifier::from_esdt_bytes(staking_token);
        let admins = MultiValueEncoded::new();

        let new_address = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .gas(140_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .init(
                farming_token_id,
                DIVISION_SAFETY_CONSTANT,
                2500u64,
                10u64,
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

    pub async fn register_farm_token_staking(&mut self) -> TokenIdentifier<StaticApi> {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(90_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .register_farm_token("SRIDE", "FSRD", 18u8)
            .egld(issue_cost)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        self.interactor
            .query()
            .to(self.state.current_farm_staking_address())
            .typed(farm_staking_proxy::FarmStakingProxy)
            .farm_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn set_rewards_per_block_farm_staking(&mut self) {
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

    pub async fn set_boosted_yields_rewards_percentage_farm_staking(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .set_boosted_yields_rewards_percentage(BOOSTED_YIELDS_PERCENTAGE_STAKING)
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

    pub async fn set_energy_address_in_farm_staking(&mut self) {
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

    pub async fn topup_rewards(&mut self, token_id: String, amount: u64) {
        let amount = BigUint::from(amount);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(90_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .top_up_rewards()
            .esdt(EsdtTokenPayment::new(
                TokenIdentifier::from_esdt_bytes(token_id),
                0,
                amount,
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
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

    pub async fn deploy_metastaking(
        &mut self,
        lp_token: String,
        lp_farm: String,
        staking_token: String,
        staking_farm: String,
    ) {
        let code_path = MxscPath::new(&self.contract_code);
        let energy_factory_address = self.state.current_energy_factory_address();
        let lp_farm_address = self.state.current_farm_address();
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

    pub async fn register_metastaking_token(&mut self) -> TokenIdentifier<StaticApi> {
        let issue_cost = BigUint::<StaticApi>::from(50000000000000000u64);
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(90_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .register_dual_yield_token("METARIDE", "MRDE", 18u8)
            .egld(issue_cost)
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;

        self.interactor
            .query()
            .to(self.state.current_address())
            .typed(proxy::FarmStakingProxyProxy)
            .dual_yield_token()
            .returns(ReturnsResultUnmanaged)
            .run()
            .await
    }

    pub async fn whitelist_metastaking_in_pair(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_pair_address())
            .gas(30_000_000u64)
            .typed(pair_proxy::PairProxy)
            .whitelist_endpoint(self.state.current_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn whitelist_metastaking_in_farm(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .add_sc_address_to_whitelist(self.state.current_address())
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn whitelist_metastaking_in_farm_staking(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_staking_address())
            .gas(30_000_000u64)
            .typed(farm_staking_proxy::FarmStakingProxy)
            .add_sc_address_to_whitelist(self.state.current_address())
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
            )
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

    pub async fn enter_farm_endpoint(&mut self, lp_token: String, amount: u64) {
        let opt_orig_caller: OptionalValue<[u8; 32]> = OptionalValue::None;
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_farm_address())
            .gas(30_000_000u64)
            .typed(farm_with_locked_rewards_proxy::FarmProxy)
            .enter_farm_endpoint(opt_orig_caller)
            .esdt(EsdtTokenPayment::new(
                TokenIdentifier::from_esdt_bytes(lp_token),
                0,
                BigUint::from(amount),
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
    }

    pub async fn setup_tests(&mut self) {
        let WEGLD = self
            .issue_fungible_token(
                self.wallet_address.clone(),
                "WrappedEGLD".as_bytes(),
                "WEGLD".as_bytes(),
                RustBigUint::from(10000u64),
                18,
            )
            .await;
        let RIDE = self
            .issue_fungible_token(
                self.wallet_address.clone(),
                "RideToken".as_bytes(),
                "RIDE".as_bytes(),
                RustBigUint::from(100000u64),
                18,
            )
            .await;

        let pair_template_address = self.deploy_pair_template(WEGLD.clone(), RIDE.clone()).await;
        self.deploy_router(pair_template_address).await;
        self.deploy_pair(WEGLD.clone(), RIDE.clone()).await;
        let lp_token = self.issue_lp_token_and_set_roles().await;
        self.add_initial_liquidity_pair(WEGLD, 5000, RIDE.clone(), 5000)
            .await;

        self.deploy_locked_asset_factory().await;
        self.deploy_energy_factory().await;
        self.issue_locked_token().await;
        //self.set_local_roles_energy().await;
        self.set_transfer_role_locked_token().await;

        self.deploy_farm(lp_token.to_string()).await;
        let lp_farm = self.register_farm_token().await;
        self.whitelist_farm_in_pool().await;
        self.set_energy_address_in_farm().await;
        self.set_locking_contract_in_farm().await;
        self.whitelist_farm_in_energy_factory().await;
        self.set_lock_epochs().await;
        self.set_boosted_rewards_farm().await;
        self.set_boosted_yields_factor_farm().await;
        self.set_rewards_per_block_farm().await;
        self.set_penalty_farm().await;
        self.set_min_farming_epochs_farm().await;

        self.deploy_farm_staking(RIDE.clone()).await;
        let staking_farm = self.register_farm_token_staking().await;
        self.set_rewards_per_block_farm_staking().await;
        self.set_boosted_yields_rewards_percentage_farm_staking()
            .await;
        self.set_boosted_yields_factors_farm_staking().await;
        self.set_energy_address_in_farm_staking().await;
        self.topup_rewards(RIDE.clone(), 50_000u64).await;

        self.deploy_metastaking(
            lp_token.to_string(),
            lp_farm.to_string(),
            RIDE.clone(),
            staking_farm.to_string(),
        )
        .await;
        self.register_metastaking_token().await;
        self.whitelist_metastaking_in_pair().await;
        self.whitelist_metastaking_in_farm().await;
        self.whitelist_metastaking_in_farm_staking().await;

        self.get_energy_address().await;
        self.get_storage_key().await;
        self.get_locked_token_id().await;
        self.enter_farm_endpoint(lp_token.to_string(), 1000).await;
    }
}
