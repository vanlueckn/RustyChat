use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ProtocolMessage {
    pub command: Command,
    pub server_unique_identifier: Option<String>,
    pub parameter: Option<ParamMessageType>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PluginStateParameter {
    pub version: String,
    pub active_instances: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct InitiateParameter {
    pub server_unique_identifier: String,
    pub name: String,
    pub channel_id: u64,
    pub channel_password: String,
    pub sound_pack: String,
    pub swiss_channel_ids: Vec<u64>,
    #[serde(default = "default_talk_state")]
    pub send_talk_states: bool,
    #[serde(default = "default_radio_traffic_state")]
    pub send_radio_traffic_states: bool,
    #[serde(default = "default_ultra_short_range_distance")]
    pub ultra_short_range_distance: f32,
    #[serde(default = "default_short_range_distance")]
    pub short_range_distance: f32,
    #[serde(default = "default_long_range_distance")]
    pub long_range_distance: f32,
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
pub struct InstanceStateParameter {
    pub is_connected_to_server: bool,
    pub is_ready: bool,
    pub state: GameInstanceState,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SoundStateParameter {
    pub is_microphone_muted: bool,
    pub is_microphone_enabled: bool,
    pub is_sound_muted: bool,
    pub is_sound_enabled: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SelfStateUpdateParameter {
    pub position: Vector3,
    pub rotation: f32,
    pub voice_range: f32,
    #[serde(default = "default_is_alive")]
    pub is_alive: bool,
    pub echo: Option<EchoEffect>,
}

fn default_is_alive() -> bool {
    true
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct EchoEffect {
    #[serde(default = "default_duration")]
    pub duration: i32,
    #[serde(default = "default_rolloff")]
    pub rolloff: f32,
    #[serde(default = "default_delay")]
    pub delay: i32,
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

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerStateUpdateParameter {
    pub name: String,
    pub position: Vector3,
    pub rotation: f32,
    pub voice_range: f32,
    #[serde(default = "default_is_alive")]
    pub is_alive: bool,
    pub volume_override: Option<f32>,
    pub distance_culled: bool,
    pub muffle: Option<MuffleEffect>,
}

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct MuffleEffect {
    #[serde(default = "default_intensity")]
    intensity: i32,
}

fn default_intensity() -> i32 {
    10
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BulkUpdateParameter {
    pub player_states: Vec<PlayerStateUpdateParameter>,
    pub self_state: SelfStateUpdateParameter,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemovePlayerParameter {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TalkStateParameter {
    pub name: String,
    pub is_talking: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlaySoundParameter {
    pub file_name: String,
    pub is_loop: bool,
    pub handle: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopSoundParameter {
    pub handle: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PhoneCommunicationUpdateParameter {
    pub name: String,
    pub signal_strength: i32,
    pub volume: Option<f32>,
    pub direct: bool,
    pub relayed_by: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopPhoneCommunicationParameter {
    name: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioCommunicationUpdateParameter {
    pub name: String,
    pub sender_radio_type: RadioType,
    pub own_radio_type: RadioType,
    pub play_mic_click: bool,
    pub volume: Option<f32>,
    pub direct: bool,
    pub secondary: bool,
    pub relayed_by: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopRadioCommunicationParameter {
    pub name: String,
    pub play_mic_click: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioTowerUpdateParameter {
    pub towers: Vec<Tower>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Tower {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    #[serde(default = "default_range")]
    pub range: f32,
}

fn default_range() -> f32 {
    8000.0
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RadioTrafficStateParameter {
    pub name: String,
    pub is_sending: bool,
    pub is_primary_channel: bool,
    pub active_relay: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AddRadioChannelMemberParameter {
    pub player_name: String,
    pub is_primary_channel: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct UpdateRadioChannelMembersParameter {
    pub player_names: Vec<String>,
    pub is_primary_channel: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct RemoveRadioChannelMemberParameter {
    pub player_name: String,
    pub is_primary_channel: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MegaphoneCommunicationUpdateParameter {
    pub name: String,
    pub range: f32,
    pub volume: Option<f32>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct StopMegaphoneCommunicationParameter {
    pub name: String,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum GameInstanceState {
    NotConnected = 0,
    Connected = 1,
    Ingame = 2,
    InSwissChannel = 3,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum RadioType {
    None = 1,
    ShortRange = 2,
    LongRange = 4,
    Distributed = 8,
    UltraShortRange = 16,
}

#[derive(Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Error {
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

#[derive(Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "PascalCase")]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
#[serde(untagged)]
pub enum ParamMessageType {
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

#[derive(Serialize, Deserialize)]
#[repr(u32)]
pub enum Command {
    // Plugin
    PluginState = 0,

    // Instance
    Initiate = 1,
    Reset = 2,
    Ping = 3,
    Pong = 4,
    InstanceState = 5,
    SoundState = 6,
    SelfStateUpdate = 7,
    PlayerStateUpdate = 8,
    BulkUpdate = 9,
    RemovePlayer = 10,
    TalkState = 11,
    PlaySound = 18,
    StopSound = 19,

    // Phone
    PhoneCommunicationUpdate = 20,
    StopPhoneCommunication = 21,

    // Radio
    RadioCommunicationUpdate = 30,
    StopRadioCommunication = 31,
    RadioTowerUpdate = 32,
    RadioTrafficState = 33,

    AddRadioChannelMember = 37,
    UpdateRadioChannelMembers = 38,
    RemoveRadioChannelMember = 39,

    // Megaphone
    MegaphoneCommunicationUpdate = 40,
    StopMegaphoneCommunication = 41,
}
