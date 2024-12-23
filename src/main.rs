use bluer::{Adapter, AdapterEvent};
use futures::StreamExt;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    // Create a session for interacting with the Bluetooth adapter
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    println!("Using Bluetooth adapter: {}", adapter.name());

    // Enable the Bluetooth adapter if it is not powered on
    if !adapter.is_powered().await? {
        println!("Enabling Bluetooth adapter...");
        adapter.set_powered(true).await?;
    }

    // Start scanning for BLE devices
    println!("Scanning for BLE devices...");
    let mut events = adapter.discover_devices().await?;
    while let Some(event) = events.next().await {
        match event {
            // Event triggered when a new device is added
            AdapterEvent::DeviceAdded(addr) => {
                println!("Device added: {:?}", addr);

                // Try to retrieve device information
                match adapter.device(addr) {
                    Ok(device) => {
                        // Retrieve the device name
                        if let Ok(Some(name)) = device.name().await {
                            println!("  Name: {}", name);
                        } else {
                            println!("  Name: Unknown");
                        }

                        // Display the device address as its unique identifier
                        println!("  Address: {:?}", device.address());

                        // Retrieve other information such as RSSI, class, or paired status
                        if let Ok(Some(rssi)) = device.rssi().await {
                            println!("  RSSI: {}", rssi);
                        }
                        if let Ok(paired) = device.is_paired().await {
                            println!("  Paired: {}", paired);
                        }
                    }
                    Err(e) => {
                        println!("Failed to get device: {}", e);
                    }
                }
            }
            // Event triggered when a device is removed
            AdapterEvent::DeviceRemoved(addr) => {
                println!("Device removed: {:?}", addr);
            }
            // Ignore other events
            _ => {}
        }
    }

    Ok(())
}
