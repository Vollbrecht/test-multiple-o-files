use esp_idf_sys::esp;
use esp_idf_sys::rmt_channel_handle_t;
use esp_idf_sys::rmt_config_t;
use esp_idf_sys::EspError;

use hal::my_legacy_driver;
use hal::my_new_driver;

fn main() -> Result<(), EspError> {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_sys::link_patches();

    println!("hello world");

    //my_legacy_driver()?;

    my_new_driver()?;
    Ok(())
}
