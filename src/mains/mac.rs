#[cfg(target_os = "macos")]
pub fn mac() -> eframe::Result<()> {
    use crate::app;
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions {
        active: true,
        app_id: Some("type".to_owned()),
        fullsize_content: false,
        always_on_top: false,
        maximized: false,
        decorated: true,
        drag_and_drop_support: true,
        icon_data: None,
        initial_window_pos: Option::from(egui::Pos2::new(300_f32, 100_f32)),
        initial_window_size: Option::from(egui::Vec2::new(1000_f32, 1000_f32)),
        min_window_size: None,
        max_window_size: None,
        resizable: true,
        transparent: true,
        vsync: true,
        multisampling: 0,
        depth_buffer: 0,
        stencil_buffer: 0,
        fullscreen: false,
        mouse_passthrough: Default::default(),
        hardware_acceleration: eframe::HardwareAcceleration::Preferred,
        renderer: eframe::Renderer::Glow,
        follow_system_theme: Default::default(),
        default_theme: eframe::Theme::Dark,
        run_and_return: Default::default(),
        event_loop_builder: Default::default(),
        shader_version: Default::default(),
        centered: true,
    };
    eframe::run_native(
        "hurlalot",
        native_options,
        Box::new(|cc| Box::new(TypeFastApp::new(cc))),
    )
}
