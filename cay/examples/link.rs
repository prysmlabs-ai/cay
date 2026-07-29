//! Reports the USB link the accelerator negotiated and what it can do.
//!
//! Every byte of an inference crosses this link twice over, so the negotiated
//! speed sets the floor no amount of host-side work gets under.

fn main() {
    for device in rusb::devices().expect("devices").iter() {
        let Ok(descriptor) = device.device_descriptor() else {
            continue;
        };
        if descriptor.vendor_id() != 0x18d1 || descriptor.product_id() != 0x9302 {
            continue;
        }
        println!("speed:          {:?}", device.speed());
        println!(
            "bcdUSB:         {:#06x}",
            descriptor.usb_version().0 as u16 * 256
        );
        println!("usb_version:    {}", descriptor.usb_version());
        for n in 0..descriptor.num_configurations() {
            let Ok(config) = device.config_descriptor(n) else {
                continue;
            };
            for interface in config.interfaces() {
                for setting in interface.descriptors() {
                    for endpoint in setting.endpoint_descriptors() {
                        println!(
                            "endpoint {:#04x}: max packet {} B",
                            endpoint.address(),
                            endpoint.max_packet_size()
                        );
                    }
                }
            }
        }
    }
}
