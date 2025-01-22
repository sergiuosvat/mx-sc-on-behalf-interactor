use farm_staking_proxy_interactor::ContractInteract;
use multiversx_sc_snippets::imports::*;

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test_deploy() {
    let mut interactor = ContractInteract::new().await;
    interactor.setup_tests().await;
}

// #[tokio::test]
// #[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
// async fn test_stake_farm_not_whitelisted() {
//     let mut interactor = ContractInteract::new().await;
//     let lp_token = interactor.setup_tests().await;
//     interactor
//         .stake_farm_on_behalf(
//             interactor.bob_address.clone(),
//             interactor.wallet_address.clone(),
//             lp_token,
//             0,
//             BigUint::from(1u64),
//             Some("Caller is not whitelisted by the user or is blacklisted"),
//         )
//         .await;
// }

// #[tokio::test]
// #[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
// async fn stake_farm_blacklisted() {
//     let mut interactor = ContractInteract::new().await;
//     let lp_token = interactor.setup_tests().await;
//     interactor
//         .blacklist_address(interactor.bob_address.clone())
//         .await;
//     interactor
//         .stake_farm_on_behalf(
//             interactor.bob_address.clone(),
//             interactor.wallet_address.clone(),
//             lp_token,
//             0,
//             BigUint::from(1u64),
//             Some("Caller is not whitelisted by the user or is blacklisted"),
//         )
//         .await;
// }
