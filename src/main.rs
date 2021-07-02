use std::{thread::sleep, time::Duration};

use hidapi::{DeviceInfo, HidApi, HidDevice};

fn main() {
    // Create a new API to query all available devices
    let api = HidApi::new().unwrap();

    // Filter out Arctis devices
    let arctis: Vec<&DeviceInfo> = api
        .device_list()
        .filter(|&d| match d.product_string() {
            Some(s) => s.to_lowercase().contains("steelseries"),
            None => false,
        })
        .collect();

    // Bytes to poll devices battery
    // Found here: https://github.com/SteelSeries/gamesense-sdk/issues/74#issuecomment-530909243
    let payload = vec![0x06, 0x18];

    // Since there are 3 devices that Arctis presents to our PC, we will send the payload to all of them.
    // Incorrect device will report "Incorrect function."
    // Correct device will give us number of bytes written
    let mut correct_dev: Option<HidDevice> = None;
    for device in arctis {
        let handle = device.open_device(&api).unwrap();
        match handle.write(&payload) {
            Ok(_b) => {
                println!("Querying battery level from your Arctis...");
                correct_dev = Some(handle);
            }
            Err(_e) => continue,
        };
    }

    let mut buf = [0u8; 8];
    match correct_dev {
        Some(d) => {
            d.read(&mut buf).unwrap();
            // Battery level should be at the third byte returned from the device
            let battery = buf[2];
            if battery == 0 {
                println!(
                    "Your {} battery is at {}% (Did you turn on the headset?)",
                    d.get_product_string().unwrap().unwrap(),
                    battery
                )
            } else {
                println!(
                    "Your {} battery is at {}%",
                    d.get_product_string().unwrap().unwrap(),
                    battery
                )
            }
        }
        None => eprint!("No Arctis devices found!"),
    }
    let duration = Duration::from_secs(5);
    println!("Exit in {:#?}...", duration);
    sleep(duration);
}
