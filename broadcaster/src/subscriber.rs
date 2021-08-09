#![allow(dead_code)]

use crate::{broadcaster::*, utils::get_program_accounts};
use randomness_oracle_program::state::{AccountType, RandomnessOracle};
use solana_client::{pubsub_client::PubsubClient, rpc_client::RpcClient};
use solana_program::{clock::Slot, program_pack::Pack, pubkey::Pubkey};
use solana_sdk::signer::Signer;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

pub struct Subscriber {
    websocket_url: String,
}

impl Subscriber {
    pub fn new(websocket_url: String) -> Self {
        Self { websocket_url }
    }

    pub async fn run(&self, broadcaster: &Broadcaster) -> anyhow::Result<()> {
        let exit = Arc::new(AtomicBool::new(false));
        let mut current_slot: Option<Slot> = None;
        let rpc_client = RpcClient::new(broadcaster.rpc_url.clone());

        let (mut client, receiver) = PubsubClient::slot_subscribe(&self.websocket_url).unwrap();

        loop {
            if exit.load(Ordering::Relaxed) {
                eprintln!("exit");
                client.shutdown().unwrap();
                break;
            }

            match receiver.recv() {
                Ok(new_info) => {
                    let current_root = new_info.root;

                    match current_slot {
                        Some(value) => {
                            if current_root == value {
                                continue;
                            }

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

                            println!(
                                "Latest stable block: {}, Pending block: {}, Total accounts: {}",
                                current_root,
                                current_root - value,
                                accounts.len(),
                            );

                            let _signatures = broadcaster.broadcast(accounts).await;
                            // println!("{:?}", signatures);

                            current_slot = Some(current_root);
                        }
                        _ => current_slot = Some(current_root),
                    };
                }
                Err(err) => {
                    eprintln!("disconnected: {}", err);
                    break;
                }
            }
        }

        Ok(())
    }
}
