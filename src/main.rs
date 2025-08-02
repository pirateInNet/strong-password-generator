use arboard::Clipboard;
use eframe::egui::{self, FontFamily, FontId, Slider, TextStyle};
use random_string::generate;

#[derive(Default)]
struct MyApp {
    // variable that count (all the states wrotten here)
    password: String,
    length: usize,
    nums: bool,
    upper: bool,
    lower: bool,
    symbols: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // to make the widgets looks larger
        let mut style = (*ctx.style()).clone();
        style.text_styles = [
            (
                TextStyle::Heading,
                FontId::new(40.0, FontFamily::Proportional),
            ),
            (TextStyle::Body, FontId::new(16.0, FontFamily::Monospace)),
            (
                TextStyle::Button,
                FontId::new(16.0, FontFamily::Proportional),
            ),
        ]
        .into();
        style.spacing.item_spacing = egui::vec2(10.0, 25.0);
        style.spacing.button_padding = egui::vec2(10.0, 10.0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Password Generator");
            });
            ui.add(Slider::new(&mut self.length, 8..=128).text("length"));
            ui.checkbox(&mut self.nums, "numbers");
            ui.checkbox(&mut self.upper, "uppercase");
            ui.checkbox(&mut self.lower, "lowercase");
            ui.checkbox(&mut self.symbols, "symbols");
            ui.vertical_centered(|ui| {
                if ui.button("generate & copy").clicked() {
                    self.password =
                        gen_password(self.nums, self.upper, self.lower, self.symbols, self.length);
                    copy_password(&self.password);
                };
                ui.label(format!("{}", self.password));
            });
        });
    }
}

fn gen_password(nums: bool, upper: bool, lower: bool, symbols: bool, len: usize) -> String {
    let mut chars = String::from("");
    if nums {
        chars += "0123456789";
    }
    if upper {
        chars += "AZERTYUIOPQSDFGHJKLMWXCVBN";
    }
    if lower {
        chars += "azertyuiopsdfghjklmwxcvbn";
    }
    if symbols {
        chars += "/*.+-;:!,?.*%&~#'\"[]()-|`{}@_";
    }
    if !nums && !upper && !lower && !symbols {
        return "my twitter -> @pirateinnet".to_string();
    }
    generate(len, chars)
}

fn copy_password(password: &String) {
    let mut clipboard = Clipboard::new().unwrap();
    clipboard.set_text(password).unwrap();
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::viewport::ViewportBuilder::default()
            .with_inner_size([440.0, 510.0]) // width x height
            .with_min_inner_size([410.0, 510.0])
            .with_resizable(true),
        ..Default::default()
    };
    eframe::run_native(
        "Password Generator(by Youness BenDaoud)",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}
