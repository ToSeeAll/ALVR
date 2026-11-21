use alvr_common::{LogEntry, LogSeverity};
use alvr_gui_common::theme::{self, log_colors};
use alvr_session::Settings;
use eframe::{
    egui::{self, Frame, Label, Layout, Panel, RichText, TextWrapMode, Ui},
    emath::Align,
    epaint::Color32,
};
use rand::seq::IndexedRandom;
use std::time::Duration;

#[cfg(target_arch = "wasm32")]
use instant::Instant;
#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

const TIMEOUT: Duration = Duration::from_secs(5);
const NO_NOTIFICATIONS_MESSAGE: &str = "暂无新通知";
const NOTIFICATION_TIPS: &[&str] = &[
    // The following tips are ordered roughtly in the order settings appear
    r#"如果在修改某些设置后开始崩溃，请到“安装”标签页点击“运行设置向导”，再点击“重置设置”来重置 ALVR。"#,
    r#"部分设置默认是折叠的。点击设置旁的“展开”按钮即可展开子菜单。"#,
    r#"强烈建议保持 ALVR 的音频设置为默认值，改为在任务栏托盘中修改系统默认音频设备。"#,
    r#"提高“视频”→“最大缓冲”可以减少卡顿，但会增加延迟。"#,
    r#"在某些显卡上，有时需要在 h264 与 HEVC 编码之间切换，才能解决崩溃或回退到软件编码的问题。"#,
    r#"使用 NVIDIA 显卡时建议用高码率 H264；使用 AMD 显卡时 HEVC 画质可能更好。"#,
    r#"如果画面出现“雪花”闪烁，请把“预设”→“分辨率”设为“低”，并关闭“视频”→“中心凹编码”。"#,
    r#"提高“视频”→“色彩校正”→“锐化”可以改善主观画质。"#,
    r#"如果外接控制器或追踪器无法与 ALVR 的追踪空间对齐，请在“头显”→“额外的 OpenVR 属性”里添加一项，并自定义“追踪系统名称字符串”。"#,
    r#"要改变控制器在外观上的显示，请设置“头显”→“控制器”→“仿真型号”。"#,
    r#"ALVR 支持自定义按键绑定！如需帮助，请到我们的 Discord 服务器提问。"#,
    r#"ALVR 支持手部追踪手势（“预设”→“手部追踪交互”→“ALVR 绑定”）。用法见 wiki：https://github.com/alvr-org/ALVR/wiki/Hand-tracking-controller-bindings。"#,
    r#"如果手部追踪手势很烦人，可以在“头显”→“控制器”→“手部追踪交互”中关闭；也可以只启用其中的“仅触摸”。"#,
    r#"可以在“头显”→“控制器”→“预测”中微调控制器的响应速度。"#,
    r#"如果控制器/手部模型与实体控制器位置不一致，可以在“头显”→“控制器”→“左控制器位置/旋转偏移”中调整（会同时影响左右两只控制器）。"#,
    r#"使用外部追踪器或控制器时，应把“头显”中的“位置重新定位模式”和“旋转重新定位模式”都设为“禁用”。"#,
    r#"可以开启倾斜模式：把“头显”→“位置重新定位模式”设为“本地”，把“旋转重新定位模式”设为“倾斜”。"#,
    r#"如果经常出现画面错乱，可以用“连接”→“避免画面错乱”来换取更多卡顿帧。"#,
    r#"可以用“连接”→“启用连接/断开脚本”在头显连接或断开时运行自定义命令或程序。"#,
    r#"如果需要反馈问题，请开启“额外”→“日志”→“写入日志文件”以获取日志。日志位于“session_log.txt”。"#,
    r#"做逆向分析时，可以开启“额外”→“日志”→“记录追踪”、“记录按键”和“记录触觉反馈”，并通过 ws://localhost:8082/api/events 这个 websocket 获取数据。"#,
    r#"如果需要反馈问题并分享日志，建议开启“额外”→“日志”→“优先显示调用栈”。"#,
    r#"开关“额外”→“日志”→“显示通知小贴士”，即可像现在这样快速轮换浏览提示。"#,
    r#"建议开启“额外”→“SteamVR 启动器”→“自动启动和关闭 SteamVR”。"#,
    r#"如果要分享录像来反馈问题，可以开启“额外”→“采集”→“滚动录像文件”来限制上传文件的大小。"#,
    // Miscellaneous
    r#"如果头显没有出现在设备列表中，它可能位于另一个网段。可以用“手动添加设备”，填入头显内显示的 IP。"#,
];

pub struct NotificationBar {
    message: String,
    current_level: LogSeverity,
    receive_instant: Instant,
    min_notification_level: LogSeverity,
    tip_message: Option<String>,
    expanded: bool,
}

impl NotificationBar {
    pub fn new() -> Self {
        Self {
            message: NO_NOTIFICATIONS_MESSAGE.into(),
            current_level: LogSeverity::Debug,
            receive_instant: Instant::now(),
            min_notification_level: LogSeverity::Debug,
            tip_message: None,
            expanded: false,
        }
    }

    pub fn update_settings(&mut self, settings: &Settings) {
        self.min_notification_level = settings.extra.logging.notification_level;

        if settings.extra.logging.show_notification_tip {
            if self.tip_message.is_none() {
                self.tip_message = NOTIFICATION_TIPS
                    .choose(&mut rand::rng())
                    .map(|s| format!("小贴士：{s}"));
            }
        } else {
            self.tip_message = None;
        }
    }

    pub fn push_notification(&mut self, event: LogEntry, from_dashboard: bool) {
        let now = Instant::now();
        let min_severity = if from_dashboard {
            if cfg!(debug_assertions) {
                LogSeverity::Debug
            } else {
                LogSeverity::Info
            }
        } else {
            self.min_notification_level
        };

        if event.severity >= min_severity
            && (now > self.receive_instant + TIMEOUT || event.severity >= self.current_level)
        {
            self.message = event.content;
            self.current_level = event.severity;
            self.receive_instant = now;
        }
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let now = Instant::now();
        if now > self.receive_instant + TIMEOUT {
            self.message = self
                .tip_message
                .clone()
                .unwrap_or_else(|| NO_NOTIFICATIONS_MESSAGE.into());
            self.current_level = LogSeverity::Debug;
        }

        let (fg, bg) = match self.current_level {
            LogSeverity::Error => (Color32::BLACK, log_colors::ERROR_LIGHT),
            LogSeverity::Warning => (Color32::BLACK, log_colors::WARNING_LIGHT),
            LogSeverity::Info => (Color32::BLACK, log_colors::INFO_LIGHT),
            LogSeverity::Debug => (theme::FG, theme::LIGHTER_BG),
        };

        let mut bottom_bar = Panel::bottom("bottom_panel").frame(
            Frame::default()
                .inner_margin(egui::vec2(10.0, 5.0))
                .fill(bg),
        );
        let alignment = if !self.expanded {
            bottom_bar = bottom_bar.max_size(26.0);

            Align::TOP
        } else {
            Align::Center
        };
        let wrapping = if !self.expanded {
            TextWrapMode::Truncate
        } else {
            TextWrapMode::Wrap
        };

        bottom_bar.show(ui, |ui| {
            ui.with_layout(Layout::right_to_left(alignment), |ui| {
                if !self.expanded {
                    if ui.small_button("展开").clicked() {
                        self.expanded = true;
                    }
                } else if ui.button("收起").clicked() {
                    self.expanded = false;
                }
                ui.with_layout(Layout::left_to_right(alignment), |ui| {
                    //A LayoutJob that has its TextWrapping updated to fill the available space would probably be a more elegant solution.
                    ui.add(
                        Label::new(RichText::new(&self.message).color(fg).size(12.0))
                            .wrap_mode(wrapping),
                    );
                })
            })
        });
    }
}
