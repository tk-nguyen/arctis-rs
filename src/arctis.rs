use hidapi::{DeviceInfo, HidApi};
use serde_json::json;
use std::{collections::HashMap, thread::sleep, time::Duration};

pub fn get_battery(api: HidApi) {
    // Filter out Arctis devices
    let arctis: Vec<&DeviceInfo> = api
        .device_list()
        .filter(|&d| match d.product_string() {
            Some(s) => s.to_lowercase().contains("steelseries"),
            None => false,
        })
        .collect();

    let mut devices_battery: HashMap<String, u8> = HashMap::new();
    // Bytes to poll devices battery
    // Found here: https://github.com/SteelSeries/gamesense-sdk/issues/74#issuecomment-530909243
    let payload = [0x06, 0x18];

    // Since there are 3 devices that Arctis presents to our PC, we will send the payload to all of them.
    // Incorrect device will report "Incorrect function."
    // Correct device will give us number of bytes written
    println!("Querying battery level from your devices...");
    for device in arctis {
        let handle = device.open_device(&api).unwrap();
        let mut buf = [0u8; 8];
        match handle.write(&payload) {
            Ok(_b) => {
                handle.read(&mut buf).unwrap();
                devices_battery.insert(
                    handle.get_product_string().unwrap().unwrap(),
                    // Battery level should be in the third byte
                    buf[2],
                );
            }
            Err(_e) => continue,
        };
    }
    for (key, val) in devices_battery {
        println!("Your {} battery is at {}%", key, val)
    }
    let duration = Duration::from_secs(5);
    println!("Exiting in {:#?}...", duration);
    sleep(duration);
}

pub fn get_devices_list(api: HidApi) {
    // Filter out Arctis devices
    let arctis: Vec<&DeviceInfo> = api
        .device_list()
        .filter(|&d| match d.product_string() {
            Some(s) => s.to_lowercase().contains("steelseries"),
            None => false,
        })
        .collect();

    for device in arctis {
        let output = json!({
            "Manufacturer": device.manufacturer_string().unwrap(),
            "Product": device.product_string().unwrap(),
            "Path": device.path().to_str().unwrap(),
        });
        println!("{}", output);
    }
}
