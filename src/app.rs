use eframe::egui;
use egui::{
    Align, Color32, FontId, Layout, RichText, Rounding, Stroke, UiBuilder, Vec2, ViewportCommand,
};

use crate::calculator::{AngleMode, Calculator};

const CALC_WIDTH: f32 = 320.0;
const BUTTON_HEIGHT: f32 = 48.0;
const SCI_BUTTON_HEIGHT: f32 = 32.0;
const SPACING: f32 = 4.0;
const TITLE_BAR_HEIGHT: f32 = 32.0;

#[derive(Default)]
pub struct CalculatorApp {
    calc: Calculator,
    error_message: Option<String>,
    show_history: bool,
    is_maximized: bool,
}

impl CalculatorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        configure_style(&cc.egui_ctx);
        Self::default()
    }

    fn handle_keyboard(&mut self, ctx: &egui::Context) {
        ctx.input(|i| {
            for event in &i.events {
                if let egui::Event::Key {
                    key, pressed: true, ..
                } = event
                {
                    match key {
                        egui::Key::Backspace => self.calc.clear_entry(),
                        egui::Key::Delete => self.calc.clear(),
                        egui::Key::Escape => {
                            self.calc.clear();
                            self.error_message = None;
                        }
                        egui::Key::Enter => self.do_calculate(),
                        _ => {}
                    }
                }
                if let egui::Event::Text(text) = event {
                    self.error_message = None;
                    match text.as_str() {
                        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
                            self.calc.input_digit(text);
                        }
                        "." | "," => self.calc.input_decimal(),
                        "+" => self.calc.input_operator("+"),
                        "-" => self.calc.input_operator("−"),
                        "*" => self.calc.input_operator("×"),
                        "/" => self.calc.input_operator("÷"),
                        "^" => self.calc.input_power(),
                        "(" => self.calc.input_open_paren(),
                        ")" => self.calc.input_close_paren(),
                        "%" => self.calc.input_percent(),
                        "=" => self.do_calculate(),
                        _ => {}
                    }
                }
            }
        });
    }

    fn do_calculate(&mut self) {
        self.error_message = None;
        if let Err(e) = self.calc.calculate() {
            self.error_message = Some(e);
        }
    }
}

impl eframe::App for CalculatorApp {
    fn clear_color(&self, _visuals: &egui::Visuals) -> [f32; 4] {
        [0.0, 0.0, 0.0, 0.0] // Transparent background
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle_keyboard(ctx);

        // Window frame with rounded corners
        egui::CentralPanel::default()
            .frame(egui::Frame::none())
            .show(ctx, |ui| {
                let panel_rect = ui.max_rect();
                let rounding = if self.is_maximized { 0.0 } else { 12.0 };

                // Draw window background
                ui.painter().rect_filled(
                    panel_rect,
                    Rounding::same(rounding),
                    Color32::from_rgb(22, 22, 30),
                );

                // Draw subtle border (not when maximized)
                if !self.is_maximized {
                    ui.painter().rect_stroke(
                        panel_rect,
                        Rounding::same(rounding),
                        Stroke::new(1.0, Color32::from_rgb(45, 45, 60)),
                    );
                }

                ui.allocate_new_ui(UiBuilder::new().max_rect(panel_rect), |ui| {
                    ui.vertical(|ui| {
                        // Custom title bar
                        self.render_title_bar(ui, ctx);

                        // Content area with centering
                        let available = ui.available_size();

                        // Calculate content height (approximate)
                        // Display: 70 + margins, mode bar: ~30, keypad: ~360
                        let content_height = 490.0;
                        let content_width = CALC_WIDTH + 16.0; // with margins

                        // Center horizontally and vertically
                        let h_padding = ((available.x - content_width) / 2.0).max(0.0);
                        let v_padding = ((available.y - content_height) / 2.0).max(0.0);

                        ui.add_space(v_padding);

                        ui.horizontal(|ui| {
                            ui.add_space(h_padding);

                            egui::Frame::none()
                                .inner_margin(egui::Margin::symmetric(8.0, 8.0))
                                .show(ui, |ui| {
                                    ui.set_max_width(CALC_WIDTH);

                                    ui.vertical(|ui| {
                                        self.render_display(ui);
                                        ui.add_space(8.0);
                                        self.render_mode_bar(ui);
                                        ui.add_space(6.0);

                                        if self.show_history {
                                            self.render_history(ui);
                                        } else {
                                            self.render_keypad(ui);
                                        }
                                    });
                                });
                        });
                    });
                });
            });
    }
}

impl CalculatorApp {
    fn render_title_bar(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        let title_bar_rect = ui.available_rect_before_wrap();
        let title_bar_rect = egui::Rect::from_min_size(
            title_bar_rect.min,
            Vec2::new(title_bar_rect.width(), TITLE_BAR_HEIGHT),
        );

        // Allocate space for title bar
        let response = ui.allocate_rect(title_bar_rect, egui::Sense::click_and_drag());

        // Draw title bar background
        let corner_rounding = if self.is_maximized { 0.0 } else { 12.0 };
        ui.painter().rect_filled(
            title_bar_rect,
            Rounding {
                nw: corner_rounding,
                ne: corner_rounding,
                sw: 0.0,
                se: 0.0,
            },
            Color32::from_rgb(28, 28, 38),
        );

        // Handle window dragging
        if response.dragged() {
            ctx.send_viewport_cmd(ViewportCommand::StartDrag);
        }

        // Double-click to maximize/restore
        if response.double_clicked() {
            self.is_maximized = !self.is_maximized;
            ctx.send_viewport_cmd(ViewportCommand::Maximized(self.is_maximized));
        }

        // Title bar content
        ui.allocate_new_ui(UiBuilder::new().max_rect(title_bar_rect), |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(12.0);

                // App title
                ui.label(
                    RichText::new("Calculator")
                        .color(Color32::from_rgb(160, 160, 180))
                        .font(FontId::proportional(13.0)),
                );

                // Spacer
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.add_space(4.0);

                    // Close button
                    let close_btn = ui.add(
                        egui::Button::new(
                            RichText::new("×")
                                .font(FontId::monospace(18.0))
                                .color(Color32::from_rgb(200, 200, 200)),
                        )
                        .fill(Color32::TRANSPARENT)
                        .min_size(Vec2::new(TITLE_BAR_HEIGHT, TITLE_BAR_HEIGHT - 4.0))
                        .rounding(Rounding::same(6.0)),
                    );
                    if close_btn.clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }

                    // Maximize/Restore button
                    let max_icon = if self.is_maximized { "[  ]" } else { "[ ]" };
                    let max_btn = ui.add(
                        egui::Button::new(
                            RichText::new(max_icon)
                                .font(FontId::monospace(10.0))
                                .color(Color32::from_rgb(200, 200, 200)),
                        )
                        .fill(Color32::TRANSPARENT)
                        .min_size(Vec2::new(TITLE_BAR_HEIGHT, TITLE_BAR_HEIGHT - 4.0))
                        .rounding(Rounding::same(6.0)),
                    );
                    if max_btn.clicked() {
                        self.is_maximized = !self.is_maximized;
                        ctx.send_viewport_cmd(ViewportCommand::Maximized(self.is_maximized));
                    }

                    // Minimize button
                    let min_btn = ui.add(
                        egui::Button::new(
                            RichText::new("−")
                                .font(FontId::monospace(18.0))
                                .color(Color32::from_rgb(200, 200, 200)),
                        )
                        .fill(Color32::TRANSPARENT)
                        .min_size(Vec2::new(TITLE_BAR_HEIGHT, TITLE_BAR_HEIGHT - 4.0))
                        .rounding(Rounding::same(6.0)),
                    );
                    if min_btn.clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Minimized(true));
                    }
                });
            });
        });
    }

    fn render_display(&mut self, ui: &mut egui::Ui) {
        let display_height = 70.0;

        egui::Frame::none()
            .fill(Color32::from_rgb(28, 28, 36))
            .rounding(Rounding::same(12.0))
            .inner_margin(egui::Margin::symmetric(12.0, 8.0))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_height(display_height);

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.set_height(display_height);

                    if let Some(err) = &self.error_message {
                        ui.label(
                            RichText::new(err)
                                .color(Color32::from_rgb(255, 120, 120))
                                .font(FontId::monospace(20.0)),
                        );
                    } else {
                        let text = &self.calc.display;
                        let font_size = adaptive_font_size(text.len());

                        // Show open parens indicator
                        let display_text = if self.calc.get_open_parens() > 0 {
                            format!(
                                "{}{}",
                                text,
                                "⸣".repeat(self.calc.get_open_parens() as usize)
                            )
                        } else {
                            text.clone()
                        };

                        ui.label(
                            RichText::new(&display_text)
                                .color(Color32::WHITE)
                                .font(FontId::monospace(font_size)),
                        );
                    }
                });
            });
    }

    fn render_mode_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.set_width(ui.available_width());

            // Angle mode button
            let mode_text = match self.calc.angle_mode {
                AngleMode::Degrees => "DEG",
                AngleMode::Radians => "RAD",
            };

            if ui.add(mode_button(mode_text)).clicked() {
                self.calc.toggle_angle_mode();
            }

            ui.add_space(8.0);

            // Open parens indicator
            if self.calc.get_open_parens() > 0 {
                ui.label(
                    RichText::new(format!("({}", self.calc.get_open_parens()))
                        .color(Color32::from_rgb(255, 200, 100))
                        .font(FontId::monospace(12.0)),
                );
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                // History toggle
                let hist_icon = if self.show_history { "123" } else { "H" };
                if ui.add(mode_button(hist_icon)).clicked() {
                    self.show_history = !self.show_history;
                }
            });
        });
    }

    fn render_history(&mut self, ui: &mut egui::Ui) {
        let mut clicked_result: Option<String> = None;

        egui::Frame::none()
            .fill(Color32::from_rgb(28, 28, 36))
            .rounding(Rounding::same(12.0))
            .inner_margin(egui::Margin::same(12.0))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                ui.set_min_height(320.0);

                egui::ScrollArea::vertical()
                    .auto_shrink([false, false])
                    .show(ui, |ui| {
                        if self.calc.history.is_empty() {
                            ui.vertical_centered(|ui| {
                                ui.add_space(40.0);
                                ui.label(
                                    RichText::new("No history")
                                        .color(Color32::from_rgb(100, 100, 120))
                                        .font(FontId::monospace(14.0)),
                                );
                            });
                        } else {
                            for entry in self.calc.history.iter().rev() {
                                let result_clone = entry.result.clone();

                                let response = ui.horizontal(|ui| {
                                    ui.label(
                                        RichText::new(&entry.expression)
                                            .color(Color32::from_rgb(140, 140, 160))
                                            .font(FontId::monospace(13.0)),
                                    );
                                    ui.label(
                                        RichText::new(" = ")
                                            .color(Color32::from_rgb(100, 100, 120))
                                            .font(FontId::monospace(13.0)),
                                    );
                                    ui.label(
                                        RichText::new(&entry.result)
                                            .color(Color32::WHITE)
                                            .font(FontId::monospace(14.0)),
                                    );
                                });

                                // Click to use result
                                if response.response.interact(egui::Sense::click()).clicked() {
                                    clicked_result = Some(result_clone);
                                }

                                ui.add_space(4.0);
                            }
                        }
                    });
            });

        // Apply clicked result after iteration
        if let Some(result) = clicked_result {
            self.calc.use_history(&result);
            self.show_history = false;
        }
    }

    fn render_keypad(&mut self, ui: &mut egui::Ui) {
        let available_width = ui.available_width();
        let btn_width = (available_width - SPACING * 5.0) / 6.0;
        let main_btn_width = (available_width - SPACING * 3.0) / 4.0;

        // Scientific row 1
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = SPACING;

            if ui.add(sci_button("sin", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_function("sin");
            }
            if ui.add(sci_button("cos", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_function("cos");
            }
            if ui.add(sci_button("tan", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_function("tan");
            }
            if ui.add(sci_button("log", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_function("log10");
            }
            if ui.add(sci_button("ln", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_function("ln");
            }
            if ui.add(sci_button("√", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_function("sqrt");
            }
        });

        ui.add_space(SPACING);

        // Scientific row 2
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = SPACING;

            if ui.add(sci_button("x²", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_square();
            }
            if ui.add(sci_button("xʸ", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_power();
            }
            if ui.add(sci_button("(", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_open_paren();
            }
            if ui.add(sci_button(")", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_close_paren();
            }
            if ui.add(sci_button("π", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_constant("π");
            }
            if ui.add(sci_button("e", btn_width)).clicked() {
                self.error_message = None;
                self.calc.input_constant("e");
            }
        });

        ui.add_space(SPACING);

        // Main keypad
        let rows = [
            vec![
                ("C", BtnStyle::Clear),
                ("CE", BtnStyle::Clear),
                ("%", BtnStyle::Op),
                ("÷", BtnStyle::Op),
            ],
            vec![
                ("7", BtnStyle::Num),
                ("8", BtnStyle::Num),
                ("9", BtnStyle::Num),
                ("×", BtnStyle::Op),
            ],
            vec![
                ("4", BtnStyle::Num),
                ("5", BtnStyle::Num),
                ("6", BtnStyle::Num),
                ("−", BtnStyle::Op),
            ],
            vec![
                ("1", BtnStyle::Num),
                ("2", BtnStyle::Num),
                ("3", BtnStyle::Num),
                ("+", BtnStyle::Op),
            ],
            vec![
                ("±", BtnStyle::Num),
                ("0", BtnStyle::Num),
                (".", BtnStyle::Num),
                ("=", BtnStyle::Eq),
            ],
        ];

        for row in &rows {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = SPACING;

                for (label, style) in row {
                    if ui.add(calc_button(label, main_btn_width, *style)).clicked() {
                        self.error_message = None;
                        match *label {
                            "C" => self.calc.clear(),
                            "CE" => self.calc.clear_entry(),
                            "%" => self.calc.input_percent(),
                            "÷" => self.calc.input_operator("÷"),
                            "×" => self.calc.input_operator("×"),
                            "−" => self.calc.input_operator("−"),
                            "+" => self.calc.input_operator("+"),
                            "±" => self.calc.toggle_sign(),
                            "." => self.calc.input_decimal(),
                            "=" => self.do_calculate(),
                            digit => self.calc.input_digit(digit),
                        }
                    }
                }
            });

            ui.add_space(SPACING);
        }
    }
}

fn adaptive_font_size(len: usize) -> f32 {
    match len {
        0..=10 => 32.0,
        11..=15 => 26.0,
        16..=20 => 22.0,
        _ => 18.0,
    }
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    // Embedded subset font (33KB, contains only needed characters)
    let font_data = include_bytes!("../assets/font.ttf");

    fonts
        .font_data
        .insert("calc".into(), egui::FontData::from_static(font_data).into());
    fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap()
        .insert(0, "calc".into());
    fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap()
        .insert(0, "calc".into());

    ctx.set_fonts(fonts);
}

fn configure_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();

    style.visuals.dark_mode = true;
    style.visuals.panel_fill = Color32::from_rgb(16, 16, 22);
    style.visuals.window_fill = Color32::from_rgb(16, 16, 22);
    style.visuals.extreme_bg_color = Color32::from_rgb(12, 12, 16);

    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(32, 32, 42);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(38, 38, 50);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(52, 52, 68);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(62, 62, 82);

    style.visuals.widgets.noninteractive.rounding = Rounding::same(12.0);
    style.visuals.widgets.inactive.rounding = Rounding::same(12.0);
    style.visuals.widgets.hovered.rounding = Rounding::same(12.0);
    style.visuals.widgets.active.rounding = Rounding::same(12.0);

    style.spacing.item_spacing = Vec2::new(SPACING, SPACING);
    style.spacing.button_padding = Vec2::new(8.0, 6.0);

    ctx.set_style(style);
}

#[derive(Clone, Copy)]
enum BtnStyle {
    Num,
    Op,
    Clear,
    Eq,
}

fn calc_button(text: &str, width: f32, style: BtnStyle) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        let (bg, bg_hover, bg_active, fg) = match style {
            BtnStyle::Num => (
                Color32::from_rgb(48, 48, 62),
                Color32::from_rgb(58, 58, 75),
                Color32::from_rgb(38, 38, 50),
                Color32::WHITE,
            ),
            BtnStyle::Op => (
                Color32::from_rgb(58, 58, 78),
                Color32::from_rgb(70, 70, 95),
                Color32::from_rgb(45, 45, 65),
                Color32::from_rgb(140, 190, 255),
            ),
            BtnStyle::Clear => (
                Color32::from_rgb(85, 55, 55),
                Color32::from_rgb(100, 65, 65),
                Color32::from_rgb(70, 45, 45),
                Color32::from_rgb(255, 160, 160),
            ),
            BtnStyle::Eq => (
                Color32::from_rgb(65, 125, 195),
                Color32::from_rgb(80, 140, 210),
                Color32::from_rgb(50, 105, 170),
                Color32::WHITE,
            ),
        };

        let size = Vec2::new(width, BUTTON_HEIGHT);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let fill = if response.is_pointer_button_down_on() {
                bg_active
            } else if response.hovered() {
                bg_hover
            } else {
                bg
            };

            ui.painter().rect_filled(rect, Rounding::same(12.0), fill);

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                FontId::monospace(20.0),
                fg,
            );
        }

        response
    }
}

fn sci_button(text: &str, width: f32) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        let size = Vec2::new(width, SCI_BUTTON_HEIGHT);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let fill = if response.is_pointer_button_down_on() {
                Color32::from_rgb(28, 28, 38)
            } else if response.hovered() {
                Color32::from_rgb(46, 46, 60)
            } else {
                Color32::from_rgb(36, 36, 48)
            };

            ui.painter().rect_filled(rect, Rounding::same(8.0), fill);

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                FontId::monospace(13.0),
                Color32::from_rgb(180, 180, 200),
            );
        }

        response
    }
}

fn mode_button(text: &str) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| {
        let size = Vec2::new(48.0, 24.0);
        let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click());

        if ui.is_rect_visible(rect) {
            let fill = if response.is_pointer_button_down_on() {
                Color32::from_rgb(24, 24, 34)
            } else if response.hovered() {
                Color32::from_rgb(42, 42, 56)
            } else {
                Color32::from_rgb(32, 32, 44)
            };

            ui.painter().rect_filled(rect, Rounding::same(6.0), fill);
            ui.painter().rect_stroke(
                rect,
                Rounding::same(6.0),
                Stroke::new(1.0, Color32::from_rgb(50, 50, 65)),
            );

            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                FontId::monospace(11.0),
                Color32::from_rgb(150, 150, 170),
            );
        }

        response
    }
}
