mod card;

use card::Card;
use eframe::{
    egui::{CentralPanel, Color32, Context, Frame, Hyperlink, Label, RichText, TopBottomPanel},
    App,
};

struct CardsApp {
    card: Card,
}

impl App for CardsApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                ui.vertical_centered(|ui| {
                    ui.heading("Gart Card Generator");
                    ui.spacing_mut().item_spacing.x = 0.0;
                    ui.add(Label::new("Rust Rewrite by "));
                    ui.add(Hyperlink::from_label_and_url(
                        "Hdert",
                        "https://github.com/hdert/",
                    ))
                });
            });
        });
        CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                Frame::none().fill(Color32::BLACK).show(ui, |ui| {
                    // ui.add_sized([66., 83.], ui.vertical_centered(|ui| Label::new("text")));
                    ui.horizontal_wrapped(|ui| {
                        ui.vertical_centered(|ui| {
                            ui.spacing_mut().item_spacing.x = 0.0;
                            if self.card.prompt == " (card reader's) favorite website _______." {
                                ui.add(Hyperlink::from_label_and_url(
                                    "This is",
                                    "https://ngtgyu.hdert.com/",
                                ));
                            }
                            ui.label(self.card.prompt)
                        });
                    });
                });
                if ui.button("New Prompt").clicked() {
                    self.card.increment_iterator();
                };
            });
        });

        TopBottomPanel::bottom("footer").show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add(Hyperlink::from_label_and_url(
                    RichText::new("© gareth,").monospace(),
                    "https://github.com/GsnailG/",
                ));
                ui.add(Label::new(
                    RichText::new("applicable parts MIT licensed").monospace(),
                ));
            })
        });
    }
}

impl CardsApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        CardsApp { card: Card::new() }
    }
}

fn main() {
    let win_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Cards Against gart - Rust rewrite by hdert",
        win_options,
        Box::new(|cc| Box::new(CardsApp::new(cc))),
    );
}
