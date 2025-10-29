// & creates a reference to borrow a value (read)
// mut makes a reference mutable, which allows us to modify the original value (modify)
// &mut creates a reference to borrow a value and mutates it (read & write)
pub fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Written with");
        ui.hyperlink_to(" egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
