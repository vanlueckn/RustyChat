use std::path::PathBuf;

use anyhow::anyhow;
use ts3plugin::{ServerId, TsApi};

#[derive(Clone, Debug)]
pub struct Sound {
    pub file_name: String,
    pub is_loop: bool,
    pub handle: String,
    pub wave_handle: u64,
}

pub fn play_sound(
    sound: &mut Sound,
    sound_pack_name: &str,
    server_id: u64,
) -> anyhow::Result<(), anyhow::Error> {
    sound.wave_handle = 0;
    let api = TsApi::lock_api().ok_or(anyhow!("Could not lock api"))?;
    let plugin_path = PathBuf::from(api.get_plugin_path());

    let paths_to_check = [
        plugin_path.join("override").join(&sound.file_name),
        plugin_path.join(sound_pack_name).join(&sound.file_name),
        plugin_path.join("default").join(&sound.file_name),
    ];

    let path = paths_to_check
        .iter()
        .find(|&path| path.exists())
        .ok_or_else(|| anyhow::anyhow!("Sound file not found: {}", sound.file_name))?;

    let server = api
        .get_server(ServerId(server_id))
        .ok_or(anyhow!("Could not get server"))?;
    let path = path
        .to_str()
        .ok_or(anyhow::anyhow!("Could not convert to cstring"))?;

    let _res = server.play_wave_file_handle(path, sound.is_loop, &mut sound.wave_handle);

    anyhow::Ok(())
}

pub fn stop_playing(sound: &Sound, api: &mut TsApi, server_id: u64) {
    let _num = unsafe { (api.get_raw_api().close_wave_file_handle)(server_id, sound.wave_handle) };
}
