use std::{ffi::CString, path::PathBuf};

use anyhow::{anyhow, Ok};
use ts3plugin::TsApi;

#[derive(Clone, Debug)]
pub struct Sound {
    file_name: String,
    is_loop: bool,
    handle: String,
    wave_handle: u64,
}

pub fn play_sound(
    sound: &mut Sound,
    api: &mut TsApi,
    sound_pack_name: &str,
    server_id: u64,
) -> Result<(), anyhow::Error> {
    sound.wave_handle = 0;
    let plugin_path = PathBuf::from(api.get_plugin_path());

    let paths_to_check = [
        plugin_path.join("override").join(&sound.file_name),
        plugin_path.join(sound_pack_name).join(&sound.file_name),
        plugin_path.join("default").join(&sound.file_name),
    ];

    let path = paths_to_check
        .iter()
        .find(|&path| path.exists())
        .ok_or_else(|| anyhow!("Sound file not found: {}", sound.file_name))?;

    let c_str = CString::new(
        path.to_str()
            .ok_or(anyhow!("Could not convert to cstring"))?,
    )?;
    let _num = unsafe {
        (api.get_raw_api().play_wave_file_handle)(
            server_id,
            c_str.as_ptr(),
            sound.is_loop as i32,
            &mut sound.wave_handle,
        )
    };

    Ok(())
}

pub fn stop_playing(sound: &Sound, api: &mut TsApi, server_id: u64) {
    let _num = unsafe { (api.get_raw_api().close_wave_file_handle)(server_id, sound.wave_handle) };
}
