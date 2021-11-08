#![allow(dead_code)]

use crate::{broadcaster::*, utils::get_program_accounts};
use randomness_oracle_program::state::{AccountType, RandomnessOracle};
use solana_client::rpc_client::RpcClient;
use solana_program::{program_pack::Pack, pubkey::Pubkey};
use solana_sdk::signer::Signer;
use std::time;

pub const SLEEP_TIME: time::Duration = time::Duration::from_secs(5);

pub struct Subscriber {
    rpc_url: String,
}

impl Subscriber {
    pub fn new(rpc_url: String) -> Self {
        Self { rpc_url }
    }

    pub async fn run(&self, broadcaster: &Broadcaster) -> anyhow::Result<()> {
        let rpc_client = RpcClient::new(broadcaster.rpc_url.clone());

        let accounts: Vec<(Pubkey, RandomnessOracle)> = get_program_accounts(
            &rpc_client,
            AccountType::RandomnessOracle,
            &broadcaster.authority.pubkey(),
        )?
        .into_iter()
        .filter_map(|(address, account)| {
            match RandomnessOracle::unpack_unchecked(&account.data) {
                Ok(pool) => Some((address, pool)),
                _ => None,
            }
        })
        .collect();

        let signatures = broadcaster.broadcast(accounts).await.ok();

        if let Some(signatures) = signatures {
            println!("{:?}", signatures);
        } else {
            eprintln!("Broadcaster was disconnected while was sending transaction");
        }

        Ok(())
    }
}
