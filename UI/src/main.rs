use imgui::*;
use imgui::ImStr;

mod support;

fn main() {
    let mut     create_master: bool     = true;
    let mut      use_outlines: bool     = true;
    let mut  threadcount_text: ImString = ImString::with_capacity(3);

    let mut printbed_height: ImString = ImString::with_capacity(10);
    let mut printbed_width:  ImString = ImString::with_capacity(10);

    let mut x_scale:     ImString = ImString::with_capacity(10);
    let mut y_scale:     ImString = ImString::with_capacity(10);
    let mut preserve_aspect: bool = true;
    let mut render_orphaned_points: bool = true;
    let mut pixels_per_mm: ImString = ImString::with_capacity(3);

    const STARTING_WINDOW_WIDTH: u32 = 1440;
    const STARTING_WINDOW_HEIGHT: u32 = 1080;
    const WINDOW_HEADER: f32 = 20.0;

    const GLOBAL_PIVOT: [f32; 2] = [0.0, 0.0];

//    const GLOBAL_PIVOT: [f32; 2] = [-0.335, 0.0];

    let mut is_slicing: bool = false;

    let system = support::init(file!(), STARTING_WINDOW_WIDTH, STARTING_WINDOW_HEIGHT);
    system.main_loop(move |_, ui, window_size| {

        let mut y_offset: f32 = 0.0;
        let mut last_collapsed: bool = true;

        let window_width:  u32 = window_size.0;
        let window_height: u32 = window_size.1;

        Window::new(im_str!("Slicer Start"))
            .flags(WindowFlags::NO_RESIZE | WindowFlags::NO_TITLE_BAR | WindowFlags::NO_BACKGROUND)
            .size([150.0, 100.0], Condition::Always)
            .position([-0.5, 0.0], Condition::Always)
            .position_pivot(GLOBAL_PIVOT)
            .build(ui, || {
                let color_change = ui.push_style_color(StyleColor::Button, [0.0, 0.0, 0.0, 1.0]);
                ui.button(im_str!("Slice"), [100.0, 50.0]);
                color_change.pop(&ui);
            });

        Window::new(im_str!("Slicer Options"))
            .flags(WindowFlags::NO_RESIZE)
            .size([300.0, 250.0], Condition::Always)
            .position([window_width as f32 - ui.window_size()[0], y_offset], Condition::Always)
            .position_pivot(GLOBAL_PIVOT)
            .build(ui, || {
                ui.text(im_str!("Printbed Settings"));
                let mut textfield_width = ui.push_item_width(80.0);
                ui.input_text(im_str!("Height (mm)"), &mut printbed_height)
                    .flags(ImGuiInputTextFlags::CharsDecimal)
                    .build();
                ui.input_text(im_str!("Width (mm)"), &mut printbed_width)
                    .flags(ImGuiInputTextFlags::CharsDecimal)
                    .build();
                textfield_width.pop(&ui);
                ui.separator();
                ui.text(im_str!("Scaling"));
                textfield_width = ui.push_item_width(80.0);
                ui.input_text(im_str!("X (mm)"), &mut x_scale)
                    .flags(ImGuiInputTextFlags::CharsDecimal)
                    .build();
                ui.input_text(im_str!("Y (mm)"), &mut y_scale)
                    .flags(ImGuiInputTextFlags::CharsDecimal)
                    .build();
                textfield_width.pop(&ui);
                ui.checkbox(im_str!("Preserve aspect ratio"), &mut preserve_aspect);

                if is_slicing {
                ProgressBar::new(0.6)
                    .overlay_text(im_str!("Slicing"))
                    .build(&ui);
                }

                y_offset += ui.window_size()[1];
                last_collapsed = false;
            });

        if last_collapsed {
            y_offset += WINDOW_HEADER;
        } else {
            last_collapsed = true;
        }
        
        Window::new(im_str!("Render Options"))
            .flags(WindowFlags::NO_RESIZE)
            .size([300.0, 200.0], Condition::Always)
            .position([window_width as f32 - ui.window_size()[0], y_offset], Condition::Always)
            .position_pivot(GLOBAL_PIVOT)
            .build(ui, || {
                ui.checkbox(im_str!("Create master image"), &mut create_master);
                ui.checkbox(im_str!("Add outlines to master image"), &mut use_outlines);
                ui.checkbox(im_str!("Render orphaned points"), &mut render_orphaned_points);
                ui.separator();
                let textfield_width = ui.push_item_width(80.0);
                ui.input_text(im_str!("Render threadcount: "), &mut threadcount_text)
                    .flags(ImGuiInputTextFlags::CharsDecimal)
                    .build();
                ui.input_text(im_str!("Pixels per mm"), &mut pixels_per_mm)
                    .flags(ImGuiInputTextFlags::CharsDecimal)
                    .build();
                textfield_width.pop(&ui);
                
                y_offset += ui.window_size()[1];
                last_collapsed = false;
            });

        if last_collapsed {
            y_offset += WINDOW_HEADER;
        } else {
            last_collapsed = true;
        }

        Window::new(im_str!("Info"))
            .flags(WindowFlags::NO_RESIZE)
            .size([300.0, 150.0], Condition::Always)
            .collapsed(true, Condition::Once)
            .position([window_width as f32 - ui.window_size()[0], y_offset], Condition::Always)
            .position_pivot(GLOBAL_PIVOT)
            .build(ui, || {
                ui.text(im_str!("Program info will go here. "));

                y_offset += ui.window_size()[1];
                last_collapsed = false;
            });

        if last_collapsed {
            y_offset += WINDOW_HEADER;
        }

        Window::new(im_str!("Debug"))
            .flags(WindowFlags::NO_RESIZE)
            .size([300.0, 150.0], Condition::Always)
            .collapsed(true, Condition::Once)
            .position_pivot(GLOBAL_PIVOT)
            .position([window_width as f32 - ui.window_size()[0], y_offset], Condition::Always)
            .build(ui, || {
                ui.text(format!("This windows size is  {:?}, {:?}", ui.window_size()[0], ui.window_size()[1]));
                ui.text(format!("Global window size is {:?}, {:?}", window_width, window_height));
            });
    });
}