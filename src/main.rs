use notify_rust::Notification;
use windows::Networking::Connectivity::NetworkInformation;

fn main() {
    const EXPECTED_LINK_SPEED: u64 = 1000000000;

    let network_connection =
        NetworkInformation::GetInternetConnectionProfile().expect("Failed to get network profile");

    let link_speed_inbound_bits = network_connection
        .NetworkAdapter()
        .expect("Failed to get network adapter")
        .InboundMaxBitsPerSecond()
        .expect("Failed to get inbound bits per second");

    let network_adapter_name = network_connection
        .ProfileName()
        .expect("Failed to get adapter name");

    if link_speed_inbound_bits < EXPECTED_LINK_SPEED {
        Notification::new()
            .summary("Link speed watcher")
            .body("Network link speed is less than expected.\nRestarting network adapter...")
            .show()
            .expect("Failed to show notification");

        std::process::Command::new("powershell")
            .arg(format!(
                "Restart-NetAdapter -Name '{}'",
                network_adapter_name
            ))
            .output()
            .expect("Failed to run restart network command");

    }else{
        Notification::new()
            .summary("Link speed watcher")
            .body("Network link speed is as expected.")
            .show()
            .expect("Failed to show notification");
    }
}
