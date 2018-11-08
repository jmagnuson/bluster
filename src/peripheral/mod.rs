#[cfg(any(target_os = "macos", target_os = "ios"))]
mod corebluetooth;
#[cfg(any(target_os = "macos", target_os = "ios"))]
pub use corebluetooth::Peripheral;

#[cfg(any(target_os = "linux", target_os = "android"))]
mod bluez;
#[cfg(any(target_os = "linux", target_os = "android"))]
pub use bluez::Peripheral;

#[cfg(any(target_os = "windows", target_os = "freebsd"))]
mod usb;
#[cfg(any(target_os = "windows", target_os = "freebsd"))]
pub use usb::Peripheral;

// TODO: Add struct / traits to implement for each OS
//
// pub enum BindingsEvent {
//     StateChange,
//     Platform,
//     AddressChange,
//     AdvertisingStart,
//     AdvertisingStop,
//     ServicesSet,
//     Accept,
//     MtuChange,
//     Disconnect,
//     RssiUpdate,
// }
//
// #[derive(Debug, Clone)]
// pub enum State {
//     Unknown,
//     Resetting,
//     Unsupported,
//     Unauthorized,
//     PoweredOff,
//     PoweredOn,
// }
//
// #[derive(Debug, Clone)]
// pub struct Ble {
//     initialized: bool,
//     platform: String, // TODO: Make this an enum?
//     state: State,
//     address: String, // TODO: Make this a struct or something?
//     rssi: u8,
//     mtu: u8,
// }

#[test]
fn it_connects_changes_state() {
    use std::{thread, time};

    let mut peripheral = Peripheral::new();

    thread::sleep(time::Duration::from_secs(1));

    if peripheral.is_powered_on() {
        println!("It's powered on!");
    } else {
        println!("It's powered off!");
    }

    peripheral.start_advertising("hello", &[]);
    for _ in 0..60 {
        println!("Advertising {:?}", peripheral.is_advertising());
        thread::sleep(time::Duration::from_secs(1));
    }

    peripheral.stop_advertising();

    for _ in 0..10 {
        println!("Advertising {:?}", peripheral.is_advertising());
        thread::sleep(time::Duration::from_secs(1));
    }
}