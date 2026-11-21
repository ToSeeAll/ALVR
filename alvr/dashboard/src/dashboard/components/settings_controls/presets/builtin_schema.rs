use super::schema::{
    HigherOrderChoiceOption, HigherOrderChoiceSchema, PresetModifier, PresetSchemaNode,
};
use crate::dashboard::components::presets::schema::PresetModifierOperation;
use settings_schema::ChoiceControlType;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

fn string_modifier(target_path: &str, value: &str) -> PresetModifier {
    PresetModifier {
        target_path: target_path.into(),
        operation: PresetModifierOperation::Assign(serde_json::Value::String(value.into())),
    }
}
fn num_modifier(target_path: &str, value: &str) -> PresetModifier {
    PresetModifier {
        target_path: target_path.into(),
        operation: PresetModifierOperation::Assign(serde_json::Value::Number(
            serde_json::Number::from_str(value).unwrap(),
        )),
    }
}
fn bool_modifier(target_path: &str, value: bool) -> PresetModifier {
    PresetModifier {
        target_path: target_path.into(),
        operation: PresetModifierOperation::Assign(serde_json::Value::Bool(value)),
    }
}

pub fn resolution_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "分辨率".into(),
        strings: [(
            "help".into(),
            "选择过高的分辨率（通常是“高（宽 5184）”）可能导致高延迟或黑屏。".into(),
        )]
        .into_iter()
        .collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [
            ("极低（宽 3072）", "1536"),
            ("低（宽 3712）", "1856"),
            ("中（宽 4288）", "2144"),
            ("高（宽 5184）", "2592"),
            ("超高（宽 5632）", "2816"),
            ("极致（宽 6080）", "3040"),
        ]
        .into_iter()
        .map(|(key, value)| HigherOrderChoiceOption {
            display_name: key.into(),
            modifiers: [
                string_modifier(
                    "session_settings.video.transcoding_view_resolution.variant",
                    "Absolute",
                ),
                num_modifier(
                    "session_settings.video.transcoding_view_resolution.Absolute.width",
                    value,
                ),
                bool_modifier(
                    "session_settings.video.transcoding_view_resolution.Absolute.height.set",
                    false,
                ),
                string_modifier(
                    "session_settings.video.emulated_headset_view_resolution.variant",
                    "Absolute",
                ),
                num_modifier(
                    "session_settings.video.emulated_headset_view_resolution.Absolute.width",
                    value,
                ),
                bool_modifier(
                    "session_settings.video.emulated_headset_view_resolution.Absolute.height.set",
                    false,
                ),
            ]
            .into_iter()
            .collect(),
            content: None,
        })
        .collect(),
        default_option_display_name: "中（宽 4288）".into(),
        gui: ChoiceControlType::Dropdown,
    })
}

pub fn framerate_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "首选帧率".into(),
        strings: HashMap::new(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [60, 72, 80, 90, 120]
            .into_iter()
            .map(|framerate| HigherOrderChoiceOption {
                display_name: format!("{framerate}Hz"),
                modifiers: [num_modifier(
                    "session_settings.video.preferred_fps",
                    &format!("{:?}", framerate as f32),
                )]
                .into_iter()
                .collect(),
                content: None,
            })
            .collect(),
        default_option_display_name: "72Hz".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

pub fn codec_preset_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "编码器预设".into(),
        strings: [
            (
            "notice".into(),
            "AV1 编码仅受 RDNA3、Ada Lovelace、Intel ARC 或更新架构的显卡支持（AMD RX 7xxx+、NVIDIA RTX 40xx+、Intel ARC），\n并且需要头显端搭载 XR2 Gen 2（Quest 3、Pico 4 Ultra）。\n\n在 Windows 上，Intel ARC 显卡目前不支持 H264 编码。"
                .into(),
            ),
        ]
        .into_iter()
        .collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [("H264", "H264"), ("HEVC", "Hevc"), ("AV1", "AV1")]
            .into_iter()
            .map(|(key, val_codec)| HigherOrderChoiceOption {
                display_name: key.into(),
                modifiers: [string_modifier(
                    "session_settings.video.preferred_codec.variant",
                    val_codec,
                )]
                .into_iter()
                .collect(),
                content: None,
            })
            .collect(),
        default_option_display_name: "H264".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

pub fn encoder_preset_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "编码速度预设".into(),
        strings: [(
            "help".into(),
            "选择过高的质量可能导致卡顿甚至画面静止！".into(),
        )]
        .into_iter()
        .collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [
            ("速度", "Speed", "P1"),
            ("均衡", "Balanced", "P3"),
            ("质量", "Quality", "P5"),
        ]
        .into_iter()
        .map(|(key, val_amd, val_nv)| HigherOrderChoiceOption {
            display_name: key.into(),
            modifiers: [
                string_modifier(
                    "session_settings.video.encoder_config.nvenc.quality_preset.variant",
                    val_nv,
                ),
                string_modifier(
                    "session_settings.video.encoder_config.quality_preset.variant",
                    val_amd,
                ),
            ]
            .into_iter()
            .collect(),
            content: None,
        })
        .collect(),
        default_option_display_name: "速度".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

pub fn foveation_preset_schema() -> PresetSchemaNode {
    const PREFIX: &str = "session_settings.video.foveated_encoding";
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "中心凹预设".into(),
        strings: [(
            "help".into(),
            "中心凹编码会影响画面边缘的像素化程度，并能显著降低编码延迟。\n不建议完全关闭，否则可能出现卡顿以及很高的编码/解码延迟！"
                .into(),
        )]
        .into_iter()
        .collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [
            ("轻度", [0.80, 0.80], [8.0, 8.0]),
            ("中度", [0.66, 0.60], [6.0, 6.0]),
            ("重度", [0.45, 0.40], [4.0, 5.0]),
        ]
        .into_iter()
        .map(|(key, center_size, edge_ratio)| {
            let [center_size_x, center_size_y] = center_size;
            let [edge_ratio_x, edge_ratio_y] = edge_ratio;

            HigherOrderChoiceOption {
                display_name: key.into(),
                modifiers: [
                    bool_modifier(&format!("{PREFIX}.enabled"), true),
                    num_modifier(
                        &format!("{PREFIX}.content.center_size.content[0]"),
                        &center_size_x.to_string(),
                    ),
                    num_modifier(
                        &format!("{PREFIX}.content.center_size.content[1]"),
                        &center_size_y.to_string(),
                    ),
                    num_modifier(
                        &format!("{PREFIX}.content.edge_ratio.content[0]"),
                        &edge_ratio_x.to_string(),
                    ),
                    num_modifier(
                        &format!("{PREFIX}.content.edge_ratio.content[1]"),
                        &edge_ratio_y.to_string(),
                    ),
                ]
                .into_iter()
                .collect(),
                content: None,
            }
        })
        .collect(),
        default_option_display_name: "重度".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

#[cfg(target_os = "linux")]
pub fn game_audio_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "头显扬声器".into(),
        strings: HashMap::new(),
        flags: HashSet::new(),
        options: [
            HigherOrderChoiceOption {
                display_name: "禁用".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.game_audio.enabled",
                    false,
                )],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "启用".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.game_audio.enabled",
                    true,
                )],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "启用".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

#[cfg(target_os = "linux")]
pub fn microphone_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "头显麦克风".into(),
        strings: HashMap::new(),
        flags: HashSet::new(),
        options: [
            HigherOrderChoiceOption {
                display_name: "禁用".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.microphone.enabled",
                    false,
                )],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "启用".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.microphone.enabled",
                    true,
                )],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "启用".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

#[cfg(not(target_os = "linux"))]
pub fn game_audio_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "头显扬声器".into(),
        strings: [(
            "notice".into(),
            "你可以在系统任务栏托盘（右下角）修改默认音频设备".into(),
        )]
        .into_iter()
        .collect(),
        flags: HashSet::new(),
        options: vec![
            HigherOrderChoiceOption {
                display_name: "禁用".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.audio.game_audio.enabled",
                    false,
                )],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "系统默认".to_owned(),
                modifiers: vec![
                    bool_modifier("session_settings.audio.game_audio.enabled", true),
                    bool_modifier(
                        "session_settings.audio.game_audio.content.device.set",
                        false,
                    ),
                ],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "系统默认".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

#[cfg(not(target_os = "linux"))]
pub fn microphone_schema() -> PresetSchemaNode {
    let mut microhone_options = vec![HigherOrderChoiceOption {
        display_name: "禁用".to_owned(),
        modifiers: vec![bool_modifier(
            "session_settings.audio.microphone.enabled",
            false,
        )],
        content: None,
    }];

    if cfg!(windows) {
        for (key, display_name) in [
            ("Automatic", "自动"),
            ("VAC", "Virtual Audio Cable"),
            ("VBCable", "VB Cable"),
            ("VoiceMeeter", "VoiceMeeter"),
            ("VoiceMeeterAux", "VoiceMeeter Aux"),
            ("VoiceMeeterVaio3", "VoiceMeeter VAIO3"),
        ] {
            microhone_options.push(HigherOrderChoiceOption {
                display_name: display_name.into(),
                modifiers: vec![
                    bool_modifier("session_settings.audio.microphone.enabled", true),
                    string_modifier(
                        "session_settings.audio.microphone.content.devices.variant",
                        key,
                    ),
                ],
                content: None,
            })
        }
    }

    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "头显麦克风".into(),
        strings: HashMap::new(),
        flags: HashSet::new(),
        options: microhone_options.into_iter().collect(),
        default_option_display_name: "禁用".into(),
        gui: ChoiceControlType::Dropdown,
    })
}

pub fn hand_tracking_interaction_schema() -> PresetSchemaNode {
    const HELP: &str = r"禁用：手势不能模拟按键，适合使用 Joy-Con 等非原生控制器。
SteamVR Input 2.0：为手部追踪创建独立的 SteamVR 设备。
ALVR 绑定：使用 ALVR 的手部追踪按键绑定，详细用法见 wiki。
";

    const PREFIX: &str = "session_settings.headset.controllers.content";

    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "手部追踪交互".into(),
        strings: [("help".into(), HELP.into())].into_iter().collect(),
        flags: ["steamvr-restart".into()].into_iter().collect(),
        options: [
            HigherOrderChoiceOption {
                display_name: "禁用".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.controllers.enabled", true),
                    bool_modifier(
                        &format!("{PREFIX}.hand_skeleton.content.steamvr_input_2_0"),
                        false,
                    ),
                    bool_modifier(
                        &format!("{PREFIX}.hand_tracking_interaction.enabled"),
                        false,
                    ),
                ],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "SteamVR Input 2.0".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.controllers.enabled", true),
                    bool_modifier(&format!("{PREFIX}.hand_skeleton.enabled"), true),
                    bool_modifier(
                        &format!("{PREFIX}.hand_skeleton.content.steamvr_input_2_0"),
                        true,
                    ),
                    bool_modifier(
                        &format!("{PREFIX}.hand_tracking_interaction.enabled"),
                        false,
                    ),
                ],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "ALVR 绑定".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.controllers.enabled", true),
                    bool_modifier(
                        &format!("{PREFIX}.hand_skeleton.content.steamvr_input_2_0"),
                        false,
                    ),
                    bool_modifier(&format!("{PREFIX}.hand_tracking_interaction.enabled"), true),
                ],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "SteamVR Input 2.0".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}

pub fn eye_face_tracking_schema() -> PresetSchemaNode {
    PresetSchemaNode::HigherOrderChoice(HigherOrderChoiceSchema {
        name: "眼动与面部追踪".into(),
        strings: HashMap::new(),
        flags: HashSet::new(),
        options: [
            HigherOrderChoiceOption {
                display_name: "禁用".into(),
                modifiers: vec![bool_modifier(
                    "session_settings.headset.face_tracking.enabled",
                    false,
                )],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "VRChat Eye OSC".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.face_tracking.enabled", true),
                    bool_modifier(
                        "session_settings.headset.face_tracking.content.sink.social_presence.enabled",
                        true,
                    ),
                    string_modifier(
                        "session_settings.headset.face_tracking.content.sink.social_presence.content.variant",
                        "VrchatEyeOsc",
                    ),
                ],
                content: None,
            },
            HigherOrderChoiceOption {
                display_name: "VRCFaceTracking".into(),
                modifiers: vec![
                    bool_modifier("session_settings.headset.face_tracking.enabled", true),
                    bool_modifier(
                        "session_settings.headset.face_tracking.content.sink.social_presence.enabled",
                        true,
                    ),
                    string_modifier(
                        "session_settings.headset.face_tracking.content.sink.social_presence.content.variant",
                        "VrcFaceTracking",
                    ),
                ],
                content: None,
            },
        ]
        .into_iter()
        .collect(),
        default_option_display_name: "禁用".into(),
        gui: ChoiceControlType::ButtonGroup,
    })
}
