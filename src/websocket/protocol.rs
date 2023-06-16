use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProtocolMessage {
    command: u32,
    server_unique_identifier: Option<String>,
    parameter: Option<ParamMessageType>,
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
    #[serde(default = "default_talk_state")]
    send_talk_states: bool,
    #[serde(default = "default_radio_traffic_state")]
    send_radio_traffic_states: bool,
    #[serde(default = "default_ultra_short_range_distance")]
    ultra_short_range_distance: f32,
    #[serde(default = "default_short_range_distance")]
    short_range_distance: f32,
    #[serde(default = "default_long_range_distance")]
    long_range_distance: f32,
}

fn default_talk_state() -> bool {
    true
}

fn default_radio_traffic_state() -> bool {
    false
}

fn default_ultra_short_range_distance() -> f32 {
    1800.0
}

fn default_short_range_distance() -> f32 {
    3000.0
}

fn default_long_range_distance() -> f32 {
    8000.0
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
    #[serde(default = "default_is_alive")]
    is_alive: bool,
    echo: Option<EchoEffect>,
}

fn default_is_alive() -> bool {
    true
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EchoEffect {
    #[serde(default = "default_duration")]
    duration: i32,
    #[serde(default = "default_rolloff")]
    rolloff: f32,
    #[serde(default = "default_delay")]
    delay: i32,
}

fn default_duration() -> i32 {
    100
}

fn default_rolloff() -> f32 {
    0.3
}

fn default_delay() -> i32 {
    25
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PlayerStateUpdateParameter {
    name: String,
    position: Vector3,
    rotation: f32,
    voice_range: f32,
    #[serde(default = "default_is_alive")]
    is_alive: bool,
    volume_override: Option<f32>,
    distance_culled: bool,
    muffle: Option<MuffleEffect>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MuffleEffect {
    #[serde(default = "default_intensity")]
    intensity: i32,
}

fn default_intensity() -> i32 {
    10
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
    volume: Option<f32>,
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
    volume: Option<f32>,
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
    #[serde(default = "default_range")]
    range: f32,
}

fn default_range() -> f32 {
    8000.0
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
    volume: Option<f32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct StopMegaphoneCommunicationParameter {
    name: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum GameInstanceState {
    NotConnected = 0,
    Connected = 1,
    Ingame = 2,
    InSwissChannel = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
enum RadioType {
    None = 1,
    ShortRange = 2,
    LongRange = 4,
    Distributed = 8,
    UltraShortRange = 16,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Vector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(untagged)]
enum ParamMessageType {
    PluginStateParameter(PluginStateParameter),
    InitiateParameter(InitiateParameter),
    InstanceStateParameter(InstanceStateParameter),
    SoundStateParameter(SoundStateParameter),
    SelfStateUpdateParameter(SelfStateUpdateParameter),
    PlayerStateUpdateParameter(PlayerStateUpdateParameter),
    BulkUpdateParameter(BulkUpdateParameter),
    RemovePlayerParameter(RemovePlayerParameter),
    TalkStateParameter(TalkStateParameter),
    PlaySoundParameter(PlaySoundParameter),
    StopSoundParameter(StopSoundParameter),
    PhoneCommunicationUpdateParameter(PhoneCommunicationUpdateParameter),
    StopPhoneCommunicationParameter(StopPhoneCommunicationParameter),
    RadioCommunicationUpdateParameter(RadioCommunicationUpdateParameter),
    StopRadioCommunicationParameter(StopRadioCommunicationParameter),
    RadioTowerUpdateParameter(RadioTowerUpdateParameter),
    RadioTrafficStateParameter(RadioTrafficStateParameter),
    AddRadioChannelMemberParameter(AddRadioChannelMemberParameter),
    UpdateRadioChannelMembersParameter(UpdateRadioChannelMembersParameter),
    RemoveRadioChannelMemberParameter(RemoveRadioChannelMemberParameter),
    MegaphoneCommunicationUpdateParameter(MegaphoneCommunicationUpdateParameter),
    StopMegaphoneCommunicationParameter(StopMegaphoneCommunicationParameter),
}
