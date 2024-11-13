use derivative::Derivative;
use egui::{CollapsingHeader, Slider};
use itertools::Itertools;
use uuid::Uuid;

#[derive(Clone, Derivative)]
#[derivative(Debug, Default)]
struct ScoringAction {
    #[derivative(Default(value = "Uuid::new_v4().to_string()"))]
    id: String,

    #[derivative(Default(value = "String::from(\"N/A\")"))]
    description: String,
    name: String,
    phase: i8,

    pointvalue: i8, // set to -1 to use point stages instead

    #[derivative(Default(value = "10"))]
    max_count: i8, // if using point stages this is useless

    #[derivative(Default(value = "0"))]
    count: i8, // if using point stages this is the index of the pointstages vector

    #[derivative(Default(value = "Vec::new()"))]
    pointstages: Vec<i8>,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct TemplateApp {
    #[serde(skip)]
    actions: Vec<ScoringAction>,
    score: i8,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            actions: initialize_default_actions(),
            score: 0,
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
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        initialize_default_actions();

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                if ui.button("Reset Actions").clicked() {
                    self.actions = initialize_default_actions();
                    self.score = 0;
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("AVR 2024 Calculator");

            let tot_phases = self
                .actions
                .iter()
                .map(|x| x.phase)
                .unique()
                .collect::<Vec<_>>();

            for phs in tot_phases.iter() {
                let mut actions_n_phase = self.actions.clone();
                actions_n_phase.retain(|x| x.phase == *phs);

                CollapsingHeader::new(format!("Phase {} Actions", phs))
                    .default_open(true)
                    .show(ui, |ui| {
                        for action in actions_n_phase.iter() {
                            let act_ref = self
                                .actions
                                .iter_mut()
                                .find(|act| act.id == action.id)
                                .unwrap();

                            ui.separator();
                            ui.horizontal(|ui| {
                                ui.vertical(|ui| {
                                    ui.label(egui::RichText::new(&action.name).size(16.0));
                                    ui.label(egui::RichText::new(&action.description).size(12.0))
                                });

                                ui.with_layout(
                                    egui::Layout::right_to_left(egui::Align::LEFT),
                                    |ui| {
                                        if act_ref.pointvalue == -1
                                            && !act_ref.pointstages.is_empty()
                                        {
                                            ui.add(Slider::new(
                                                &mut act_ref.count,
                                                0..=act_ref.pointstages.len() as i8 - 1,
                                            ));
                                        } else {
                                            if act_ref.pointvalue > 1 {
                                                ui.add(Slider::new(
                                                    &mut act_ref.count,
                                                    0..=act_ref.max_count,
                                                ));
                                            } else {
                                                let mut temp_bool = act_ref.count == 1;
                                                if ui.checkbox(&mut temp_bool, "").clicked() {
                                                    act_ref.count = if temp_bool { 1 } else { 0 };
                                                }
                                            }
                                        }
                                    },
                                );
                            });
                        }

                        ui.separator();
                    });
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::RIGHT), |ui| {
                score_label(ui, &mut self.actions, &mut self.score);
                ui.separator();

                author_text(ui);
            });
        });
    }
}

fn score_label(ui: &mut egui::Ui, actions: &mut Vec<ScoringAction>, score: &mut i8) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        if ui.button("Calculate").clicked() {
            *score = scoring_processor(actions.clone());
        }

        ui.add_space(10.);
        ui.label(format!("Score: {}", score));
    });
}

fn author_text(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("By Ari from Team Daedalus 76122A");
    });
}

fn scoring_processor(populated_game: Vec<ScoringAction>) -> i8 {
    return populated_game
        .iter()
        .map(|val| {
            if val.pointvalue == -1 {
                val.pointstages[val.count as usize]
            } else {
                val.pointvalue * val.count
            }
        })
        .sum();
}

fn initialize_default_actions() -> Vec<ScoringAction> {
    let mut ret = Vec::new();

    // Phase 1
    let phase_1 = ScoringAction {
        name: "Phase 1".to_string(),
        description: "Score in the goal zone".to_string(),
        pointvalue: 2,
        phase: 1,
        ..Default::default()
    };
    ret.push(phase_1);

    // Phase 2
    let phase_2 = ScoringAction {
        name: "Phase 2".to_string(),
        description: "Score in the railway".to_string(),
        pointvalue: 4,
        phase: 2,
        ..Default::default()
    };
    ret.push(phase_2);

    // Phase 3
    let phase_3 = ScoringAction {
        name: "Phase 3".to_string(),
        description: "Score in the hotspots".to_string(),
        pointvalue: 8,
        phase: 3,
        ..Default::default()
    };
    ret.push(phase_3);

    // Phase 4
    let phase_4 = ScoringAction {
        name: "Phase 4".to_string(),
        description: "Score in the stack".to_string(),
        pointvalue: 1,
        phase: 4,
        ..Default::default()
    };
    ret.push(phase_4);

    let phase_4_2 = ScoringAction {
        name: "Phase 4 2".to_string(),
        description: "Score in the stacksd".to_string(),
        pointvalue: -1,
        phase: 4,
        pointstages: vec![0, 3, 7, 4, 5, 8],
        ..Default::default()
    };
    ret.push(phase_4_2);

    return ret;
}