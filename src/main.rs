use arctis::*;
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

fn main() {
    let options = ArctisOpt::from_args();
    // Create a new API to query all available devices
    let api = HidApi::new().unwrap();
    if options.battery {
        get_battery(api);
    } else if options.list {
        get_devices_list(api)
    } else {
        ArctisOpt::clap().print_help().unwrap();
    }
}
