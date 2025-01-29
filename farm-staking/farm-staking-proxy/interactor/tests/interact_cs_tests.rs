use farm_staking_proxy_interactor::{Config, ContractInteract};
use multiversx_sc_snippets::{imports::*, sdk::gateway::SetStateAccount};

// Simple deploy test that runs using the chain simulator configuration.
// In order for this test to work, make sure that the `config.toml` file contains the chain simulator config (or choose it manually)
// The chain simulator should already be installed and running before attempting to run this test.
// The chain-simulator-tests feature should be present in Cargo.toml.
// Can be run with `sc-meta test -c`.

#[tokio::test]
#[cfg_attr(not(feature = "chain-simulator-tests"), ignore)]
async fn test() {
    // let mut devnet_interactor = ContractInteract::new(Config::load_config()).await;
    let mut chain_interactor = ContractInteract::new(Config::chain_simulator_config()).await;
    // let account = devnet_interactor
    //     .interactor
    //     .get_account(&devnet_interactor.wallet_address)
    //     .await;
    // let pairs = devnet_interactor
    //     .interactor
    //     .get_account_storage(&devnet_interactor.wallet_address)
    //     .await;

    // let state_account = SetStateAccount::from(account).with_storage(pairs);

    // let vec_state_account = vec![state_account];

    // let state_response = chain_interactor
    //     .interactor
    //     .set_state(vec_state_account.clone())
    //     .await;

    // chain_interactor.generate_blocks(2).await;

    // assert!(state_response.is_ok());

    // let storage = chain_interactor
    //     .interactor
    //     .get_account_storage(&chain_interactor.wallet_address)
    //     .await;

    // assert!(storage.len() > 1);

    chain_interactor.set_state().await;

    chain_interactor
        .interactor
        .tx()
        .from(chain_interactor.wallet_address) // alice
        .to(chain_interactor.bob_address) // bob
        .single_esdt(
            &TokenIdentifier::from_esdt_bytes(b"WEGLD-bd4d79"),
            0u64,
            &BigUint::from(1u64),
        )
        .run()
        .await;
}
