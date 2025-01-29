// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           19
// Async Callback (empty):               1
// Total number of exported functions:  22

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    factory_legacy
    (
        init => init
        upgrade => upgrade
        whitelist => whitelist
        removeWhitelist => remove_whitelist
        createAndForwardCustomPeriod => create_and_forward_custom_period
        createAndForward => create_and_forward
        unlockAssets => unlock_assets
        setBurnRoleForAddress => set_burn_role_for_address
        getLastErrorMessage => last_error_message
        getInitEpoch => init_epoch
        getWhitelistedContracts => get_whitelisted_contracts
        getDefaultUnlockPeriod => default_unlock_period
        getLockedAssetTokenId => locked_asset_token_id
        getAssetTokenId => asset_token_id
        getUnlockScheduleForSFTNonce => get_unlock_schedule_for_sft_nonce
        getCacheSize => get_cache_size
        getExtendedAttributesActivationNonce => extended_attributes_activation_nonce
        setNewFactoryAddress => set_new_factory_address
        pause => pause_endpoint
        unpause => unpause_endpoint
        isPaused => paused_status
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
