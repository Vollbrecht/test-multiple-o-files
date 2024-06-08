use esp_idf_sys::*;

pub fn my_legacy_driver() -> Result<(), EspError> {
    // legacy rmt config
    let legacy_config = rmt_config_t {
        ..Default::default()
    };

    unsafe {
        esp!(esp_idf_sys::rmt_config(&legacy_config))?;
    }
    Ok(())
}

pub fn my_new_driver() -> Result<(), EspError> {
    let mut handle: *mut rmt_channel_handle_t = core::ptr::null_mut();

    let h2: *mut *mut rmt_channel_handle_t = &mut handle;

    let new_conf = esp_idf_sys::rmt_tx_channel_config_t {
        clk_src: esp_idf_sys::soc_periph_rmt_clk_src_t_RMT_CLK_SRC_DEFAULT as _, // select clock source
        gpio_num: 4 as _,
        mem_block_symbols: 64 as _,
        resolution_hz: 1_000_000 as _,
        trans_queue_depth: 10 as _, // set the number of transactions that can be pending in the background
        ..Default::default()
    };

    unsafe { esp! {esp_idf_sys::rmt_new_tx_channel(&new_conf, h2.cast()) }? };

    Ok(())
}
