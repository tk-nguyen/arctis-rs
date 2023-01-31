use arctis::*;
use clap::{ArgGroup, Parser};
use color_eyre::eyre::{eyre, Result, WrapErr};
use hidapi::HidApi;

mod arctis;

#[derive(Parser)]
#[command(
    name = "arctis-rs",
    author = clap::crate_authors!(),
    version,
    about = "A small utility to query Arctis battery"
)]
#[command(group = ArgGroup::new("query").args(["battery", "list"]).multiple(true).required(true))]
struct ArctisOpt {
    /// Query the battery level
    #[arg(short, long)]
    battery: bool,

    /// Get the device list
    #[arg(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let options = ArctisOpt::parse();
    // Create a new API to query all available devices
    let api = HidApi::new().wrap_err("Can't connect to your Steelseries devices.")?;
    match options {
        ArctisOpt {
            battery: true,
            list: true,
        } => {
            get_devices_list(&api)?;
            get_battery(api)?;
        }
        ArctisOpt {
            battery: false,
            list: true,
        } => get_devices_list(&api)?,
        ArctisOpt {
            battery: true,
            list: false,
        } => get_battery(api)?,
        _ => Err(eyre!("An error occured when running the program!"))?, // This should not be reachable!
    }
    Ok(())
}
