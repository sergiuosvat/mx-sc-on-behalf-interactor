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
const PERMISSION_HUB_CONTRACT_CODE: &str =
    "../../dex/permissions-hub/output/permissions-hub.mxsc.json";

pub async fn farm_staking_proxy_cli() {
    //     env_logger::init();

    //     // let mut args = std::env::args();
    //     // let _ = args.next();
    //     // let cmd = args.next().expect("at least one argument required");
    //     // let mut interact = ContractInteract::new().await;
    //     // match cmd.as_str() {
    //     //     "deploy" => {
    //     //         interact
    //     //             .deploy_metastaking(String::new(), String::new(), String::new(), String::new())
    //     //             .await
    //     //     } //     "upgrade" => interact.upgrade().await,
    //         //     "registerDualYieldToken" => interact.register_dual_yield_token().await,
    //         //     "getDualYieldTokenId" => interact.dual_yield_token().await,
    //         //     "getLpFarmAddress" => interact.lp_farm_address().await,
    //         //     "getStakingFarmAddress" => interact.staking_farm_address().await,
    //         //     "getPairAddress" => interact.pair_address().await,
    //         //     "getStakingTokenId" => interact.staking_token_id().await,
    //         //     "getFarmTokenId" => interact.staking_farm_token_id().await,
    //         //     "getLpTokenId" => interact.lp_token_id().await,
    //         //     "getLpFarmTokenId" => interact.lp_farm_token_id().await,
    //         //     "setPermissionsHubAddress" => interact.set_permissions_hub_address().await,
    //         //     "setEnergyFactoryAddress" => interact.set_energy_factory_address().await,
    //         //     "getEnergyFactoryAddress" => interact.energy_factory_address().await,
    //         //     "addSCAddressToWhitelist" => interact.add_sc_address_to_whitelist().await,
    //         //     "removeSCAddressFromWhitelist" => interact.remove_sc_address_from_whitelist().await,
    //         //     "isSCAddressWhitelisted" => interact.is_sc_address_whitelisted().await,
    //         //     "stakeFarmTokens" => interact.stake_farm_tokens().await,
    //         //     "claimDualYield" => interact.claim_dual_yield_endpoint().await,
    //         //     "unstakeFarmTokens" => interact.unstake_farm_tokens().await,
    //         //     "stakeFarmOnBehalf" => interact.stake_farm_on_behalf().await,
    //         //     "claimDualYieldOnBehalf" => interact.claim_dual_yield_on_behalf().await,
    //         _ => panic!("unknown command: {}", &cmd),
    //     }
}

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

pub struct ContractInteract {
    interactor: Interactor,
    pub wallet_address: Address,
    pub bob_address: Address,
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
            permission_hub_contract_code: PERMISSION_HUB_CONTRACT_CODE.to_string(),
            state: State::load_state(),
        }
    }

    pub async fn set_addresses(&mut self) {
        self.state
            .set_farm_address(Bech32Address::from_bech32_string(
                "erd1qqqqqqqqqqqqqpgq4acurmluezvmhna8tztgcrnwh0l70a2wkp2sfh6jkp".to_string(),
            ));
        self.state
            .set_pair_address_mex(Bech32Address::from_bech32_string(
                "erd1qqqqqqqqqqqqqpgqa0fsfshnff4n76jhcye6k7uvd7qacsq42jpsp6shh2".to_string(),
            ));
        self.state
            .set_pair_address_usdc(Bech32Address::from_bech32_string(
                "erd1qqqqqqqqqqqqqpgqeel2kumf0r8ffyhth7pqdujjat9nx0862jpsg2pqaq".to_string(),
            ));
        self.state
            .set_pair_address_utk(Bech32Address::from_bech32_string(
                "erd1qqqqqqqqqqqqqpgq0lzzvt2faev4upyf586tg38s84d7zsaj2jpsglugga".to_string(),
            ));
        self.state.set_address(Bech32Address::from_bech32_string(
            "erd1qqqqqqqqqqqqqpgqmgd0eu4z9kzvrputt4l4kw4fqf2wcjsekp2sftan7s".to_string(),
        ));
        self.state
            .set_energy_factory_address(Bech32Address::from_bech32_string(
                "erd1qqqqqqqqqqqqqpgq0tajepcazernwt74820t8ef7t28vjfgukp2sw239f3".to_string(),
            ));
        self.state
            .set_farm_staking_address(Bech32Address::from_bech32_string(
                "erd1qqqqqqqqqqqqqpgqcedkmj8ezme6mtautj79ngv7fez978le2jps8jtawn".to_string(),
            ));
    }

    pub async fn set_state(&mut self) {
        let response = self.interactor.set_state_for_saved_accounts().await;

        self.interactor.generate_blocks(2).await.unwrap();

        assert!(response.is_ok());
    }

    pub async fn swap_tokens_pair(&mut self) {
        let response = self
            .interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_pair_address_utk())
            .gas(30_000_000u64)
            .typed(pair_proxy::PairProxy)
            .swap_tokens_fixed_input(
                TokenIdentifier::from_esdt_bytes("UTK-2f80e9"),
                BigUint::from(1000u64),
            )
            .with_esdt_transfer(EsdtTokenPayment::new(
                TokenIdentifier::from_esdt_bytes("WEGLD-bd4d79"),
                0,
                BigUint::from(1000u64),
            ))
            .returns(ReturnsResultUnmanaged)
            .run()
            .await;
        println!("Result: {response:?}");
    }

    pub async fn debug(&mut self) {
        let response = self
            .interactor
            .proxy
            .get_account_storage_keys(&self.wallet_address)
            .await;
        println!("Result: {response:?}");
    }

    pub async fn add_liquidity(&mut self) {
        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_pair_address_utk())
            .gas(30_000_000u64)
            .typed(pair_proxy::PairProxy)
            .add_liquidity(100_000u64, 100_000u64)
            .multi_esdt(vec![
                (
                    TokenIdentifier::from_esdt_bytes("UTK-2f80e9"),
                    0,
                    BigUint::from(100_000u64),
                ),
                (
                    TokenIdentifier::from_esdt_bytes("WEGLD-bd4d79"),
                    0,
                    BigUint::from(100_000u64),
                ),
            ])
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

    pub async fn stake_farm_tokens(
        &mut self,
        token_id: TokenIdentifier<StaticApi>,
        token_nonce: u64,
        token_amount: u64,
    ) {
        let opt_orig_caller: OptionalValue<[u8; 32]> = OptionalValue::None;

        self.interactor
            .tx()
            .from(&self.wallet_address)
            .to(self.state.current_address())
            .gas(90_000_000u64)
            .typed(proxy::FarmStakingProxyProxy)
            .stake_farm_tokens(opt_orig_caller)
            .payment((token_id, token_nonce, BigUint::from(token_amount)))
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
                &BigUint::from(amount),
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

    pub async fn enter_farm_endpoint(
        &mut self,
        lp_token: String,
        amount: u64,
    ) -> (TokenIdentifier<StaticApi>, u64) {
        let opt_orig_caller: OptionalValue<[u8; 32]> = OptionalValue::None;
        let response = self
            .interactor
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

        let (base_rewards, _boosted_rewards) = response.into_tuple();
        let (token_identifier, token_nonce, _amount) = base_rewards.into_tuple();
        (token_identifier, token_nonce)
    }

    pub async fn generate_blocks(&mut self, blocks_wanted: u64) {
        _ = self.interactor.proxy.generate_blocks(blocks_wanted).await;
    }
}
