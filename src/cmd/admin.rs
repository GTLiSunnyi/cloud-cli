use clap::App;
use clap::Arg;

use crate::utils::{parse_addr, parse_data};

use super::*;
use crate::sdk::context::Context;
use prost::Message;

use crate::crypto::{ArrayLike, Crypto};
use crate::utils::hex;

pub fn update_admin<'help, C, Co, Ex, Ev, Wa>() -> Command<'help, Co, Ex, Ev, Wa>
where
    C: Crypto,
    Context<Co, Ex, Ev, Wa>: AdminBehaviour<C>,
{
    Command::new("update-admin")
        .about("Update admin of the chain")
        .arg(
            Arg::new("admin")
                .help("the address of the new admin")
                .required(true)
                .validator(parse_addr::<C>),
        )
        .handler(|ctx, m| {
            let admin = parse_addr::<C>(m.value_of("admin").unwrap())?;
            let tx_hash = ctx.rt.block_on(ctx.update_admin(admin))?;
            println!("tx_hash: {}", hex(tx_hash.as_slice()));
            Ok(())
        })
}

pub fn update_validators<'help, C, Co, Ex, Ev, Wa>() -> Command<'help, Co, Ex, Ev, Wa>
where
    C: Crypto,
    Context<Co, Ex, Ev, Wa>: AdminBehaviour<C>,
{
    Command::new("update-validators")
        .about("Update validators of the chain")
        .arg(
            Arg::new("validators")
                .help("a space-separated list of the new validator addresses, e.g. `cldi update-validators 0x12..34 0xab..cd`")
                .required(true)
                .multiple_values(true)
                .validator(parse_addr::<C>)
        )
        .handler(|ctx, m| {
            let validators = m
                .values_of("validators")
                .unwrap()
                .map(parse_addr::<C>)
                .collect::<Result<Vec<C::Address>>>()?;
            let tx_hash = ctx.rt.block_on(ctx.update_validators(&validators))?;
            println!("tx_hash: {}", hex(&tx_hash.as_slice()));
            Ok(())
        })
}

pub fn set_block_interval<'help, C, Co, Ex, Ev, Wa>() -> Command<'help, Co, Ex, Ev, Wa>
where
    C: Crypto,
    Context<Co, Ex, Ev, Wa>: AdminBehaviour<C>,
{
    Command::new("set-block-interval")
        .about("Set block interval")
        .arg(
            Arg::new("block_interval")
                .help("new block interval")
                .required(true)
                .validator(str::parse::<u32>),
        )
        .handler(|ctx, m| {
            let block_interval = m.value_of("block_interval").unwrap().parse::<u32>()?;
            let tx_hash = ctx.rt.block_on(ctx.set_block_interval(block_interval))?;
            println!("tx_hash: {}", hex(tx_hash.as_slice()));
            Ok(())
        })
}

pub fn emergency_brake<'help, C, Co, Ex, Ev, Wa>() -> Command<'help, Co, Ex, Ev, Wa>
where
    C: Crypto,
    Context<Co, Ex, Ev, Wa>: AdminBehaviour<C>,
{
    Command::new("emergency-brake")
        .about("Send emergency brake cmd to chain")
        .arg(
            Arg::new("switch")
                .help("turn on/off")
                .required(true)
                .possible_values(&["on", "off"]),
        )
        .handler(|ctx, m| {
            let switch = m.value_of("switch").unwrap() == "on";
            let tx_hash = ctx.rt.block_on(ctx.emergency_brake(switch))?;
            println!("tx_hash: {}", hex(tx_hash.as_slice()));
            Ok(())
        })
}

pub fn admin_cmd<'help, C, Co, Ex, Ev, Wa>() -> Command<'help, Co, Ex, Ev, Wa>
where
    C: Crypto,
    Context<Co, Ex, Ev, Wa>: AdminBehaviour<C>,
{
    Command::new("admin")
        .about("The admin commands for chain")
        .setting(AppSettings::SubcommandRequired)
        .subcommands([
            update_admin(),
            update_validators(),
            set_block_interval(),
            emergency_brake(),
        ])
}
