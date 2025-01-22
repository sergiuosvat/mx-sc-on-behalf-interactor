// Code generated by the multiversx-sc build system. DO NOT EDIT.

////////////////////////////////////////////////////
////////////////// AUTO-GENERATED //////////////////
////////////////////////////////////////////////////

// Init:                                 1
// Upgrade:                              1
// Endpoints:                           12
// Async Callback (empty):               1
// Total number of exported functions:  15

#![no_std]

multiversx_sc_wasm_adapter::allocator!();
multiversx_sc_wasm_adapter::panic_handler!();

multiversx_sc_wasm_adapter::endpoints! {
    lkmex_transfer
    (
        init => init
        upgrade => upgrade
        withdraw => withdraw
        cancelTransfer => cancel_transfer
        lockFunds => lock_funds
        getScheduledTransfers => get_scheduled_transfers
        getAllSenders => all_senders
        setEnergyFactoryAddress => set_energy_factory_address
        getLockedTokenIdTest => get_locked_token_id
        getEnergyFactoryAddress => energy_factory_address
        addAdmin => add_admin_endpoint
        removeAdmin => remove_admin_endpoint
        updateOwnerOrAdmin => update_owner_or_admin_endpoint
        getPermissions => permissions
    )
}

multiversx_sc_wasm_adapter::async_callback_empty! {}
