use crate::widgets;

pub fn homepage(ctx: &egui::Context, ui: &mut egui::Ui) {
    // The central panel the region left after adding TopPanel's and SidePanel's
    ui.heading("aniruddh doki");

    ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
        widgets::powered_by_egui_and_eframe::powered_by_egui_and_eframe(ui);
        ui.add(egui::github_link_file!(
            "https://github.com/aniruddhdoki/aniruddh.dev",
            "Source code."
        ));
        egui::warn_if_debug_build(ui);
    });
}