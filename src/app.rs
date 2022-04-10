use eframe::{egui, epi};
use egui::plot::{Legend, Line, Plot, Value, Values};

use crate::obd;

pub struct App {
    csv: obd::CsvLog,

    show_legend: bool,
}

impl App {
    pub fn new(csv: obd::CsvLog) -> Self {
        Self {
            csv,
            show_legend: true,
        }
    }
}

impl epi::App for App {
    fn name(&self) -> &str {
        &self.csv.file_path
    }

    fn setup(
        &mut self,
        _ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .show(ctx, |ui| ui.checkbox(&mut self.show_legend, "Show Legend"));

        egui::CentralPanel::default().show(ctx, |ui| {
            let mut plot = Plot::new("my_plot").label_formatter(|label, value| {
                if label.is_empty() {
                    "".to_string()
                } else {
                    format!("{} = {}", label, value.y)
                }
            });

            if self.show_legend {
                plot = plot.legend(Legend::default());
            }

            plot.show(ui, |plot_ui| {
                self.csv
                    .data_cols
                    .iter()
                    .map(|c| {
                        let mut i = 0_usize;
                        let vals = c
                            .values
                            .iter()
                            .map(|v| {
                                let x = self.csv.x_col.values.get(i).unwrap_or(&0_f64);
                                i += 1;
                                Value::new(*x, *v)
                            })
                            .collect();
                        Line::new(Values::from_values(vals)).name(&c.header)
                    })
                    .for_each(|l| {
                        plot_ui.line(l);
                    });
            });
        });
    }
}
