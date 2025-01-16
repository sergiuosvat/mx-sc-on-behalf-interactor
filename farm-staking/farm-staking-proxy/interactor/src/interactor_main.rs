
use multiversx_sc_snippets::imports::*;
use farm_staking_proxy_interactor::farm_staking_proxy_cli;

#[tokio::main]
async fn main() {
    farm_staking_proxy_cli().await;
}  

