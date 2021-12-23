use hidapi::{DeviceInfo, HidApi};
use serde_json::json;
use std::collections::HashMap;

// Device with `usage_page == 65347` and `usage == 514` should be the query device
// Taken from here: https://github.com/SteelSeries/gamesense-sdk/issues/74#issuecomment-530210900
const ARCTIS_USAGE_PAGE: u16 = 65347;
const ARCTIS_USAGE: u16 = 514;

// Bytes to poll devices battery
// Taken from here: https://github.com/SteelSeries/gamesense-sdk/issues/74#issuecomment-530909243
const REPORT_PAYLOAD: [u8; 2] = [0x06, 0x18];

pub fn get_battery(api: &HidApi) {
    let arctis: Vec<&DeviceInfo> = api
        .device_list()
        // Filter out Arctis devices
        .filter(|&d| match d.product_string() {
            Some(s) => s.to_lowercase().contains("steelseries"),
            None => false,
        })
        .filter(|&d| d.usage_page() == ARCTIS_USAGE_PAGE && d.usage() == ARCTIS_USAGE)
        .collect();

    // We store all Arctis device in a hashmap
    // Name of the device is key, battery level is value
    let mut devices_battery: HashMap<String, u8> = HashMap::new();

    let payload = REPORT_PAYLOAD;

    if arctis.len() == 0 {
        eprintln!("No Arctis device found!")
    } else {
        println!("Querying battery level from your devices...");
        for device in arctis {
            let handle = device.open_device(api).unwrap();
            let mut buf = [0u8; 8];

            // If the write succeeds, it returns number of bytes written
            match handle.write(&payload) {
                Ok(_b) => {
                    // The device should return report id and battery level when read
                    handle.read(&mut buf).unwrap();
                    devices_battery.insert(
                        handle.get_product_string().unwrap().unwrap(),
                        // Battery level should be in the third byte
                        buf[2],
                    );
                }
                Err(_e) => eprintln!("Can't talk to your Arctis devices!"),
            }
        }
    }
    for (key, val) in devices_battery {
        match val {
            0 => println!(
                "Your {} battery is at {}% (Did you turn on the headset?)",
                key, val
            ),
            97 => println!("Your {} battery is at {}% (You might need to disconnect and reconnect the cable if you're charging).", key, val),
            _ => println!("Your {} battery is at {}%", key, val),
        }
    }
    ()
}

pub fn get_devices_list(api: &HidApi) {
    // Filter out Arctis devices
    let arctis: Vec<&DeviceInfo> = api
        .device_list()
        .filter(|&d| match d.product_string() {
            Some(s) => s.to_lowercase().contains("steelseries"),
            None => false,
        })
        .filter(|&d| d.usage_page() == ARCTIS_USAGE_PAGE && d.usage() == ARCTIS_USAGE)
        .collect();

    for device in arctis {
        let output = json!({
            "Manufacturer": device.manufacturer_string().unwrap().trim(),
            "Product": device.product_string().unwrap().trim(),
            "Path": device.path().to_str().unwrap().trim(),
        });
        println!("{:#}", output);
    }
    ()
}
