#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui_extras::RetainedImage;
use std::{fs::File, io::Read};

const FEET_PER_PIXEL: f32 = (18.0 + (2.0 / 3.0)) / 447.0; // (18+(2÷3))÷447

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1400.0, 720.0)),
        ..Default::default()
    };
    eframe::run_native(
        "FRC 2023 Path Planner",
        options,
        Box::new(|_cc| Box::<MyApp>::default()),
    )
}

fn format_items(items: &Vec<(egui::Pos2, Action)>) -> String {
    let mut out = String::from("{");
    for (idx, (_pos, action)) in items.iter().enumerate() {
        out.push_str(&match action {
            Action::None => {
                format!("Action::{:?}", action)
            }
            Action::Translate(t) => {
                format!("Action::Translate({}, {})", t.x, t.y)
            }
            Action::TranslateAndRotate(t, a) => {
                format!("Action::TranslateAndRotate({}, {}, {})", t.x, t.y, a)
            }
        });
        if idx < items.len() - 1 {
            out.push_str(", ");
        }
    }
    out.push('}');
    return out;
}

#[derive(PartialEq, Eq)]
enum Alliance {
    Red,
    Blue,
}

#[derive(Debug)]
#[allow(dead_code)]
enum Action {
    Translate(egui::Pos2),
    TranslateAndRotate(egui::Pos2, i32),
    None,
}

struct MyApp {
    items: Vec<(egui::Pos2, Action)>,
    field_image: RetainedImage,
    angle: i32,
    alliance: Alliance,
}

/*
struct MyApp {
    name: String,
    age: u32,
}
*/
impl Default for MyApp {
    fn default() -> Self {
        let filename = "Field_Scaled.png";
        let mut img_buffer = vec![];
        File::open(filename)
            .unwrap()
            .read_to_end(&mut img_buffer)
            .unwrap();
        let img = RetainedImage::from_image_bytes(filename, &img_buffer[..]);
        Self {
            items: vec![],
            field_image: img.unwrap(),
            angle: 0,
            alliance: Alliance::Blue,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // set up painter for points and lines
            let painter = egui::Painter::new(
                ctx.clone(),
                egui::LayerId {
                    id: egui::Id::new(0),
                    order: egui::Order::Foreground,
                },
                egui::Rect {
                    min: egui::Pos2 { x: 0.0, y: 0.0 },
                    max: egui::Pos2 {
                        x: 4096.0,
                        y: 4096.0,
                    },
                },
            );

            // set up painter for the background (field image and rectangle around it)
            let bg_painter = egui::Painter::new(
                ctx.clone(),
                egui::LayerId {
                    id: egui::Id::new(1),
                    order: egui::Order::Background,
                },
                egui::Rect {
                    min: egui::Pos2 { x: 0.0, y: 0.0 },
                    max: egui::Pos2 {
                        x: 4096.0,
                        y: 4096.0,
                    },
                },
            );

            // paint field image
            bg_painter.image(
                self.field_image.texture_id(&ctx),
                egui::Rect {
                    min: egui::Pos2 { x: 5.0, y: 5.0 },
                    max: egui::Pos2 {
                        x: 1085.0,
                        y: 528.0,
                    },
                },
                egui::Rect {
                    min: egui::Pos2 { x: 0.0, y: 0.0 },
                    max: egui::Pos2 { x: 1.0, y: 1.0 },
                },
                egui::Color32::WHITE,
            );

            // paint rectangle around field (no fill)
            bg_painter.rect_stroke(
                egui::Rect {
                    min: egui::Pos2 { x: 0.0, y: 0.0 },
                    max: egui::Pos2 {
                        x: 1090.0,
                        y: 532.0,
                    },
                },
                egui::Rounding {
                    nw: 0.0,
                    ne: 0.0,
                    se: 0.0,
                    sw: 0.0,
                },
                egui::Stroke {
                    width: 5.0,
                    color: egui::Color32::from_rgb(90, 90, 90),
                },
            );

            // button to copy path to clipboard
            if (ui.put(
                egui::Rect {
                    min: egui::Pos2 { x: 317.5, y: 537.0 },
                    max: egui::Pos2 { x: 517.5, y: 550.0 },
                },
                egui::Button::new("Copy Path to Clipboard"),
            ))
            .clicked[0]
            {
                ui.output_mut(|o| o.copied_text = format!("{:?}", format_items(&self.items)));
            }

            //button to clear path
            if (ui.put(
                egui::Rect {
                    min: egui::Pos2 { x: 567.5, y: 537.0 },
                    max: egui::Pos2 { x: 767.5, y: 550.0 },
                },
                egui::Button::new("Reset Path"),
            ))
            .clicked[0]
            {
                self.items = vec![];
            }

            // display path under the arena
            ui.put(
                egui::Rect {
                    min: egui::Pos2 { x: 10.0, y: 555.0 },
                    max: egui::Pos2 {
                        x: 1090.0,
                        y: 650.0,
                    },
                },
                egui::Label::new(format!("Your Path: {:?}", format_items(&self.items))),
            );

            // label describing action angle
            ui.put(
                egui::Rect {
                    min: egui::Pos2 {
                        x: 1100.0,
                        y: 215.0,
                    },
                    max: egui::Pos2 {
                        x: 1275.0,
                        y: 235.0,
                    },
                },
                egui::Label::new("The Angle to rotate during the translation"),
            );

            // slider for action angle
            ui.put(
                egui::Rect {
                    min: egui::Pos2 {
                        x: 1100.0,
                        y: 239.0,
                    },
                    max: egui::Pos2 {
                        x: 1100.0,
                        y: 289.0,
                    },
                },
                egui::Slider::new(&mut self.angle, -360..=360).suffix("°"),
            );

            // label to tell what radio buttons are for
            ui.put(
                egui::Rect {
                    min: egui::Pos2 {
                        x: 1120.0,
                        y: 300.0,
                    },
                    max: egui::Pos2 {
                        x: 1250.0,
                        y: 350.0,
                    },
                },
                egui::Label::new("Select the alliance that this auto is for"),
            );

            // radio buttons for alliance to change y later
            // red alliance
            if ui
                .put(
                    egui::Rect {
                        min: egui::Pos2 {
                            x: 1120.0,
                            y: 360.0,
                        },
                        max: egui::Pos2 {
                            x: 1180.0,
                            y: 360.0,
                        },
                    },
                    egui::RadioButton::new(self.alliance == Alliance::Red, "Red"),
                )
                .clicked()
            {
                self.alliance = Alliance::Red;
            }

            // blue alliance
            if ui
                .put(
                    egui::Rect {
                        min: egui::Pos2 {
                            x: 1190.0,
                            y: 360.0,
                        },
                        max: egui::Pos2 {
                            x: 1250.0,
                            y: 360.0,
                        },
                    },
                    egui::RadioButton::new(self.alliance == Alliance::Blue, "Blue"),
                )
                .clicked()
            {
                self.alliance = Alliance::Blue;
            }
            // draw points from self.actions
            for (idx, point) in self.items.iter().enumerate() {
                if idx < self.items.len() - 1 {
                    painter.line_segment(
                        [point.0, self.items[idx + 1].0],
                        egui::Stroke {
                            color: egui::Color32::from_rgba_premultiplied(75, 75, 75, 1),
                            width: 2.0,
                        },
                    )
                }
                painter.circle(
                    point.0,
                    10.0,
                    egui::Color32::from_rgba_premultiplied(50, 50, 50, 1),
                    egui::Stroke {
                        color: egui::Color32::from_rgba_premultiplied(75, 75, 75, 1),
                        width: 2.0,
                    },
                );
            }
            if self.items.len() > 0 {}
        }); // end show

        // add points to the items vector if the primary mouse button is pressed on the field
        if ctx.input(|i| i.pointer.primary_pressed()) {
            match ctx.input(|i| i.pointer.interact_pos()) {
                Some(s) => {
                    if 5.0 < s.x && s.x < 1085.0 && 5.0 < s.y && s.y < 528.0 {
                        if self.items.len() == 0 {
                            self.items.push((s, Action::None));
                        } else {
                            let x = match self.alliance {
                                Alliance::Blue => s.x - self.items.last().unwrap().0.x,
                                Alliance::Red => self.items.last().unwrap().0.x - s.x,
                            } * FEET_PER_PIXEL;
                            let y = match self.alliance {
                                Alliance::Blue => self.items.last().unwrap().0.y - s.y,
                                Alliance::Red => s.y - self.items.last().unwrap().0.y,
                            } * FEET_PER_PIXEL;
                            self.items.push((
                                s,
                                match self.angle {
                                    0 => Action::Translate(egui::Pos2 { x, y }),
                                    _ => {
                                        Action::TranslateAndRotate(egui::Pos2 { x, y }, self.angle)
                                    }
                                },
                            ))
                        }
                        self.angle = 0;
                    }
                }
                None => {}
            }
        }
    }
}
