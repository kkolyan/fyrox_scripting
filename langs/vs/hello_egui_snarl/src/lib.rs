use egui::Ui;
use egui_snarl::ui::{PinInfo, SnarlPin, SnarlViewer};
use egui_snarl::{InPin, OutPin, Snarl};

pub enum MyNode {
    Operator,
}

impl SnarlViewer<MyNode> for MyNode {
    fn title(&mut self, node: &MyNode) -> String {
        todo!()
    }

    fn inputs(&mut self, node: &MyNode) -> usize {
        todo!()
    }

    fn show_input(
        &mut self,
        pin: &InPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<MyNode>,
    ) -> impl SnarlPin + 'static {
        PinInfo::circle()
    }

    fn outputs(&mut self, node: &MyNode) -> usize {
        todo!()
    }

    fn show_output(
        &mut self,
        pin: &OutPin,
        ui: &mut Ui,
        scale: f32,
        snarl: &mut Snarl<MyNode>,
    ) -> impl SnarlPin + 'static {
        PinInfo::triangle()
    }
}
