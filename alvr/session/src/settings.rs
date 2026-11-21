use alvr_common::{
    ALVR_VERSION, DebugGroupsConfig, DebugGroupsConfigDefault, LogSeverity, LogSeverityDefault,
    LogSeverityDefaultVariant,
};
use alvr_system_info::{ClientFlavor, ClientFlavorDefault, ClientFlavorDefaultVariant};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use settings_schema::{
    ArrayDefault, DictionaryDefault, OptionalDefault, SettingsSchema, Switch, SwitchDefault,
    VectorDefault,
};

include!(concat!(env!("OUT_DIR"), "/openvr_property_keys.rs"));

pub enum OpenvrPropType {
    Bool,
    Float,
    Int32,
    Uint64,
    Vector3,
    Double,
    String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug)]
pub struct OpenvrProperty {
    #[schema(strings(display_name = "键"))]
    pub key: OpenvrPropKey,
    #[schema(strings(display_name = "值"))]
    pub value: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum FrameSize {
    #[schema(strings(display_name = "按比例缩放"))]
    Scale(#[schema(gui(slider(min = 0.25, max = 2.0, step = 0.01)))] f32),

    #[schema(strings(display_name = "绝对尺寸"))]
    Absolute {
        #[schema(strings(display_name = "宽"))]
        #[schema(gui(slider(min = 32, max = 8192, step = 32)))]
        width: u32,
        #[schema(strings(display_name = "高"))]
        #[schema(gui(slider(min = 32, max = 8192, step = 32)))]
        height: Option<u32>,
    },
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum EncoderQualityPreset {
    #[schema(strings(display_name = "质量"))]
    Quality = 0,
    #[schema(strings(display_name = "均衡"))]
    Balanced = 1,
    #[schema(strings(display_name = "速度"))]
    Speed = 2,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum EncoderQualityPresetNvidia {
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum NvencTuningPreset {
    #[schema(strings(display_name = "高画质"))]
    HighQuality = 1,
    #[schema(strings(display_name = "低延迟"))]
    LowLatency = 2,
    #[schema(strings(display_name = "超低延迟"))]
    UltraLowLatency = 3,
    #[schema(strings(display_name = "无损"))]
    Lossless = 4,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum NvencMultiPass {
    #[schema(strings(display_name = "禁用"))]
    Disabled = 0,
    #[schema(strings(display_name = "1/4 分辨率"))]
    QuarterResolution = 1,
    #[schema(strings(display_name = "完整分辨率"))]
    FullResolution = 2,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum NvencAdaptiveQuantizationMode {
    #[schema(strings(display_name = "禁用"))]
    Disabled = 0,
    #[schema(strings(display_name = "空间"))]
    Spatial = 1,
    #[schema(strings(display_name = "时间"))]
    Temporal = 2,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[schema(gui = "button_group")]
pub enum RateControlMode {
    #[schema(strings(display_name = "CBR"))]
    Cbr = 0,
    #[schema(strings(display_name = "VBR"))]
    Vbr = 1,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[schema(gui = "button_group")]
pub enum EntropyCoding {
    #[schema(strings(display_name = "CAVLC"))]
    Cavlc = 1,
    #[schema(strings(display_name = "CABAC"))]
    Cabac = 0,
}

/// Except for preset, the value of these fields is not applied if == -1 (flag)
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct NvencConfig {
    #[schema(strings(
        display_name = "质量预设",
        help = "P1 是最快的预设，P7 画质最好。P6 和 P7 速度太慢，实际不可用。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub quality_preset: EncoderQualityPresetNvidia,
    #[schema(strings(display_name = "调优预设"))]
    #[schema(flag = "steamvr-restart")]
    pub tuning_preset: NvencTuningPreset,
    #[schema(strings(
        display_name = "多次编码",
        help = "以少量性能开销换取更少的压缩伪影"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub multi_pass: NvencMultiPass,
    #[schema(strings(
        display_name = "自适应量化模式",
        help = "空间：有助于减少色彩断层，但高复杂度场景可能更差。\n时间：有助于提升整体编码质量，速度损失很小。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub adaptive_quantization_mode: NvencAdaptiveQuantizationMode,
    #[schema(strings(display_name = "低延迟关键帧缩放"))]
    #[schema(flag = "steamvr-restart")]
    pub low_delay_key_frame_scale: i64,
    #[schema(strings(display_name = "刷新率"))]
    #[schema(flag = "steamvr-restart")]
    pub refresh_rate: i64,
    #[schema(strings(display_name = "启用帧内刷新"))]
    #[schema(flag = "steamvr-restart")]
    pub enable_intra_refresh: bool,
    #[schema(strings(display_name = "帧内刷新周期"))]
    #[schema(flag = "steamvr-restart")]
    pub intra_refresh_period: i64,
    #[schema(strings(display_name = "帧内刷新次数"))]
    #[schema(flag = "steamvr-restart")]
    pub intra_refresh_count: i64,
    #[schema(strings(display_name = "最大参考帧数"))]
    #[schema(flag = "steamvr-restart")]
    pub max_num_ref_frames: i64,
    #[schema(strings(display_name = "GOP 长度"))]
    #[schema(flag = "steamvr-restart")]
    pub gop_length: i64,
    #[schema(strings(display_name = "P 帧策略"))]
    #[schema(flag = "steamvr-restart")]
    pub p_frame_strategy: i64,
    #[schema(strings(display_name = "码率控制模式"))]
    #[schema(flag = "steamvr-restart")]
    pub rate_control_mode: i64,
    #[schema(strings(display_name = "码控缓冲区大小"))]
    #[schema(flag = "steamvr-restart")]
    pub rc_buffer_size: i64,
    #[schema(strings(display_name = "码控初始延迟"))]
    #[schema(flag = "steamvr-restart")]
    pub rc_initial_delay: i64,
    #[schema(strings(display_name = "最大码率"))]
    #[schema(flag = "steamvr-restart")]
    pub rc_max_bitrate: i64,
    #[schema(strings(display_name = "平均码率"))]
    #[schema(flag = "steamvr-restart")]
    pub rc_average_bitrate: i64,
    #[schema(strings(display_name = "启用加权预测"))]
    #[schema(flag = "steamvr-restart")]
    pub enable_weighted_prediction: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct AmfConfig {
    #[schema(
        strings(
            display_name = "启用高动态画质增强",
            help = "启用高动态画质增强模式。\n允许编码器预先分析画面运动，并用这些信息获得更好的编码效果"
        ),
        flag = "steamvr-restart"
    )]
    pub enable_hmqb: bool,
    #[schema(strings(display_name = "使用预处理"))]
    #[schema(flag = "steamvr-restart")]
    pub use_preproc: bool,
    #[schema(strings(display_name = "预处理强度"))]
    #[schema(gui(slider(min = 0, max = 10)))]
    #[schema(flag = "steamvr-restart")]
    pub preproc_sigma: u32,
    #[schema(strings(display_name = "预处理阈值"))]
    #[schema(gui(slider(min = 0, max = 10)))]
    #[schema(flag = "steamvr-restart")]
    pub preproc_tor: u32,
    #[schema(
        strings(
            display_name = "启用预分析",
            help = "在编码时启用预分析。通常会降低性能，但可能提升画质。\n与“减少色彩断层”选项不兼容，需要先启用“使用预处理”"
        ),
        flag = "steamvr-restart"
    )]
    pub enable_pre_analysis: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct SoftwareEncodingConfig {
    #[schema(strings(
        display_name = "强制使用软件编码",
        help = "强制编码器使用 CPU 而不是 GPU"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub force_software_encoding: bool,

    #[schema(strings(display_name = "编码线程数"))]
    #[schema(flag = "steamvr-restart")]
    pub thread_count: u32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct HDRConfig {
    #[schema(strings(
        display_name = "启用 HDR",
        help = "如果客户端没有偏好，则把 VR 图层合成到 RGBA float16 帧缓冲，并在着色器中完成 sRGB/YUV 转换。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub enable: Option<bool>,

    #[schema(strings(
        display_name = "强制 HDR sRGB 校正",
        help = "对所有参与合成的 SteamVR 图层强制进行 sRGB 校正。适用于注入 HDR 后画面过暗的游戏。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub force_hdr_srgb_correction: bool,

    #[schema(strings(
        display_name = "限制 HDR 扩展范围",
        help = "把 HDR 扩展范围限制在 0.0~1.0，适合只想用 HDR 减少色彩断层的情况。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub clamp_hdr_extended_range: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct EncoderConfig {
    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        display_name = "质量预设",
        help = "控制编码器的整体质量预设。仅在 Windows 的 AMD AMF、Linux 的 VAAPI（AMD/Intel）上生效。"
    ))]
    pub quality_preset: EncoderQualityPreset,

    #[schema(
        strings(
            display_name = "启用 VBAQ/CAQ",
            help = "在 h264 与 HEVC 上启用基于方差的量化，在 AV1 上启用内容自适应量化"
        ),
        flag = "steamvr-restart"
    )]
    pub enable_vbaq: bool,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(display_name = "码率控制模式", help = "CBR：固定码率模式，推荐使用。\nVBR：可变码率模式。不推荐，因为它可能干扰自适应码率算法。仅 Windows 上的 AMD/NVIDIA 显卡支持。"))]
    #[schema(flag = "steamvr-restart")]
    pub rate_control_mode: RateControlMode,

    #[schema(strings(
        display_name = "h264 配置档",
        help = "在有条件时尝试使用该配置档，可提高与不同移动设备的兼容性。只对 h264 生效，不影响 Windows 上的 NVENC。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub h264_profile: H264Profile,

    #[schema(strings(display_name = "熵编码", help = "推荐使用 CAVLC 算法。\nCABAC 压缩率更好，但明显更慢，可能导致延迟失控。"))]
    #[schema(flag = "steamvr-restart")]
    pub entropy_coding: EntropyCoding,

    #[schema(strings(
        display_name = "填充数据",
        help = "在 CBR 模式下确保码率不低于指定值。主要用于调试。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub filler_data: bool,

    #[schema(strings(
        display_name = "10 位编码",
        help = "如果客户端没有偏好，则让编码器每通道使用 10 位而不是 8 位。在 Linux 搭配 Nvidia 时不可用"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub use_10bit: Option<bool>,

    #[schema(strings(
        display_name = "编码 Gamma",
        help = "若希望优先保留暗部细节（代价是中间调可能出现额外断层），设为 2.2。若希望由编码器自行决定优先级，设为 1.0。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub encoding_gamma: Option<f32>,

    #[schema(strings(display_name = "HDR"))]
    #[schema(flag = "steamvr-restart")]
    pub hdr: HDRConfig,

    #[schema(strings(display_name = "NVENC"))]
    #[schema(flag = "steamvr-restart")]
    pub nvenc: NvencConfig,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(display_name = "AMF"))]
    #[schema(flag = "steamvr-restart")]
    pub amf: AmfConfig,

    #[schema(strings(display_name = "软件（CPU）编码"))]
    pub software: SoftwareEncodingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MediacodecPropType {
    #[schema(strings(display_name = "浮点数"))]
    Float,
    #[schema(strings(display_name = "32 位整数"))]
    Int32,
    #[schema(strings(display_name = "64 位整数"))]
    Int64,
    #[schema(strings(display_name = "字符串"))]
    String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MediacodecProperty {
    #[schema(strings(display_name = "类型"))]
    pub ty: MediacodecPropType,
    #[schema(strings(display_name = "值"))]
    pub value: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct EncoderLatencyLimiter {
    #[schema(strings(
        display_name = "最大时间占比",
        help = "允许分配给视频编码的帧间隔百分比"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.3, max = 1.0, step = 0.01)))]
    pub max_saturation_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct DecoderLatencyLimiter {
    #[schema(strings(
        display_name = "解码器最大延迟",
        help = "当解码延迟超过该阈值时，将降低码率"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 50)), suffix = "ms")]
    pub max_decoder_latency_ms: u64,

    #[schema(strings(
        display_name = "延迟超限帧数",
        help = "连续超过阈值的帧数达到该值后触发降码率"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 100)), suffix = " frames")]
    pub latency_overstep_frames: usize,

    #[schema(strings(
        display_name = "超限降码率系数",
        help = "控制解码延迟超过阈值时码率下调的幅度"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.5, max = 1.0)))]
    pub latency_overstep_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum BitrateMode {
    #[schema(strings(display_name = "固定码率"))]
    ConstantMbps(#[schema(gui(slider(min = 5, max = 1000, logarithmic)), suffix = "Mbps")] u64),

    #[schema(strings(display_name = "自适应"))]
    #[schema(collapsible)]
    Adaptive {
        #[schema(strings(
            display_name = "带宽占用比例",
            help = "分配给视频传输的网络带宽百分比"
        ))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 0.5, max = 5.0, step = 0.01)))]
        saturation_multiplier: f32,

        #[schema(strings(display_name = "最大码率"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 1000, logarithmic)), suffix = "Mbps")]
        max_throughput_mbps: Switch<u64>,

        #[schema(strings(display_name = "最小码率"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 100, logarithmic)), suffix = "Mbps")]
        min_throughput_mbps: Switch<u64>,

        #[schema(strings(display_name = "最大网络延迟"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 50)), suffix = "ms")]
        max_network_latency_ms: Switch<u64>,

        #[schema(strings(display_name = "编码延迟限制器"))]
        #[schema(flag = "real-time")]
        encoder_latency_limiter: Switch<EncoderLatencyLimiter>,

        #[schema(strings(
            display_name = "解码延迟限制器",
            help = "目前存在一个 bug：码率高于某个值后，解码延迟会持续上升"
        ))]
        #[schema(flag = "real-time")]
        decoder_latency_limiter: Switch<DecoderLatencyLimiter>,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct BitrateAdaptiveFramerateConfig {
    #[schema(strings(
        display_name = "帧率重置阈值倍数",
        help = "如果帧率变化超过该倍数，则触发一次参数更新",
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1.0, max = 3.0, step = 0.1)))]
    pub framerate_reset_threshold_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct BitrateConfig {
    #[schema(strings(display_name = "码率模式"))]
    #[schema(flag = "real-time")]
    pub mode: BitrateMode,

    #[schema(strings(
        display_name = "随帧率自适应",
        help = "确保无论帧率如何变化，指定的码率值都被遵守"
    ))]
    #[schema(flag = "real-time")]
    pub adapt_to_framerate: Switch<BitrateAdaptiveFramerateConfig>,

    #[schema(strings(display_name = "历史长度", help = "控制计算过程的平滑程度"))]
    pub history_size: usize,

    #[schema(strings(
        display_name = "画面损坏修复",
        help = "启用后，码率发生变化时会请求一个 IDR 帧。\n仅对 AMD 显卡有效。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub image_corruption_fix: bool,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum ClientsideFoveationLevel {
    #[schema(strings(display_name = "低"))]
    Low = 1,
    #[schema(strings(display_name = "中"))]
    Medium = 2,
    #[schema(strings(display_name = "高"))]
    High = 3,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum ClientsideFoveationMode {
    #[schema(strings(display_name = "固定"))]
    Static { level: ClientsideFoveationLevel },
    #[schema(strings(display_name = "动态"))]
    Dynamic { max_level: ClientsideFoveationLevel },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClientsideFoveationConfig {
    #[schema(strings(display_name = "模式"))]
    pub mode: ClientsideFoveationMode,

    #[schema(strings(display_name = "中心凹偏移"))]
    #[schema(gui(slider(min = -45.0, max = 45.0, step = 0.1)), suffix = "°")]
    pub vertical_offset_deg: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct FoveatedEncodingConfig {
    #[schema(strings(display_name = "在手机客户端上强制启用", help = "在手机客户端上强制启用"))]
    pub force_enable: bool,

    #[schema(strings(
        display_name = "中心区域尺寸",
        help = "中心区域的宽和高，按 X/Y 顺序"
    ))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_size: [f32; 2],

    #[schema(strings(
        display_name = "中心偏移",
        help = "水平与垂直偏移，按 X/Y 顺序。作为静态回退值；当头显提供眼动中心时会被覆盖。"
    ))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_shift: [f32; 2],

    #[schema(strings(display_name = "边缘压缩比", help = "水平与垂直的压缩比，按 X/Y 顺序"))]
    #[schema(gui(slider(min = 1.0, max = 10.0, step = 1.0)))]
    #[schema(flag = "steamvr-restart")]
    pub edge_ratio: [f32; 2],
}

#[repr(C)]
#[derive(SettingsSchema, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
pub struct ColorCorrectionConfig {
    #[schema(strings(display_name = "亮度"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.001)))]
    #[schema(flag = "steamvr-restart")]
    pub brightness: f32,

    #[schema(strings(display_name = "对比度"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.001)))]
    #[schema(flag = "steamvr-restart")]
    pub contrast: f32,

    #[schema(strings(display_name = "饱和度"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub saturation: f32,

    #[schema(strings(display_name = "Gamma"))]
    #[schema(gui(slider(min = 0.0, max = 5.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub gamma: f32,

    #[schema(strings(display_name = "锐化"))]
    #[schema(gui(slider(min = -1.0, max = 5.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub sharpening: f32,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[schema(gui = "button_group")]
pub enum CodecType {
    #[default]
    #[schema(strings(display_name = "h264"))]
    H264 = 0,
    #[schema(strings(display_name = "HEVC"))]
    Hevc = 1,
    #[schema(strings(display_name = "AV1"))]
    AV1 = 2,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum H264Profile {
    #[schema(strings(display_name = "High"))]
    High = 0,
    #[schema(strings(display_name = "Main"))]
    Main = 1,
    #[schema(strings(display_name = "Baseline"))]
    Baseline = 2,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RgbChromaKeyConfig {
    #[schema(strings(display_name = "红"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub red: u8,

    #[schema(strings(display_name = "绿"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub green: u8,

    #[schema(strings(display_name = "蓝"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub blue: u8,

    #[schema(strings(display_name = "距离阈值", help = "阈值按通道分别应用"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 255)))]
    pub distance_threshold: u8,

    #[schema(strings(display_name = "边缘羽化"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.01, max = 1.0, step = 0.01)))]
    pub feathering: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct HsvChromaKeyConfig {
    #[schema(strings(display_name = "色相起点上限"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_start_max_deg: f32,

    #[schema(strings(display_name = "色相起点下限"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_start_min_deg: f32,

    #[schema(strings(display_name = "色相终点下限"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_end_min_deg: f32,

    #[schema(strings(display_name = "色相终点上限"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_end_max_deg: f32,

    #[schema(strings(display_name = "饱和度起点上限"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_start_max: f32,

    #[schema(strings(display_name = "饱和度起点下限"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_start_min: f32,

    #[schema(strings(display_name = "饱和度终点下限"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_end_min: f32,

    #[schema(strings(display_name = "饱和度终点上限"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5,step = 0.01)))]
    pub saturation_end_max: f32,

    #[schema(strings(display_name = "明度起点上限"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_start_max: f32,

    #[schema(strings(display_name = "明度起点下限"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_start_min: f32,

    #[schema(strings(display_name = "明度终点下限"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_end_min: f32,

    #[schema(strings(display_name = "明度终点上限"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_end_max: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[schema(gui = "button_group")]
pub enum PassthroughMode {
    #[schema(strings(display_name = "混合"))]
    Blend {
        #[schema(strings(
            display_name = "预乘 Alpha",
            help = "启用后会根据每个像素的亮度调整透明度。\n效果类似 AR 眼镜。"
        ))]
        #[schema(flag = "real-time")]
        premultiplied_alpha: bool,

        #[schema(strings(display_name = "阈值"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
        threshold: f32,
    },

    #[schema(strings(display_name = "RGB 抠像"))]
    RgbChromaKey(#[schema(flag = "real-time")] RgbChromaKeyConfig),

    #[schema(strings(display_name = "HSV 抠像"))]
    HsvChromaKey(#[schema(flag = "real-time")] HsvChromaKeyConfig),
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum ClientsidePostProcessingSuperSamplingMode {
    #[schema(strings(display_name = "禁用"))]
    Disabled = 0,
    #[schema(strings(display_name = "普通"))]
    Normal = 1 << 0,
    #[schema(strings(display_name = "高质量"))]
    Quality = 1 << 1,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum ClientsidePostProcessingSharpeningMode {
    #[schema(strings(display_name = "禁用"))]
    Disabled = 0,
    #[schema(strings(display_name = "普通"))]
    Normal = 1 << 2,
    #[schema(strings(display_name = "高质量"))]
    Quality = 1 << 3,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ClientsidePostProcessingConfig {
    #[schema(strings(
        display_name = "超采样",
        help = "减少高对比边缘的闪烁。\n当输入分辨率相对头显较高时有用"
    ))]
    pub super_sampling: ClientsidePostProcessingSuperSamplingMode,
    #[schema(strings(
        display_name = "锐化",
        help = "提升高对比边缘的清晰度、抵消模糊。\n当输入分辨率相对头显较低时有用"
    ))]
    pub sharpening: ClientsidePostProcessingSharpeningMode,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UpscalingConfig {
    #[schema(strings(
        display_name = "边缘方向放大",
        help = "利用边缘方向进行放大，以极小的性能损失提升画质"
    ))]
    pub edge_direction: bool,
    #[schema(strings(display_name = "边缘阈值"))]
    #[schema(gui(slider(min = 1.0, max = 16.0, step = 1.0)))]
    pub edge_threshold: f32,
    #[schema(strings(display_name = "边缘锐度"))]
    #[schema(gui(slider(min = 1.0, max = 2.0, step = 0.01)))]
    pub edge_sharpness: f32,
    #[schema(gui(slider(min = 1.0, max = 3.0, step = 0.01)))]
    #[schema(strings(
        display_name = "放大倍数",
        help = "分辨率放大倍数，数值过高会在性能较弱或分辨率较高的头显上造成性能问题"
    ))]
    pub upscale_factor: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct VideoConfig {
    #[schema(strings(display_name = "透视"))]
    #[schema(flag = "real-time")]
    pub passthrough: Switch<PassthroughMode>,

    #[schema(strings(display_name = "码率"))]
    pub bitrate: BitrateConfig,

    #[schema(strings(
        display_name = "首选编码",
        help = "HEVC 可能提供更好的画面保真度，代价是编码延迟增加"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub preferred_codec: CodecType,

    #[schema(strings(
        display_name = "中心凹编码",
        notice = "关闭中心凹编码可能导致编码/解码时间显著增加、卡顿甚至崩溃。\n如果想减少画面边缘的像素化，请增大中心区域的宽和高。\n要使用眼动中心凹，请启用“头显 > 面部追踪 > 输出 > 眼动追踪中心凹编码”。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub foveated_encoding: Switch<FoveatedEncodingConfig>,

    #[schema(strings(display_name = "色彩校正"))]
    #[schema(flag = "steamvr-restart")]
    pub color_correction: Switch<ColorCorrectionConfig>,

    #[schema(
        strings(
            display_name = "最大缓冲",
            help = "增大该值有助于减少卡顿，但会增加延迟"
        ),
        gui(slider(min = 1.0, max = 10.0, step = 0.1, logarithmic)),
        suffix = " frames"
    )]
    pub max_buffering_frames: f32,

    #[schema(strings(display_name = "缓冲历史权重"))]
    #[schema(gui(slider(min = 0.50, max = 0.99, step = 0.01)))]
    pub buffering_history_weight: f32,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(
        display_name = "强制串流端帧率同步",
        help = "仅在 Windows 上生效。除非明确知道 VR 游戏达不到目标帧率，否则不应关闭。"
    ))]
    #[schema(flag = "real-time")]
    pub enforce_server_frame_pacing: bool,

    #[schema(strings(display_name = "编码器配置"))]
    #[schema(flag = "steamvr-restart")]
    pub encoder_config: EncoderConfig,

    #[schema(strings(
        display_name = "强制使用软件解码",
        help = "尝试在设备上使用软件解码器。速度较慢，但可绕过损坏的编解码器。"
    ))]
    pub force_software_decoder: bool,

    #[schema(strings(display_name = "Mediacodec 额外选项"))]
    pub mediacodec_extra_options: Vec<(String, MediacodecProperty)>,

    #[schema(strings(
        display_name = "转码视图分辨率",
        help = "用于编码和解码的分辨率，相对于单眼视图。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub transcoding_view_resolution: FrameSize,

    #[schema(strings(
        display_name = "模拟头显视图分辨率",
        help = "这是 SteamVR 用于游戏渲染的默认分辨率，相对于单眼视图。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub emulated_headset_view_resolution: FrameSize,

    #[schema(strings(display_name = "首选帧率"))]
    #[schema(gui(slider(min = 60.0, max = 120.0)), suffix = "Hz")]
    #[schema(flag = "steamvr-restart")]
    pub preferred_fps: f32,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(
        display_name = "显卡适配器索引",
        help = "通常不需要修改。用于更改 ALVR 合成器使用的显卡。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub adapter_index: u32,

    #[schema(strings(display_name = "客户端中心凹"))]
    pub clientside_foveation: Switch<ClientsideFoveationConfig>,

    #[schema(strings(
        display_name = "客户端后处理",
        help = "硬件优化的算法，在 Quest 和 Pico 头显上可用"
    ))]
    #[schema(flag = "real-time")]
    pub clientside_post_processing: Switch<ClientsidePostProcessingConfig>,

    #[schema(strings(display_name = "客户端放大", help = "Snapdragon Game Super Resolution 客户端放大"))]
    pub upscaling: Switch<UpscalingConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum CustomAudioDeviceConfig {
    #[schema(strings(display_name = "按名称（子串）"))]
    NameSubstring(String),
    #[schema(strings(display_name = "按索引"))]
    Index(usize),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct AudioBufferingConfig {
    #[schema(strings(display_name = "平均缓冲"))]
    #[schema(gui(slider(min = 0, max = 200)), suffix = "ms")]
    pub average_buffering_ms: u64,

    #[schema(strings(display_name = "批次大小"))]
    #[schema(gui(slider(min = 1, max = 20)), suffix = "ms")]
    pub batch_ms: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct GameAudioConfig {
    #[schema(strings(display_name = "输出设备"))]
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    pub device: Option<CustomAudioDeviceConfig>,

    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    #[schema(strings(display_name = "串流时静音桌面音频"))]
    pub mute_when_streaming: bool,

    #[schema(strings(display_name = "缓冲"))]
    pub buffering: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum MicrophoneDevicesConfig {
    #[schema(strings(display_name = "自动"))]
    Automatic,
    #[schema(strings(display_name = "Virtual Audio Cable"))]
    VAC,
    #[schema(strings(display_name = "VB Cable"))]
    VBCable,
    #[schema(strings(display_name = "VoiceMeeter"))]
    VoiceMeeter,
    #[schema(strings(display_name = "VoiceMeeter Aux"))]
    VoiceMeeterAux,
    #[schema(strings(display_name = "VoiceMeeter VAIO3"))]
    VoiceMeeterVaio3,
    #[schema(strings(display_name = "自定义"))]
    Custom {
        #[schema(strings(
            display_name = "输出设备",
            help = "ALVR 使用该设备输出麦克风音频"
        ))]
        sink: CustomAudioDeviceConfig,
        #[schema(strings(
            display_name = "输入设备",
            help = "该设备会在 SteamVR 中被设为默认麦克风"
        ))]
        source: CustomAudioDeviceConfig,
    },
}

// Note: sample rate is a free parameter for microphone, because both server and client supports
// resampling. In contrary, for game audio, the server does not support resampling.
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct MicrophoneConfig {
    #[schema(strings(display_name = "设备"))]
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    pub devices: MicrophoneDevicesConfig,

    #[schema(strings(display_name = "缓冲"))]
    pub buffering: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    #[schema(strings(display_name = "头显扬声器"))]
    pub game_audio: Switch<GameAudioConfig>,

    #[cfg_attr(
        windows,
        schema(strings(
            display_name = "头显麦克风",
            display_name = "头显麦克风",
            notice = "要在 Windows 上使用麦克风，需要安装 Virtual Audio Cable"
        ))
    )]
    pub microphone: Switch<MicrophoneConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum HeadsetEmulationMode {
    #[schema(strings(display_name = "Rift S"))]
    RiftS,
    #[schema(strings(display_name = "Quest 1"))]
    Quest1,
    #[schema(strings(display_name = "Quest 2"))]
    Quest2,
    #[schema(strings(display_name = "Quest Pro"))]
    QuestPro,
    #[schema(strings(display_name = "Pico 4"))]
    Pico4,
    #[schema(strings(display_name = "Vive"))]
    Vive,
    #[schema(strings(display_name = "自定义"))]
    Custom {
        #[schema(strings(display_name = "序列号"))]
        serial_number: String,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, PartialEq, Clone)]
pub enum PerformanceLevel {
    #[schema(strings(display_name = "省电"))]
    PowerSavings,
    #[schema(strings(display_name = "持续低性能"))]
    SustainedLow,
    #[schema(strings(display_name = "持续高性能"))]
    SustainedHigh,
    #[schema(strings(display_name = "加速"))]
    #[schema(flag = "hidden")]
    Boost,
}

#[derive(SettingsSchema, Serialize, Deserialize, PartialEq, Clone)]
pub struct PerformanceLevelConfig {
    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "CPU",
        help = "关闭该项后，需要重启客户端才会生效。"
    ))]
    pub cpu: Switch<PerformanceLevel>,
    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "GPU",
        help = "关闭该项后，需要重启客户端才会生效。"
    ))]
    pub gpu: Switch<PerformanceLevel>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum FaceTrackingSourcesConfig {
    #[schema(strings(display_name = "优先仅眼动追踪"))]
    PreferEyeTrackingOnly,
    #[schema(strings(display_name = "优先完整面部追踪"))]
    PreferFullFaceTracking,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum FaceTrackingSocialPresenceSinkConfig {
    #[schema(strings(display_name = "VRChat Eye OSC"))]
    VrchatEyeOsc {
        #[schema(strings(display_name = "端口"))]
        port: u16,
    },
    #[schema(strings(display_name = "VRCFaceTracking"))]
    VrcFaceTracking,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct FaceTrackingSinkConfig {
    #[schema(strings(display_name = "社交数据转发", help = "把眼动与面部追踪数据转发给其他应用"))]
    pub social_presence: Switch<FaceTrackingSocialPresenceSinkConfig>,
    #[schema(strings(
        display_name = "眼动追踪中心凹编码",
        help = "使用眼动追踪移动中心凹区域。需要先在“视频 > 中心凹编码”中启用"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub eye_tracked_foveated_encoding: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct FaceTrackingConfig {
    #[schema(strings(display_name = "来源"))]
    pub sources: FaceTrackingSourcesConfig,
    #[schema(strings(display_name = "输出"))]
    pub sink: FaceTrackingSinkConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct BodyTrackingMetaConfig {
    #[schema(strings(display_name = "优先全身追踪"))]
    pub prefer_full_body: bool,
    #[schema(strings(display_name = "优先高保真", help = "优先使用主动上半身追踪，仅 Quest 3 支持"))]
    pub prefer_high_fidelity: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum BodyTrackingBDConfig {
    #[schema(strings(display_name = "身体追踪"))]
    BodyTracking {
        #[schema(strings(
            display_name = "高精度追踪",
            help = "提升追踪精度，代价是延迟增加。"
        ))]
        high_accuracy: bool,
        #[schema(strings(
            display_name = "启动时提示校准",
            help = "如果追踪器此前未校准，连接串流端后将自动开始校准流程。"
        ))]
        prompt_calibration_on_start: bool,
    },

    #[schema(strings(display_name = "物体追踪"))]
    ObjectTracking,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct BodyTrackingSourcesConfig {
    #[schema(strings(display_name = "Meta（Quest）"))]
    pub meta: BodyTrackingMetaConfig,
    #[schema(strings(display_name = "BD（Pico）"))]
    pub bd: BodyTrackingBDConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum BodyTrackingSinkConfig {
    #[schema(strings(display_name = "虚拟 Vive 追踪器"))]
    FakeViveTracker,
    #[schema(strings(display_name = "VRChat Body OSC"))]
    VrchatBodyOsc { port: u16 },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct BodyTrackingConfig {
    #[schema(strings(display_name = "来源"))]
    pub sources: BodyTrackingSourcesConfig,
    #[schema(strings(display_name = "输出"))]
    pub sink: BodyTrackingSinkConfig,
    #[schema(strings(display_name = "启用追踪", help = "关闭可临时暂停追踪。"))]
    #[schema(flag = "real-time")]
    pub tracked: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct VMCConfig {
    #[schema(strings(display_name = "主机"))]
    pub host: String,
    #[schema(strings(display_name = "端口"))]
    pub port: u16,
    #[schema(strings(display_name = "发送数据", help = "关闭可临时停止发送数据。"))]
    #[schema(flag = "real-time")]
    pub publish: bool,
    #[schema(strings(display_name = "朝向校正"))]
    #[schema(flag = "real-time")]
    pub orientation_correction: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ControllersEmulationMode {
    #[schema(strings(display_name = "Rift S Touch"))]
    RiftSTouch,
    #[schema(strings(display_name = "Quest 1 Touch"))]
    Quest1Touch,
    #[schema(strings(display_name = "Quest 2 Touch"))]
    Quest2Touch,
    #[schema(strings(display_name = "Quest 3 Touch Plus"))]
    Quest3Plus,
    #[schema(strings(display_name = "Quest Pro"))]
    QuestPro,
    #[schema(strings(display_name = "Pico 4"))]
    Pico4,
    #[schema(strings(display_name = "PSVR2 Sense Controller"))]
    PSVR2Sense,
    #[schema(strings(display_name = "Valve Index"))]
    ValveIndex,
    #[schema(strings(display_name = "Vive Wand"))]
    ViveWand,
    #[schema(strings(display_name = "Vive Tracker"))]
    ViveTracker,
    #[schema(strings(display_name = "自定义"))]
    Custom {
        #[schema(strings(display_name = "序列号"))]
        serial_number: String,
        #[schema(strings(display_name = "按键集合"))]
        button_set: Vec<String>,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct HysteresisThreshold {
    #[schema(strings(display_name = "阈值"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub value: f32,
    #[schema(strings(display_name = "回差"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub deviation: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct BinaryToScalarStates {
    #[schema(strings(display_name = "关状态值"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub off: f32,
    #[schema(strings(display_name = "开状态值"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub on: f32,
}

// Remaps 0..1 to custom range
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct Range {
    #[schema(strings(display_name = "最小值"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub min: f32,
    #[schema(strings(display_name = "最大值"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub max: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum ButtonMappingType {
    #[schema(strings(display_name = "直通"))]
    Passthrough,
    #[schema(strings(display_name = "迟滞阈值"))]
    HysteresisThreshold(HysteresisThreshold),
    #[schema(strings(display_name = "二值转标量"))]
    BinaryToScalar(BinaryToScalarStates),
    #[schema(strings(display_name = "重映射"))]
    Remap(Range),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ButtonBindingTarget {
    #[schema(strings(display_name = "目标路径"))]
    pub destination: String,
    #[schema(strings(display_name = "映射类型"))]
    pub mapping_type: ButtonMappingType,
    #[schema(strings(display_name = "二值条件"))]
    pub binary_conditions: Vec<String>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct AutomaticButtonMappingConfig {
    #[schema(strings(display_name = "点击阈值"))]
    pub click_threshold: HysteresisThreshold,
    #[schema(strings(display_name = "触摸阈值"))]
    pub touch_threshold: HysteresisThreshold,
    #[schema(strings(display_name = "力度阈值"))]
    pub force_threshold: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HandTrackingInteractionConfig {
    #[schema(strings(display_name = "仅触摸"))]
    #[schema(flag = "real-time")]
    pub only_touch: bool,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "捏合触摸距离",
        help = "指尖需要靠多近才算捏合点击。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)), suffix = "cm")]
    pub pinch_touch_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "捏合扳机距离",
        help = "指尖需要靠多近才开始触发扳机。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 2.5, step = 0.025)), suffix = "cm")]
    pub pinch_trigger_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "握拳触摸距离",
        help = "指尖需要靠手掌多近才算弯曲点击。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 5.0)), suffix = "cm")]
    pub curl_touch_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "握拳扳机距离",
        help = "指尖需要靠手掌多近才开始触发扳机。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 10.0)), suffix = "cm")]
    pub curl_trigger_distance: f32,

    #[schema(strings(display_name = "摇杆死区"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 100.0)), suffix = "%")]
    pub joystick_deadzone: f32,

    #[schema(strings(display_name = "摇杆水平偏移"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -5.0, max = 5.0)), suffix = "cm")]
    pub joystick_offset_horizontal: f32,

    #[schema(strings(display_name = "摇杆垂直偏移"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -5.0, max = 5.0)), suffix = "cm")]
    pub joystick_offset_vertical: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "摇杆活动范围",
        help = "摇杆的活动半径。拇指位于该范围的 2 倍以内时即可操作摇杆。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 5.0)), suffix = "cm")]
    pub joystick_range: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "激活延迟",
        help = "手势必须持续保持多久才会被激活。"
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub activation_delay: u32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "停用延迟",
        help = "手势必须持续松开多久才会被取消。"
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub deactivation_delay: u32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "重复延迟",
        help = "手势取消后需要等待多久才能再次激活。"
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub repeat_delay: u32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct HapticsConfig {
    #[schema(strings(display_name = "强度倍数"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 5.0, step = 0.1)))]
    pub intensity_multiplier: f32,

    #[schema(strings(display_name = "振幅曲线"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub amplitude_curve: f32,

    #[schema(strings(display_name = "最短持续时间"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 0.1, step = 0.001)), suffix = "s")]
    pub min_duration_s: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HandSkeletonConfig {
    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        display_name = "SteamVR 输入 2.0",
        help = "启用后会为手部追踪创建独立的追踪器对象，并使用完整骨骼层级。VRChat 手部追踪需要此项。"
    ))]
    pub steamvr_input_2_0: bool,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "预测骨骼",
        help = "对骨骼进行预测，减少拖影感。但可能让手部更抖动。"
    ))]
    pub predict: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct ControllersConfig {
    #[schema(strings(display_name = "启用追踪", help = "关闭后，控制器会显示为未开机状态。"))]
    #[schema(flag = "real-time")]
    pub tracked: bool,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        display_name = "手部骨骼",
        help = "启用后会把骨骼手部数据（手指追踪）传递给 SteamVR。"
    ))]
    pub hand_skeleton: Switch<HandSkeletonConfig>,

    #[schema(flag = "real-time")]
    #[schema(strings(
        display_name = "手部追踪交互",
        help = "启用后可以用手势模拟控制器输入。"
    ))]
    pub hand_tracking_interaction: Switch<HandTrackingInteractionConfig>,

    #[schema(strings(
        display_name = "预测",
        help = "数值越大，控制器追踪越平滑。\n技术上讲，这是从姿态提交给 SteamVR 到对应虚拟 vsync 发生之间的时间（以帧计）。\n目前无法可靠地自动估算。正确值应为 2，默认值为 3 以获得更平滑的追踪（代价是轻微滞后）。"
    ))]
    #[schema(gui(slider(min = 1.0, max = 10.0, logarithmic)), suffix = "frames")]
    pub steamvr_pipeline_frames: f32,

    #[schema(strings(display_name = "触觉反馈"))]
    #[schema(flag = "real-time")]
    pub haptics: Switch<HapticsConfig>,

    #[schema(strings(display_name = "仿真型号"))]
    #[schema(flag = "steamvr-restart")]
    pub emulation_mode: ControllersEmulationMode,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(display_name = "额外的 OpenVR 属性"))]
    pub extra_openvr_props: Vec<OpenvrProperty>,

    #[schema(flag = "real-time")]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(strings(display_name = "线速度截止值"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)), suffix = "m/s")]
    pub linear_velocity_cutoff: f32,

    #[schema(flag = "real-time")]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(strings(display_name = "角速度截止值"))]
    #[schema(gui(slider(min = 0.0, max = 100.0, step = 1.0)), suffix = "°/s")]
    pub angular_velocity_cutoff: f32,

    #[schema(flag = "real-time")]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(strings(
        display_name = "左控制器位置偏移",
        help = "右控制器的偏移会按水平方向镜像"
    ))]
    #[schema(gui(slider(min = -0.5, max = 0.5, step = 0.001)), suffix = "m")]
    pub left_controller_position_offset: [f32; 3],

    #[schema(flag = "real-time")]
    #[schema(strings(display_name = "左控制器旋转偏移", help = "右控制器的偏移会按水平方向镜像"))]
    #[schema(gui(slider(min = -180.0, max = 180.0, step = 1.0)), suffix = "°")]
    pub left_controller_rotation_offset: [f32; 3],

    #[schema(flag = "real-time")]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(strings(
        display_name = "左手追踪位置偏移",
        help = "右控制器的偏移会按水平方向镜像"
    ))]
    #[schema(gui(slider(min = -0.5, max = 0.5, step = 0.001)), suffix = "m")]
    pub left_hand_tracking_position_offset: [f32; 3],

    #[schema(flag = "real-time")]
    #[schema(strings(display_name = "左手追踪旋转偏移", help = "右控制器的偏移会按水平方向镜像"))]
    #[schema(gui(slider(min = -180.0, max = 180.0, step = 1.0)), suffix = "°")]
    pub left_hand_tracking_rotation_offset: [f32; 3],

    #[schema(strings(display_name = "按键映射", help = "OpenXR 风格的路径列表"))]
    pub button_mappings: Option<Vec<(String, Vec<ButtonBindingTarget>)>>,

    #[schema(strings(display_name = "自动按键映射配置"))]
    pub button_mapping_config: AutomaticButtonMappingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum RecenteringMode {
    #[schema(strings(display_name = "舞台"))]
    Stage,
    #[schema(strings(display_name = "本地地面"))]
    LocalFloor,
    #[schema(strings(display_name = "本地"))]
    Local {
        #[schema(strings(display_name = "视点高度"))]
        #[schema(gui(slider(min = 0.0, max = 3.0)), suffix = "m")]
        view_height: f32,
    },
    #[schema(strings(display_name = "倾斜"))]
    Tilted {
        #[schema(strings(display_name = "视点高度"))]
        #[schema(gui(slider(min = 0.0, max = 3.0)), suffix = "m")]
        view_height: f32,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct MultimodalTracking {
    #[schema(strings(display_name = "启用"))]
    pub enabled: bool,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        display_name = "把未握持的控制器映射为 SteamVR 追踪器",
        help = "未握持的控制器会被映射到左右脚。\n以后会做成可配置的。"
    ))]
    pub detached_controllers_steamvr_sink: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HeadsetConfig {
    #[schema(strings(
        display_name = "重新定位模式",
        help = "舞台：游玩空间原点由房间尺度的守护者设置决定。\n本地地面：原点位于地面，长按重新定位键时重置。\n本地：长按重新定位键时重置，原点按当前头部位置的偏移计算。\n倾斜：长按重新定位键时世界会倾斜，适合躺着使用 VR。"
    ))]
    #[schema(flag = "real-time")]
    pub recentering_mode: RecenteringMode,

    #[schema(strings(display_name = "控制器"))]
    #[schema(flag = "steamvr-restart")]
    pub controllers: Switch<ControllersConfig>,

    #[schema(strings(display_name = "仿真型号"))]
    #[schema(flag = "steamvr-restart")]
    pub emulation_mode: HeadsetEmulationMode,

    #[schema(strings(
        display_name = "性能等级",
        help = "省电可能增加延迟或降低帧率稳定性，但能降低温度并延长续航。\n持续低性能会保持帧率稳定，必要时可能增加延迟。\n持续高性能会保持帧率稳定，但温度更高。\n主要针对 Quest 头显，其他设备效果可能不同。"
    ))]
    pub performance_level: PerformanceLevelConfig,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(display_name = "额外的 OpenVR 属性"))]
    pub extra_openvr_props: Vec<OpenvrProperty>,

    #[schema(strings(display_name = "仅作为追踪参考系"))]
    #[schema(flag = "steamvr-restart")]
    pub tracking_ref_only: bool,

    #[schema(strings(display_name = "启用 Vive 追踪器代理"))]
    #[schema(flag = "steamvr-restart")]
    pub enable_vive_tracker_proxy: bool,

    #[schema(strings(display_name = "面部追踪"))]
    pub face_tracking: Switch<FaceTrackingConfig>,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        display_name = "多模态追踪",
        help = "在握持控制器时也追踪手部骨骼。这会把手部追踪频率降到 30Hz。\n由于运行时限制，身体追踪启用时该选项会被忽略。"
    ))]
    pub multimodal_tracking: Switch<MultimodalTracking>,

    #[schema(strings(display_name = "身体追踪"))]
    #[schema(flag = "steamvr-restart")]
    pub body_tracking: Switch<BodyTrackingConfig>,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(display_name = "VMC"))]
    pub vmc: Switch<VMCConfig>,

    #[schema(strings(
        display_name = "最大预测时间",
        help = "头部与控制器的最大预测时间。用于避免加载过程中出现过多抖动。"
    ))]
    #[schema(gui(slider(min = 0, max = 200, step = 5)), suffix = "ms")]
    pub max_prediction_ms: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
#[schema(gui = "button_group")]
pub enum SocketProtocol {
    #[schema(strings(display_name = "UDP"))]
    Udp,
    #[schema(strings(display_name = "TCP"))]
    Tcp,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct DiscoveryConfig {
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    #[schema(strings(
        display_name = "自动信任客户端",
        help = "允许未经确认的客户端直接连接。出于安全考虑不推荐。"
    ))]
    pub auto_trust_clients: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, Default)]
pub enum SocketBufferSize {
    #[schema(strings(display_name = "默认"))]
    #[default]
    Default,
    #[schema(strings(display_name = "最大"))]
    Maximum,
    #[schema(strings(display_name = "自定义"))]
    Custom(#[schema(suffix = "B")] u32),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy, Default)]
pub struct SocketBufferConfig {
    #[schema(strings(display_name = "发送缓冲区大小"))]
    pub send_size_bytes: SocketBufferSize,
    #[schema(strings(display_name = "接收缓冲区大小"))]
    pub recv_size_bytes: SocketBufferSize,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct WiredClientAutoLaunchConfig {
    #[schema(strings(
        display_name = "启动延迟",
        help = "头显开机后等待多少秒再尝试启动客户端。"
    ))]
    pub boot_delay: u32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    #[schema(strings(
        display_name = "串流协议",
        help = "UDP：速度更快，但不如 TCP 稳定。如果网络优化良好且没有干扰，可以试试。\nTCP：比 UDP 慢，但更稳定。如果使用 UDP 时出现画面或声音卡顿，请选择它。"
    ))]
    pub stream_protocol: SocketProtocol,

    #[schema(strings(display_name = "客户端发现"))]
    pub client_discovery: Switch<DiscoveryConfig>,

    #[schema(strings(
        display_name = "有线客户端类型",
        help = "建立有线连接时，ALVR 应寻找哪种发行版本的客户端。"
    ))]
    pub wired_client_type: ClientFlavor,

    #[schema(strings(
        display_name = "有线客户端自动启动",
        help = "建立有线连接时，ALVR 是否应尝试自动启动客户端。"
    ))]
    pub wired_client_autolaunch: Switch<WiredClientAutoLaunchConfig>,

    #[cfg_attr(
        windows,
        schema(strings(
            display_name = "启用连接脚本",
            help = "如果 session.json 同目录下存在 on_connect.bat，头显连接时会执行它。环境变量 ACTION 会被设为 `connect`。"
        ))
    )]
    #[cfg_attr(
        not(windows),
        schema(strings(
            display_name = "启用连接脚本",
            help = "如果 session.json 同目录下存在 on_connect.sh，头显连接时会执行它。环境变量 ACTION 会被设为 `connect`。"
        ))
    )]
    pub enable_on_connect_script: bool,

    #[cfg_attr(
        windows,
        schema(strings(
            display_name = "启用断开脚本",
            help = "如果 session.json 同目录下存在 on_disconnect.bat，头显断开时会执行它。环境变量 ACTION 会被设为 `disconnect`。"
        ))
    )]
    #[cfg_attr(
        not(windows),
        schema(strings(
            display_name = "启用断开脚本",
            help = "如果 session.json 同目录下存在 on_disconnect.sh，头显断开时会执行它。环境变量 ACTION 会被设为 `disconnect`。"
        ))
    )]
    #[schema(flag = "real-time")]
    pub enable_on_disconnect_script: bool,

    #[schema(strings(
        display_name = "允许不受信任的 HTTP",
        help = "允许跨源的浏览器请求远程控制 ALVR 设置。"
    ))]
    pub allow_untrusted_http: bool,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        display_name = "允许远程访问面板",
        help = "在所有网络接口上提供面板，而不是仅限本机。这样任何能访问本机的人都可以控制 ALVR，因为 Web 服务器没有认证。"
    ))]
    pub allow_remote_dashboard: bool,

    #[schema(strings(
        display_name = "避免画面错乱",
        help = "如果客户端、串流端或网络丢弃了一个数据包，则丢弃后续数据包，直到收到 IDR 包。"
    ))]
    pub avoid_video_glitching: bool,

    #[schema(strings(display_name = "数据包大小"))]
    #[schema(gui(slider(min = 1024, max = 65507, logarithmic)), suffix = "B")]
    pub packet_size: i32,

    #[schema(strings(display_name = "串流端口"))]
    pub stream_port: u16,
    #[schema(strings(display_name = "Web 服务端口"))]
    pub web_server_port: u16,

    #[schema(strings(display_name = "本地 OSC 端口"))]
    pub osc_local_port: u16,

    #[schema(strings(display_name = "串流端缓冲区"))]
    pub server_buffer_config: SocketBufferConfig,
    #[schema(strings(display_name = "客户端缓冲区"))]
    pub client_buffer_config: SocketBufferConfig,

    #[schema(strings(
        display_name = "串流端最大排队帧数",
        help = "如果串流端无法把视频包推送到网络，就会丢弃这些包（在 TCP 上可能发生）。此时会请求一个 IDR 帧。"
    ))]
    pub max_queued_server_video_frames: usize,

    #[schema(strings(display_name = "统计数据历史长度"))]
    #[schema(suffix = " frames")]
    pub statistics_history_size: usize,

    #[schema(strings(display_name = "最小 IDR 间隔"))]
    #[schema(flag = "steamvr-restart")]
    #[schema(gui(slider(min = 5, max = 1000, step = 5)), suffix = "ms")]
    pub minimum_idr_interval_ms: u64,

    #[schema(strings(display_name = "DSCP（数据包优先级）"))]
    pub dscp: Option<DscpTos>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
#[schema(gui = "button_group")]
pub enum DropProbability {
    #[schema(strings(display_name = "低"))]
    Low = 0x01,
    #[schema(strings(display_name = "中"))]
    Medium = 0x10,
    #[schema(strings(display_name = "高"))]
    High = 0x11,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum DscpTos {
    #[schema(strings(display_name = "尽力而为"))]
    BestEffort,

    #[schema(strings(display_name = "按类别选择"))]
    ClassSelector(#[schema(gui(slider(min = 1, max = 7)))] u8),

    #[schema(strings(display_name = "确保转发"))]
    AssuredForwarding {
        #[schema(strings(display_name = "类别"))]
        #[schema(gui(slider(min = 1, max = 4)))]
        class: u8,
        #[schema(strings(display_name = "丢弃概率"))]
        drop_probability: DropProbability,
    },

    #[schema(strings(display_name = "加速转发"))]
    ExpeditedForwarding,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct RawEventsConfig {
    #[schema(strings(display_name = "隐藏高频事件"))]
    #[schema(flag = "real-time")]
    pub hide_spammy_events: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    #[schema(strings(display_name = "显示通知小贴士", help = "通知小贴士会教你如何使用 ALVR"))]
    pub show_notification_tip: bool,

    #[schema(strings(display_name = "优先显示调用栈", help = "仅对部分错误或警告消息生效。"))]
    #[schema(flag = "steamvr-restart")]
    pub prefer_backtrace: bool,

    #[schema(strings(display_name = "通知级别"))]
    #[schema(flag = "real-time")]
    pub notification_level: LogSeverity,

    #[schema(strings(display_name = "客户端日志上报级别"))]
    pub client_log_report_level: Switch<LogSeverity>,

    #[schema(strings(display_name = "显示原始事件"))]
    #[schema(flag = "real-time")]
    pub show_raw_events: Switch<RawEventsConfig>,

    #[schema(strings(display_name = "写入日志文件", help = "把日志写入 session_log.txt 文件。"))]
    pub log_to_disk: bool,

    #[schema(strings(display_name = "记录追踪数据"))]
    #[schema(flag = "real-time")]
    pub log_tracking: bool,

    #[schema(strings(display_name = "记录按键"))]
    #[schema(flag = "real-time")]
    pub log_button_presses: bool,

    #[schema(strings(display_name = "记录触觉反馈"))]
    #[schema(flag = "real-time")]
    pub log_haptics: bool,

    #[cfg_attr(not(debug_assertions), schema(flag = "hidden"))]
    #[schema(strings(display_name = "调试日志分组", help = "这些设置会启用额外的、非常频繁的调试日志。"))]
    pub debug_groups: DebugGroupsConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct SteamvrLauncher {
    #[schema(strings(
        display_name = "自动启动和关闭 SteamVR",
        help = "打开 ALVR 面板时自动启动 SteamVR，关闭面板时关闭它。"
    ))]
    pub open_close_steamvr_with_dashboard: bool,

    #[cfg_attr(
        windows,
        schema(strings(
            display_name = "直接启动 VR 服务",
            help = "直接启动 VR 服务，绕过 Steam。如果 session.json 同目录下存在 start_server.bat 会执行它，否则尝试自动查找 SteamVR。"
        ))
    )]
    #[cfg_attr(
        not(windows),
        schema(strings(
            display_name = "直接启动 VR 服务",
            help = "直接启动 VR 服务，绕过 Steam。如果 session.json 同目录下存在 start_server.sh 会执行它，否则尝试自动查找 SteamVR。"
        ))
    )]
    pub direct_launch: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct RollingVideoFilesConfig {
    #[schema(strings(display_name = "时长"))]
    #[schema(suffix = "s")]
    pub duration_s: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct CaptureConfig {
    #[schema(strings(display_name = "客户端连接时开始录像"))]
    pub startup_video_recording: bool,

    #[schema(strings(display_name = "滚动录像文件"))]
    pub rolling_video_files: Switch<RollingVideoFilesConfig>,

    #[schema(strings(display_name = "截图输出目录"))]
    #[schema(flag = "steamvr-restart")]
    pub capture_frame_dir: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Patches {
    #[schema(strings(
        display_name = "Linux 异步计算",
        help = "SteamVR 中的异步计算目前有问题，请保持关闭。仅供测试。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub linux_async_compute: bool,
    #[schema(strings(
        display_name = "Linux 异步重投影",
        help = "异步重投影只有在始终能达到至少一半刷新率时才有效。",
    ))]
    #[schema(flag = "steamvr-restart")]
    pub linux_async_reprojection: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct NewVersionPopupConfig {
    #[schema(strings(display_name = "在该版本下不再提示"))]
    pub hide_while_version: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ExtraConfig {
    #[schema(strings(display_name = "SteamVR 启动器"))]
    pub steamvr_launcher: SteamvrLauncher,
    #[schema(strings(display_name = "采集"))]
    pub capture: CaptureConfig,
    #[schema(strings(display_name = "日志"))]
    pub logging: LoggingConfig,
    #[schema(strings(display_name = "实验性补丁"))]
    #[cfg_attr(not(target_os = "linux"), schema(flag = "hidden"))]
    pub patches: Patches,

    #[schema(
        strings(display_name = "速度倍率（调试）", help = "用于调试的线速度与角速度倍率。\n不会实时更新。")
    )]
    pub velocities_multiplier: f32,

    #[schema(strings(display_name = "打开设置向导"))]
    pub open_setup_wizard: bool,
    #[schema(strings(display_name = "新版本提示"))]
    pub new_version_popup: Switch<NewVersionPopupConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Settings {
    #[schema(strings(display_name = "视频"))]
    pub video: VideoConfig,
    #[schema(strings(display_name = "音频"))]
    pub audio: AudioConfig,
    #[schema(strings(display_name = "头显"))]
    pub headset: HeadsetConfig,
    #[schema(strings(display_name = "连接"))]
    pub connection: ConnectionConfig,
    #[schema(strings(display_name = "额外"))]
    pub extra: ExtraConfig,
}

pub fn session_settings_default() -> SettingsDefault {
    let view_resolution = FrameSizeDefault {
        variant: FrameSizeDefaultVariant::Absolute,
        Scale: 1.0,
        Absolute: FrameSizeAbsoluteDefault {
            width: 2144,
            height: OptionalDefault {
                set: false,
                content: 1072,
            },
        },
    };
    let default_custom_audio_device = CustomAudioDeviceConfigDefault {
        NameSubstring: "".into(),
        Index: 0,
        variant: CustomAudioDeviceConfigDefaultVariant::NameSubstring,
    };
    let default_custom_openvr_props = VectorDefault {
        gui_collapsed: true,
        element: OpenvrPropertyDefault {
            key: OpenvrPropKeyDefault {
                variant: OpenvrPropKeyDefaultVariant::TrackingSystemNameString,
            },
            value: "".into(),
        },
        content: vec![],
    };
    let socket_buffer = SocketBufferSizeDefault {
        Custom: 100000,
        variant: SocketBufferSizeDefaultVariant::Maximum,
    };
    let socket_buffer_config = SocketBufferConfigDefault {
        send_size_bytes: socket_buffer.clone(),
        recv_size_bytes: socket_buffer,
    };

    SettingsDefault {
        video: VideoConfigDefault {
            passthrough: SwitchDefault {
                enabled: false,
                content: PassthroughModeDefault {
                    variant: PassthroughModeDefaultVariant::Blend,
                    Blend: PassthroughModeBlendDefault {
                        premultiplied_alpha: true,
                        threshold: 0.5,
                    },
                    RgbChromaKey: RgbChromaKeyConfigDefault {
                        red: 0,
                        green: 255,
                        blue: 0,
                        distance_threshold: 85,
                        feathering: 0.05,
                    },
                    HsvChromaKey: HsvChromaKeyConfigDefault {
                        hue_start_max_deg: 70.0,
                        hue_start_min_deg: 80.0,
                        hue_end_min_deg: 160.0,
                        hue_end_max_deg: 170.0,
                        saturation_start_max: 0.2,
                        saturation_start_min: 0.3,
                        saturation_end_min: 1.0,
                        saturation_end_max: 1.1,
                        value_start_max: 0.0,
                        value_start_min: 0.1,
                        value_end_min: 1.0,
                        value_end_max: 1.1,
                    },
                },
            },
            clientside_post_processing: SwitchDefault {
                enabled: false,
                content: ClientsidePostProcessingConfigDefault {
                    super_sampling: ClientsidePostProcessingSuperSamplingModeDefault {
                        variant: ClientsidePostProcessingSuperSamplingModeDefaultVariant::Quality,
                    },
                    sharpening: ClientsidePostProcessingSharpeningModeDefault {
                        variant: ClientsidePostProcessingSharpeningModeDefaultVariant::Quality,
                    },
                },
            },
            upscaling: SwitchDefault {
                enabled: false,
                content: UpscalingConfigDefault {
                    edge_direction: true,
                    edge_threshold: 4.0,
                    edge_sharpness: 2.0,
                    upscale_factor: 1.5,
                },
            },
            adapter_index: 0,
            transcoding_view_resolution: view_resolution.clone(),
            emulated_headset_view_resolution: view_resolution,
            preferred_fps: 72.,
            max_buffering_frames: 2.0,
            buffering_history_weight: 0.90,
            enforce_server_frame_pacing: true,
            bitrate: BitrateConfigDefault {
                gui_collapsed: false,
                mode: BitrateModeDefault {
                    ConstantMbps: 30,
                    Adaptive: BitrateModeAdaptiveDefault {
                        gui_collapsed: true,
                        saturation_multiplier: 0.95,
                        max_throughput_mbps: SwitchDefault {
                            enabled: false,
                            content: 100,
                        },
                        min_throughput_mbps: SwitchDefault {
                            enabled: false,
                            content: 5,
                        },
                        max_network_latency_ms: SwitchDefault {
                            enabled: false,
                            content: 8,
                        },
                        encoder_latency_limiter: SwitchDefault {
                            enabled: true,
                            content: EncoderLatencyLimiterDefault {
                                max_saturation_multiplier: 0.9,
                            },
                        },
                        decoder_latency_limiter: SwitchDefault {
                            enabled: true,
                            content: DecoderLatencyLimiterDefault {
                                gui_collapsed: false,
                                max_decoder_latency_ms: 30,
                                latency_overstep_frames: 90,
                                latency_overstep_multiplier: 0.99,
                            },
                        },
                    },
                    variant: BitrateModeDefaultVariant::ConstantMbps,
                },
                adapt_to_framerate: SwitchDefault {
                    enabled: false,
                    content: BitrateAdaptiveFramerateConfigDefault {
                        framerate_reset_threshold_multiplier: 2.0,
                    },
                },
                history_size: 256,
                image_corruption_fix: false,
            },
            preferred_codec: CodecTypeDefault {
                variant: CodecTypeDefaultVariant::H264,
            },
            encoder_config: EncoderConfigDefault {
                gui_collapsed: true,
                rate_control_mode: RateControlModeDefault {
                    variant: RateControlModeDefaultVariant::Cbr,
                },
                filler_data: false,
                h264_profile: H264ProfileDefault {
                    variant: H264ProfileDefaultVariant::High,
                },
                entropy_coding: EntropyCodingDefault {
                    variant: EntropyCodingDefaultVariant::Cavlc,
                },
                use_10bit: OptionalDefault {
                    set: false,
                    content: false,
                },
                encoding_gamma: OptionalDefault {
                    set: false,
                    content: 1.0,
                },
                hdr: HDRConfigDefault {
                    gui_collapsed: true,
                    enable: OptionalDefault {
                        set: false,
                        content: false,
                    },
                    force_hdr_srgb_correction: false,
                    clamp_hdr_extended_range: false,
                },
                nvenc: NvencConfigDefault {
                    gui_collapsed: true,
                    quality_preset: EncoderQualityPresetNvidiaDefault {
                        variant: EncoderQualityPresetNvidiaDefaultVariant::P1,
                    },
                    tuning_preset: NvencTuningPresetDefault {
                        variant: NvencTuningPresetDefaultVariant::LowLatency,
                    },
                    multi_pass: NvencMultiPassDefault {
                        variant: NvencMultiPassDefaultVariant::QuarterResolution,
                    },
                    adaptive_quantization_mode: NvencAdaptiveQuantizationModeDefault {
                        variant: NvencAdaptiveQuantizationModeDefaultVariant::Spatial,
                    },
                    low_delay_key_frame_scale: -1,
                    refresh_rate: -1,
                    enable_intra_refresh: false,
                    intra_refresh_period: -1,
                    intra_refresh_count: -1,
                    max_num_ref_frames: -1,
                    gop_length: -1,
                    p_frame_strategy: -1,
                    rate_control_mode: -1,
                    rc_buffer_size: -1,
                    rc_initial_delay: -1,
                    rc_max_bitrate: -1,
                    rc_average_bitrate: -1,
                    enable_weighted_prediction: false,
                },
                quality_preset: EncoderQualityPresetDefault {
                    variant: EncoderQualityPresetDefaultVariant::Speed,
                },
                enable_vbaq: false,
                amf: AmfConfigDefault {
                    gui_collapsed: true,
                    enable_pre_analysis: false,
                    enable_hmqb: false,
                    use_preproc: false,
                    preproc_sigma: 4,
                    preproc_tor: 7,
                },
                software: SoftwareEncodingConfigDefault {
                    gui_collapsed: true,
                    force_software_encoding: false,
                    thread_count: 0,
                },
            },
            mediacodec_extra_options: {
                fn int32_default(int32: i32) -> MediacodecPropertyDefault {
                    MediacodecPropertyDefault {
                        ty: MediacodecPropTypeDefault {
                            variant: MediacodecPropTypeDefaultVariant::Int32,
                        },
                        value: int32.to_string(),
                    }
                }
                DictionaryDefault {
                    gui_collapsed: true,
                    key: "".into(),
                    value: int32_default(0),
                    content: vec![
                        ("operating-rate".into(), int32_default(i32::MAX)),
                        ("priority".into(), int32_default(0)),
                        // low-latency: only applicable on API level 30. Quest 1 and 2 might not be
                        // cabable, since they are on level 29.
                        // ("low-latency".into(), int32_default(1)), // Android smartphones crashes enabling this feature (https://github.com/PhoneVR-Developers/alvr-cardboard/issues/5)
                        (
                            "vendor.qti-ext-dec-low-latency.enable".into(),
                            int32_default(1),
                        ),
                    ],
                }
            },
            foveated_encoding: SwitchDefault {
                enabled: true,
                content: FoveatedEncodingConfigDefault {
                    gui_collapsed: true,
                    force_enable: false,
                    center_size: ArrayDefault {
                        gui_collapsed: false,
                        content: [0.45, 0.4],
                    },
                    center_shift: ArrayDefault {
                        gui_collapsed: false,
                        content: [0.4, 0.1],
                    },
                    edge_ratio: ArrayDefault {
                        gui_collapsed: false,
                        content: [4., 5.],
                    },
                },
            },
            clientside_foveation: SwitchDefault {
                enabled: false,
                content: ClientsideFoveationConfigDefault {
                    mode: ClientsideFoveationModeDefault {
                        Static: ClientsideFoveationModeStaticDefault {
                            level: ClientsideFoveationLevelDefault {
                                variant: ClientsideFoveationLevelDefaultVariant::High,
                            },
                        },
                        Dynamic: ClientsideFoveationModeDynamicDefault {
                            max_level: ClientsideFoveationLevelDefault {
                                variant: ClientsideFoveationLevelDefaultVariant::High,
                            },
                        },
                        variant: ClientsideFoveationModeDefaultVariant::Dynamic,
                    },
                    vertical_offset_deg: 0.0,
                },
            },
            force_software_decoder: false,
            color_correction: SwitchDefault {
                enabled: false,
                content: ColorCorrectionConfigDefault {
                    brightness: 0.,
                    contrast: 0.,
                    saturation: 0.5,
                    gamma: 1.,
                    sharpening: 0.5,
                },
            },
        },
        audio: AudioConfigDefault {
            game_audio: SwitchDefault {
                enabled: true,
                content: GameAudioConfigDefault {
                    gui_collapsed: true,
                    device: OptionalDefault {
                        set: false,
                        content: default_custom_audio_device.clone(),
                    },
                    mute_when_streaming: true,
                    buffering: AudioBufferingConfigDefault {
                        gui_collapsed: true,
                        average_buffering_ms: 50,
                        batch_ms: 10,
                    },
                },
            },
            microphone: SwitchDefault {
                enabled: cfg!(target_os = "linux"),
                content: MicrophoneConfigDefault {
                    gui_collapsed: true,
                    devices: MicrophoneDevicesConfigDefault {
                        Custom: MicrophoneDevicesConfigCustomDefault {
                            source: default_custom_audio_device.clone(),
                            sink: default_custom_audio_device,
                        },
                        variant: MicrophoneDevicesConfigDefaultVariant::Automatic,
                    },
                    buffering: AudioBufferingConfigDefault {
                        gui_collapsed: true,
                        average_buffering_ms: 50,
                        batch_ms: 10,
                    },
                },
            },
        },
        headset: HeadsetConfigDefault {
            emulation_mode: HeadsetEmulationModeDefault {
                Custom: HeadsetEmulationModeCustomDefault {
                    serial_number: "Unknown".into(),
                },
                variant: HeadsetEmulationModeDefaultVariant::Quest2,
            },
            performance_level: PerformanceLevelConfigDefault {
                cpu: SwitchDefault {
                    enabled: false,
                    content: PerformanceLevelDefault {
                        variant: PerformanceLevelDefaultVariant::PowerSavings,
                    },
                },
                gpu: SwitchDefault {
                    enabled: false,
                    content: PerformanceLevelDefault {
                        variant: PerformanceLevelDefaultVariant::PowerSavings,
                    },
                },
            },
            extra_openvr_props: default_custom_openvr_props.clone(),
            tracking_ref_only: false,
            enable_vive_tracker_proxy: false,
            face_tracking: SwitchDefault {
                enabled: false,
                content: FaceTrackingConfigDefault {
                    gui_collapsed: true,
                    sources: FaceTrackingSourcesConfigDefault {
                        variant: FaceTrackingSourcesConfigDefaultVariant::PreferFullFaceTracking,
                    },
                    sink: FaceTrackingSinkConfigDefault {
                        social_presence: SwitchDefault {
                            enabled: true,
                            content: FaceTrackingSocialPresenceSinkConfigDefault {
                                VrchatEyeOsc:
                                    FaceTrackingSocialPresenceSinkConfigVrchatEyeOscDefault {
                                        port: 9000,
                                    },
                                variant:
                                    FaceTrackingSocialPresenceSinkConfigDefaultVariant::VrchatEyeOsc,
                            },
                        },
                        eye_tracked_foveated_encoding: false,
                    },
                },
            },
            multimodal_tracking: SwitchDefault {
                enabled: false,
                content: MultimodalTrackingDefault {
                    enabled: true,
                    detached_controllers_steamvr_sink: false,
                },
            },
            body_tracking: SwitchDefault {
                enabled: false,
                content: BodyTrackingConfigDefault {
                    gui_collapsed: true,
                    sources: BodyTrackingSourcesConfigDefault {
                        meta: BodyTrackingMetaConfigDefault {
                            prefer_full_body: true,
                            prefer_high_fidelity: true,
                        },
                        bd: BodyTrackingBDConfigDefault {
                            BodyTracking: BodyTrackingBDConfigBodyTrackingDefault {
                                high_accuracy: true,
                                prompt_calibration_on_start: true,
                            },
                            variant: BodyTrackingBDConfigDefaultVariant::BodyTracking,
                        },
                    },
                    sink: BodyTrackingSinkConfigDefault {
                        VrchatBodyOsc: BodyTrackingSinkConfigVrchatBodyOscDefault { port: 9000 },
                        variant: BodyTrackingSinkConfigDefaultVariant::FakeViveTracker,
                    },
                    tracked: true,
                },
            },
            vmc: SwitchDefault {
                enabled: false,
                content: VMCConfigDefault {
                    gui_collapsed: true,
                    host: "127.0.0.1".into(),
                    port: 39539,
                    publish: true,
                    orientation_correction: true,
                },
            },
            controllers: SwitchDefault {
                enabled: true,
                content: ControllersConfigDefault {
                    gui_collapsed: false,
                    tracked: true,
                    hand_skeleton: SwitchDefault {
                        enabled: true,
                        content: HandSkeletonConfigDefault {
                            steamvr_input_2_0: true,
                            predict: false,
                        },
                    },
                    emulation_mode: ControllersEmulationModeDefault {
                        Custom: ControllersEmulationModeCustomDefault {
                            serial_number: "ALVR Controller".into(),
                            button_set: VectorDefault {
                                gui_collapsed: false,
                                element: "/user/hand/left/input/a/click".into(),
                                content: vec![],
                            },
                        },
                        variant: ControllersEmulationModeDefaultVariant::Quest2Touch,
                    },
                    extra_openvr_props: default_custom_openvr_props,
                    button_mappings: OptionalDefault {
                        set: false,
                        content: DictionaryDefault {
                            gui_collapsed: false,
                            key: "/user/hand/left/input/a/click".into(),
                            value: VectorDefault {
                                gui_collapsed: false,
                                element: ButtonBindingTargetDefault {
                                    destination: "/user/hand/left/input/a/click".into(),
                                    mapping_type: ButtonMappingTypeDefault {
                                        HysteresisThreshold: HysteresisThresholdDefault {
                                            value: 0.5,
                                            deviation: 0.05,
                                        },
                                        BinaryToScalar: BinaryToScalarStatesDefault {
                                            off: 0.0,
                                            on: 1.0,
                                        },
                                        Remap: RangeDefault { min: 0.0, max: 1.0 },
                                        variant: ButtonMappingTypeDefaultVariant::Passthrough,
                                    },
                                    binary_conditions: VectorDefault {
                                        gui_collapsed: true,
                                        element: "/user/hand/left/input/trigger/touch".into(),
                                        content: vec![],
                                    },
                                },
                                content: vec![],
                            },
                            content: vec![],
                        },
                    },
                    button_mapping_config: AutomaticButtonMappingConfigDefault {
                        gui_collapsed: true,
                        click_threshold: HysteresisThresholdDefault {
                            value: 0.5,
                            deviation: 0.05,
                        },
                        touch_threshold: HysteresisThresholdDefault {
                            value: 0.1,
                            deviation: 0.05,
                        },
                        force_threshold: 0.8,
                    },
                    hand_tracking_interaction: SwitchDefault {
                        enabled: false,
                        content: HandTrackingInteractionConfigDefault {
                            only_touch: false,
                            pinch_touch_distance: 0.0,
                            pinch_trigger_distance: 0.25,
                            curl_touch_distance: 2.0,
                            curl_trigger_distance: 2.5,
                            joystick_deadzone: 40.0,
                            joystick_offset_horizontal: 0.0,
                            joystick_offset_vertical: 0.0,
                            joystick_range: 1.0,
                            repeat_delay: 100,
                            activation_delay: 50,
                            deactivation_delay: 100,
                        },
                    },
                    steamvr_pipeline_frames: 2.1,
                    linear_velocity_cutoff: 0.05,
                    angular_velocity_cutoff: 10.0,
                    left_controller_position_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0, 0.0, -0.11],
                    },
                    left_controller_rotation_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0; 3],
                    },
                    left_hand_tracking_position_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.04, -0.02, -0.13],
                    },
                    left_hand_tracking_rotation_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0, -45.0, -90.0],
                    },
                    haptics: SwitchDefault {
                        enabled: true,
                        content: HapticsConfigDefault {
                            gui_collapsed: true,
                            intensity_multiplier: 1.0,
                            amplitude_curve: 1.0,
                            min_duration_s: 0.01,
                        },
                    },
                },
            },
            recentering_mode: RecenteringModeDefault {
                Local: RecenteringModeLocalDefault { view_height: 1.5 },
                Tilted: RecenteringModeTiltedDefault { view_height: 1.5 },
                variant: RecenteringModeDefaultVariant::LocalFloor,
            },
            max_prediction_ms: 100,
        },
        connection: ConnectionConfigDefault {
            stream_protocol: SocketProtocolDefault {
                variant: SocketProtocolDefaultVariant::Udp,
            },
            client_discovery: SwitchDefault {
                enabled: true,
                content: DiscoveryConfigDefault {
                    auto_trust_clients: cfg!(debug_assertions),
                },
            },
            wired_client_type: ClientFlavorDefault {
                Custom: "alvr.client".to_owned(),
                variant: if alvr_common::is_stable() {
                    ClientFlavorDefaultVariant::Store
                } else {
                    ClientFlavorDefaultVariant::Github
                },
            },
            wired_client_autolaunch: SwitchDefault {
                enabled: true,
                content: WiredClientAutoLaunchConfigDefault { boot_delay: 0 },
            },
            web_server_port: 8082,
            stream_port: 9944,
            osc_local_port: 9942,
            dscp: OptionalDefault {
                set: false,
                content: DscpTosDefault {
                    ClassSelector: 7,
                    AssuredForwarding: DscpTosAssuredForwardingDefault {
                        class: 4,
                        drop_probability: DropProbabilityDefault {
                            variant: DropProbabilityDefaultVariant::Low,
                        },
                    },
                    variant: DscpTosDefaultVariant::ExpeditedForwarding,
                },
            },
            server_buffer_config: socket_buffer_config.clone(),
            client_buffer_config: socket_buffer_config,
            max_queued_server_video_frames: 1024,
            avoid_video_glitching: false,
            minimum_idr_interval_ms: 100,
            enable_on_connect_script: false,
            enable_on_disconnect_script: false,
            allow_untrusted_http: false,
            allow_remote_dashboard: false,
            packet_size: 1400,
            statistics_history_size: 256,
        },
        extra: ExtraConfigDefault {
            logging: LoggingConfigDefault {
                client_log_report_level: SwitchDefault {
                    enabled: true,
                    content: LogSeverityDefault {
                        variant: LogSeverityDefaultVariant::Error,
                    },
                },
                log_to_disk: cfg!(debug_assertions),
                log_button_presses: false,
                log_tracking: false,
                log_haptics: false,
                notification_level: LogSeverityDefault {
                    variant: if cfg!(debug_assertions) {
                        LogSeverityDefaultVariant::Info
                    } else {
                        LogSeverityDefaultVariant::Warning
                    },
                },
                show_raw_events: SwitchDefault {
                    enabled: false,
                    content: RawEventsConfigDefault {
                        hide_spammy_events: false,
                    },
                },
                prefer_backtrace: false,
                show_notification_tip: true,
                debug_groups: DebugGroupsConfigDefault {
                    server_impl: false,
                    client_impl: false,
                    server_core: false,
                    client_core: false,
                    connection: false,
                    sockets: false,
                    server_gfx: false,
                    client_gfx: false,
                    encoder: false,
                    decoder: false,
                },
            },
            steamvr_launcher: SteamvrLauncherDefault {
                open_close_steamvr_with_dashboard: false,
                direct_launch: false,
            },
            capture: CaptureConfigDefault {
                startup_video_recording: false,
                rolling_video_files: SwitchDefault {
                    enabled: false,
                    content: RollingVideoFilesConfigDefault { duration_s: 5 },
                },
                capture_frame_dir: if !cfg!(target_os = "linux") {
                    "/tmp".into()
                } else {
                    "".into()
                },
            },
            patches: PatchesDefault {
                linux_async_compute: false,
                linux_async_reprojection: false,
            },
            velocities_multiplier: 1.0,
            open_setup_wizard: alvr_common::is_stable() || alvr_common::is_nightly(),
            new_version_popup: SwitchDefault {
                enabled: alvr_common::is_stable(),
                content: NewVersionPopupConfigDefault {
                    hide_while_version: ALVR_VERSION.to_string(),
                },
            },
        },
    }
}
