
use crate::dynamics::Attractor;

pub fn show(ui: &mut egui::Ui, attractor: &mut Attractor) -> bool {
    let mut parameters_changed = false;

    egui::Panel::right("jellyfish_controls")
        .default_size(280.0)
        .resizable(true)
        .show(ui, |ui| {
            ui.heading("Jellyfish");
            ui.separator();

            ui.heading("Attractor");

            let name = match &*attractor {
                Attractor::Lorenz(_) => "Lorenz",
                Attractor::Rossler(_) => "Rössler",
            };

            ui.label(name);

            ui.separator();
            ui.heading("Parameters");

            for (index, spec) in attractor
                .parameter_specs()
                .iter()
                .enumerate()
            {
                let Some(mut value) = attractor.parameter_value(index)
                else {
                    continue;
                };

                let slider = egui::Slider::new(
                    &mut value,
                    spec.min..=spec.max,
                )
                .text(spec.name);

                if ui.add(slider).changed() {
                    parameters_changed |=
                        attractor.set_parameter(index, value);
                }
            }

            ui.separator();

            ui.heading("Controls");
            ui.label("1 - Lorenz");
            ui.label("2 - Rössler");
            ui.label("Space - Pause / Resume");
            ui.label("R - Reset animation");
            ui.label("+ / - - Animation speed");
        });

    parameters_changed
}
