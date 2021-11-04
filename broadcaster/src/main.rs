mod broadcaster;
mod subscriber;
mod utils;

use broadcaster::*;
use clap::{crate_description, crate_name, crate_version, App, AppSettings, Arg, SubCommand};
use solana_clap_utils::{input_validators::is_keypair, keypair::keypair_from_path};
use solana_sdk::{signature::Keypair, signer::Signer};
use std::process::exit;
use subscriber::*;
use utils::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let matches = App::new(crate_name!())
        .about(crate_description!())
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg({
            let arg = Arg::with_name("config_file")
                .short("C")
                .long("config")
                .value_name("PATH")
                .takes_value(true)
                .global(true)
                .help("Configuration file to use");

            if let Some(ref config_file) = *solana_cli_config::CONFIG_FILE {
                arg.default_value(config_file)
            } else {
                arg
            }
        })
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .takes_value(false)
                .global(true)
                .help("Show additional information"),
        )
        .arg(
            Arg::with_name("owner")
                .long("owner")
                .value_name("KEYPAIR")
                .validator(is_keypair)
                .takes_value(true)
                .global(true)
                .help(
                    "Specify the owner account. \
                     This may be a keypair file, the ASK keyword. \
                     Defaults to the client keypair.",
                ),
        )
        .subcommand(SubCommand::with_name("start").about("Start broadcaster"))
        .get_matches();

    let config = {
        let cli_config = if let Some(config_file) = matches.value_of("config_file") {
            solana_cli_config::Config::load(config_file).unwrap_or_default()
        } else {
            solana_cli_config::Config::default()
        };

        println!("{:#?}", cli_config);

        let rpc_url = cli_config.json_rpc_url.clone();

        let owner = keypair_from_path(
            &matches,
            matches
                .value_of("owner")
                .unwrap_or(&cli_config.keypair_path),
            "owner",
            false,
        )
        .unwrap_or_else(|e| {
            eprintln!("error: {}", e);
            exit(1);
        });

        let verbose = matches.is_present("verbose");

        Config {
            rpc_url,
            verbose,
            authority: owner,
        }
    };

    match matches.subcommand() {
        ("start", Some(_arg_matches)) => {
            println!("Authority: {}", config.authority.pubkey());

            let broadcaster = Broadcaster::new(
                config.rpc_url.clone(),
                Keypair::from_bytes(&config.authority.to_bytes()[..]).unwrap(),
            );

            let subscriber = Subscriber::new(config.rpc_url);
            subscriber.run(&broadcaster).await?;
        }
        _ => unreachable!(),
    }

    Ok(())
}
