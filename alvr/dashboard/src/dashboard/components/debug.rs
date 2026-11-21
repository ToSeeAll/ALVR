use crate::dashboard::ServerRequest;
use eframe::egui::Ui;

pub fn debug_tab_ui(ui: &mut Ui) -> Option<ServerRequest> {
    let mut request = None;

    ui.label(
        "这里的录制按钮不适合用来录制游戏画面。\n如需录制游戏，请使用其他方式，例如通过头显或桌面的 VR 输出。",
    );

    ui.columns(4, |ui| {
        if ui[0].button("抓取单帧").clicked() {
            request = Some(ServerRequest::CaptureFrame);
        }

        if ui[1].button("插入 IDR 帧").clicked() {
            request = Some(ServerRequest::InsertIdr);
        }

        if ui[2].button("开始录制").clicked() {
            request = Some(ServerRequest::StartRecording);
        }

        if ui[3].button("停止录制").clicked() {
            request = Some(ServerRequest::StopRecording);
        }
    });

    request
}
