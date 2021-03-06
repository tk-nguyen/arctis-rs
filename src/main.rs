use arctis::*;
use color_eyre::eyre::{Result, WrapErr};
use hidapi::HidApi;
use structopt::StructOpt;

mod arctis;

#[derive(Debug, StructOpt)]
#[structopt(name = "arctis-rs", about = "A small utility to query Arctis battery")]
struct ArctisOpt {
    /// Query the battery level
    #[structopt(short, long)]
    battery: bool,

    /// Get the device list
    #[structopt(short, long)]
    list: bool,
}

fn main() -> Result<()> {
    let options = ArctisOpt::from_args();
    // Create a new API to query all available devices
    let api = HidApi::new().wrap_err("Can't connect to the devices.")?;
    match options {
        ArctisOpt {
            battery: true,
            list: true,
        } => {
            get_devices_list(&api)?;
            get_battery(api)?;
        }
        ArctisOpt {
            battery: true,
            list: false,
        } => get_battery(api)?,
        ArctisOpt {
            battery: false,
            list: true,
        } => get_devices_list(&api)?,
        _ => ArctisOpt::clap().print_help().unwrap(),
    }
    Ok(())
}
