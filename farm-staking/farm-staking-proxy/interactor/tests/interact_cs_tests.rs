use farm_staking_proxy_interactor::ContractInteract;
use multiversx_sc_snippets::imports::*;

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.
const BOOSTED_YIELDS_PERCENTAGE: u64 = 2500;
#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_deploy() {
    let mut interactor = ContractInteract::new().await;
    interactor.setup_tests().await;
    interactor
        .set_boosted_yields_rewards_percentage_lp(BOOSTED_YIELDS_PERCENTAGE)
        .await;
    interactor
        .set_boosted_yields_rewards_percentage(BOOSTED_YIELDS_PERCENTAGE)
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_stake_farm_not_whitelisted() {
    let mut interactor = ContractInteract::new().await;
    let lp_token = interactor.setup_tests().await;
    interactor
        .stake_farm_on_behalf(
            interactor.bob_address.clone(),
            interactor.wallet_address.clone(),
            lp_token,
            0,
            BigUint::from(1u64),
            Some("Caller is not whitelisted by the user or is blacklisted"),
        )
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn stake_farm_blacklisted() {
    let mut interactor = ContractInteract::new().await;
    let lp_token = interactor.setup_tests().await;
    interactor
        .blacklist_address(interactor.bob_address.clone())
        .await;
    interactor
        .stake_farm_on_behalf(
            interactor.bob_address.clone(),
            interactor.wallet_address.clone(),
            lp_token,
            0,
            BigUint::from(1u64),
            Some("Caller is not whitelisted by the user or is blacklisted"),
        )
        .await;
}

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_stake_farm() {
    let farm_amount = 100_000_000u64;
    let mut interactor = ContractInteract::new().await;
    let lp_token = interactor.setup_tests().await;
    let bob_address = interactor.bob_address.clone();
    interactor.whitelist_address(bob_address.clone()).await;
    interactor
        .set_boosted_yields_rewards_percentage(BOOSTED_YIELDS_PERCENTAGE)
        .await;
    interactor
        .set_boosted_yields_rewards_percentage_lp(BOOSTED_YIELDS_PERCENTAGE)
        .await;
    let x = interactor.get_locked_token_id_wanted().await;
    println!("Locked token id: {:?}", x);
    interactor.get_storage_keys_energy_factory().await;
    interactor.get_energy_factory_address_lp().await;
    // interactor
    //     .enter_farm_endpoint(lp_token, farm_amount * 2)
    //     .await;
}
