use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ProtocolMessage {
    command: u32,
    server_unique_identifier: String,
    parameter: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PluginStateParameter {
    version: String,
    active_instances: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct InitiateParameter {
    server_unique_identifier: String,
    name: String,
    channel_id: u64,
    channel_password: String,
    sound_pack: String,
    swiss_channel_ids: Vec<u64>,
    send_talk_states: bool,
    send_radio_traffic_states: bool,
    ultra_short_range_distance: f32,
    short_range_distance: f32,
    long_range_distance: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct InstanceStateParameter {
    is_connected_to_server: bool,
    is_ready: bool,
    state: GameInstanceState,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SoundStateParameter {
    is_microphone_muted: bool,
    is_microphone_enabled: bool,
    is_sound_muted: bool,
    is_sound_enabled: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SelfStateUpdateParameter {
    position: Vector3,
    rotation: f32,
    voice_range: f32,
    is_alive: bool,
    echo: EchoEffect,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EchoEffect {
    duration: i32,
    rolloff: f32,
    delay: int32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PlayerStateUpdateParameter {
    name: String,
    position: Vector3,
    rotation: f32,
    voice_range: f32,
    is_alive: bool,
    volume_override: f32,
    distance_culled: bool,
    muffle: MuffleEffect,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MuffleEffect {
    intensity: i32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct BulkUpdateParameter {
    player_states: Vec<PlayerStateUpdateParameter>,
    self_state: PlayerStateUpdateParameter,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RemovePlayerParameter {
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TalkStateParameter {
    name: String,
    is_talking: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PlaySoundParameter {
    file_name: String,
    is_loop: bool,
    handle: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StopSoundParameter {
    handle: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PhoneCommunicationUpdateParameter {
    name: String,
    signal_strength: i32,
    volume: f32,
    direct: bool,
    relayed_by: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StopPhoneCommunicationParameter {
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RadioCommunicationUpdateParameter {
    name: String,
    sender_radio_type: RadioType,
    own_radio_type: RadioType,
    play_mic_click: bool,
    volume: f32,
    direct: bool,
    secondary: bool,
    relayed_by: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StopRadioCommunicationParameter {
    name: String,
    play_mic_click: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RadioTowerUpdateParameter {
    towers: Vec<Tower>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Tower {
    x: f32,
    y: f32,
    z: f32,
    range: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RadioTrafficStateParameter {
    name: String,
    is_sending: bool,
    is_primary_channel: bool,
    active_relay: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AddRadioChannelMemberParameter {
    player_name: String,
    is_primary_channel: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct UpdateRadioChannelMembersParameter {
    player_names: Vec<String>,
    is_primary_channel: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct RemoveRadioChannelMemberParameter {
    player_name: String,
    is_primary_channel: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MegaphoneCommunicationUpdateParameter {
    name: String,
    range: f32,
    volume: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StopMegaphoneCommunicationParameter {
    name: String,
}

enum GameInstanceState {
    NotConnected = 0,
    Connected = 1,
    Ingame = 2,
    InSwissChannel = 3,
}

enum RadioType {
    None = 1,
    ShortRange = 2,
    LongRange = 4,
    Distributed = 8,
    UltraShortRange = 16,
}

enum Error {
    OK = 0,
    InvalidJson = 1,
    NotConnectedToServer = 2,
    AlreadyInGame = 3,
    ChannelNotAvailable = 4,
    NameNotAvailable = 5,
    InvalidValue = 6,
    ServerBlacklisted = 100,
    ServerUnderlicensed = 101,
}
