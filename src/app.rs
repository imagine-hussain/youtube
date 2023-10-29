use egui::Ui;
use egui_dock::{DockArea, DockState, NodeIndex, TabViewer};

///
/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    left_shown: bool,
    #[serde(skip)]
    dock_state: DockState<Box<dyn Tab>>,
}

impl Default for TemplateApp {
    fn default() -> Self {
        let dock_state = DockState::new(Vec::new());

        Self {
            left_shown: true,
            dock_state,
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        match cc.storage {
            Some(storage) => eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default(),
            None => Default::default(),
        }
    }
}

impl TemplateApp {
    // Menu Bar
    fn menu_bar(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }
                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        self.menu_bar(ctx, frame);
        // Open a window

        // create a toggleable right panel
        // egui::SidePanel::left("left-panel").show_animated(ctx, self.left_shown, |ui| {
        //     ui.heading("Views");
        //     // egui::SidePanel::left("button").show(ctx, add_contents)
        //     ui.separator();
        //     println!("left panel");
        // });
        //
        egui::CentralPanel::default().show(ctx, |ui| {
            DockArea::new(&mut self.dock_state).show(ctx, &mut TabViewer {});
            // The central panel the region left after adding TopPanel's and SidePanel's
            // ui.heading("Download");
            // ui.add(egui::Slider::new(&mut self.value, 0.0..=10.0).text("value"));
            // if ui.button("Increment").clicked() {
            //     self.value += 1.0;
            // }

            // ui.separator();

            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

struct Tab {
    title: String,
    content: String,
}

struct TabViewer {
    dock_state: DockState<Box<dyn Tab>>,
}

impl TabViewer {
    fn new() {
        Self {
            dock_state: DockState::new(Vec::new()),
        }
    }
}

impl egui_dock::TabViewer for TabViewer {
    type Tab = app::Tab;

    fn title(&mut self, tab: &mut Self::Tab) -> egui::WidgetText {
        tab.title()
    }

    fn ui(&mut self, ui: &mut Ui, tab: &mut Self::Tab) {}
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
