use eframe::{egui, App};

struct Cards {}

impl App for Cards {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Gart Card Generator - Rust Rewrite by Hdert");
                egui::Frame::none()
                    .fill(egui::Color32::BLACK)
                    .show(ui, |ui| ui.label("Funny haha Card"));
                if ui.button("New Prompt").clicked() {
                    ui.label("clicked!");
                };
            });
        });

        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("© gareth,");
                ui.heading("applicable parts © hdert");
                ui.small("sorry safari/iPhone users")
            })
        });
    }

    // fn name(&self) -> &str {
    //     "Gart Card"
    // }
}

impl Cards {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Cards {}
    }
}

fn main() {
    let win_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cards Against gart - Rust rewrite by hdert",
        win_options,
        Box::new(|cc| Box::new(Cards::new(cc))),
    );
}
