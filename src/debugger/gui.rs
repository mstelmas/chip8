use conrod::{self, widget, Colorable, Positionable, Widget};
use conrod::backend::glium::glium;
use conrod::backend::glium::glium::{Surface};
use std;
use cli::{Cli, CpuSnapshot};
use disasm::Disasm;
use disasm::Opcode;
use commands::Commands;
use std::borrow::Cow;
use std::str::FromStr;

#[derive(Default)]
struct Chip8State {
    cpu_status_textbox: String,
    stack_textbox: String,
    disasm_textbox: String,
    mem_dump_textbox: String,
}

// TODO: Refactor when GUI becomes more "stable"
pub fn run() {
    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 600;

    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Chip8VM Debugger!")
        .with_dimensions((WIDTH, HEIGHT).into());
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64]).build();

    const FONT_PATH: &'static str = "src/debugger/assets/fonts/NotoSans-Regular.ttf";
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let ids = &mut Ids::new(ui.widget_id_generator());

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut command_text = "".to_owned();
    let mut cli = Cli::new();
    let mut chip8_state = Chip8State::default();

    let mut events = Vec::new();

    'render: loop {
        events.clear();

        events_loop.poll_events(|event| { events.push(event); });

        if events.is_empty() {
            events_loop.run_forever(|event| {
                events.push(event);
                glium::glutin::ControlFlow::Break
            });
        }

        for event in events.drain(..) {

            match event.clone() {
                glium::glutin::Event::WindowEvent { event, .. } => {
                    match event {
                        glium::glutin::WindowEvent::CloseRequested |
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::Escape),
                                ..
                            },
                            ..
                        } => break 'render,
                        _ => (),
                    }
                }
                _ => (),
            };

            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };

            ui.handle_event(input);

            set_widgets(ui.set_widgets(), ids, &mut command_text, &mut chip8_state, &mut cli);
        }

        if let Some(primitives) = ui.draw_if_changed() {
            renderer.fill(&display, primitives, &image_map);
            let mut target = display.draw();
            target.clear_color(0.0, 0.0, 0.0, 1.0);
            renderer.draw(&display, &mut target, &image_map).unwrap();
            target.finish().unwrap();
        }
    }
}

fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, command_text: &mut String, chip8_state: &mut Chip8State, cli: &mut Cli) {
    use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    let mut s = widget::canvas::Style::default();
    s.border_color = Some(color::GREEN);
    s.border = Some(3.0);

    widget::Canvas::new().flow_down(&[
        (ids.body, widget::Canvas::new().length_weight(0.94).flow_right(&[
            (ids.left_column, widget::Canvas::new().with_style(s).color(color::BLACK)),
            (ids.right_column, widget::Canvas::new().flow_down(&[
                (ids.cpu_stack_canvas, widget::Canvas::new().with_style(s).flow_right(&[
                    (ids.cpu_canvas, widget::Canvas::new().with_style(s).color(color::BLACK)),
                    (ids.stack_canvas, widget::Canvas::new().with_style(s).color(color::BLACK))
                ])),
                (ids.mem_canvas, widget::Canvas::new().with_style(s).color(color::BLACK)),
            ])),
        ])),
        (ids.commands_canvas, widget::Canvas::new().color(color::BLUE).length_weight(0.06).scroll_kids_vertically()),
    ]).set(ids.master, ui);

    widget::Text::new(&chip8_state.cpu_status_textbox)
        .color(color::WHITE)
        .padded_w_of(ids.cpu_canvas, 10.0)
        .mid_top_with_margin_on(ids.cpu_canvas, 10.0)
        .left_justify()
        .line_spacing(10.0)
        .font_size(14)
        .set(ids.cpu_status_textbox, ui);

    widget::Text::new(&chip8_state.stack_textbox)
        .color(color::WHITE)
        .padded_w_of(ids.stack_canvas, 10.0)
        .mid_top_with_margin_on(ids.stack_canvas, 10.0)
        .left_justify()
        .line_spacing(10.0)
        .font_size(13)
        .set(ids.stack_textbox, ui);

    widget::Text::new(&chip8_state.disasm_textbox)
        .color(color::WHITE)
        .padded_w_of(ids.left_column, 10.0)
        .mid_top_with_margin_on(ids.left_column, 10.0)
        .left_justify()
        .line_spacing(10.0)
        .font_size(16)
        .scroll_kids()
        .set(ids.disasm_textbox, ui);

    widget::Text::new(&chip8_state.mem_dump_textbox)
        .color(color::WHITE)
        .padded_w_of(ids.mem_canvas, 10.0)
        .mid_top_with_margin_on(ids.mem_canvas, 10.0)
        .left_justify()
        .line_spacing(10.0)
        .font_size(16)
        .scroll_kids()
        .set(ids.mem_dump_textbox, ui);

    for event in widget::TextBox::new(command_text)
        .font_size(17)
        .w_of(ids.commands_canvas)
        .h_of(ids.commands_canvas)
        .color(color::WHITE)
        .align_bottom()
        .align_left()
        .align_top()
        .align_right()
        .mid_top_of(ids.commands_canvas)
        .left_justify()
        .set(ids.command_textbox, ui) {
        match event {
            widget::text_box::Event::Enter => {
                match Commands::from_str(command_text) {
                    Ok(Commands::Cpu) => {
                        let cpu_state = cli.cpu();
                        chip8_state.cpu_status_textbox.clear();
                        chip8_state.cpu_status_textbox.push_str(&(format!("PC: 0x{:x}  SP: 0x{:x}  I: 0x{:x}\n\n\
                                                              V0: 0x{:x}  V6: 0x{:x}  V12: 0x{:x}\n\
                                                              V1: 0x{:x}  V7: 0x{:x}  V13: 0x{:x}\n\
                                                              V2: 0x{:x}  V8: 0x{:x}  V14: 0x{:x}\n\
                                                              V3: 0x{:x}  V9: 0x{:x}  V15: 0x{:x}\n\
                                                              V4: 0x{:x}  V10: 0x{:x}  \n\
                                                              V5: 0x{:x}  V11: 0x{:x}  \
                                                              ", cpu_state.pc, cpu_state.sp, cpu_state.i,
                                                              cpu_state.v[0], cpu_state.v[6], cpu_state.v[12],
                                                              cpu_state.v[1], cpu_state.v[7], cpu_state.v[13],
                                                              cpu_state.v[2], cpu_state.v[8], cpu_state.v[14],
                                                              cpu_state.v[3], cpu_state.v[9], cpu_state.v[15],
                                                              cpu_state.v[4], cpu_state.v[10],
                                                              cpu_state.v[5], cpu_state.v[11]).to_owned())[..]);

                        chip8_state.stack_textbox.clear();

                        let v: Vec<String> = cpu_state.stack.iter().enumerate()
                            .map(|(i, s)| format!("{}{:02X}", if i as u8 == cpu_state.sp { "->" } else { "  " }, s)).collect()
                        ;

                        chip8_state.stack_textbox.push_str(&v.join("\n"));

                    }
                    Ok(Commands::Disasm(addr)) => {
                        let m = cli.mem(addr, 100);

                        let a: Vec<(Opcode, u16)> = Disasm::disasm(&m);

                        let b: Vec<String> = a.iter().enumerate().map(|(i, op)| format!("0x{:03x} {:02X?} {:02X?}  {}", addr as usize + i * 2,
                                                                                        (op.1 >> 8) as u8, (op.1 & 0xFF) as u8, op.0.repr())).collect();
                        &chip8_state.disasm_textbox.clear();
                        &chip8_state.disasm_textbox.push_str(&b.join("\n"));
                    },
                    Ok(Commands::Mem(addr)) => {
                        let m = cli.mem(addr, 100);
                        let width = 12;
                        let mut f = String::new();

                        for i in 0..(m.len() / width) {
                            f.push_str(&format!("0x{:03x}  ", addr as usize + (i * width)));
                            for i in &m[width * i..(width * i) + width] {
                                f.push_str(&format!("{:02X?} ", i));
                            }
                            f.push_str("\n");
                        }

                        &chip8_state.mem_dump_textbox.clear();
                        &chip8_state.mem_dump_textbox.push_str(&f);

                    },
                    Ok(Commands::Start) => cli.start(),
                    Ok(Commands::Step) => cli.step(),
                    Ok(Commands::Stop) => cli.stop(),
                    Err(ref e) => println!("{}", e)
                }
            },
            widget::text_box::Event::Update(s) => {
                *command_text = s;
            }
        }
    }
}

widget_ids! {
        struct Ids {
            master,
            body,
            left_column,
            right_column,

            cpu_canvas,
            stack_canvas,
            cpu_stack_canvas,
            mem_canvas,
            commands_canvas,

            cpu_status_textbox,
            mem_dump_textbox,
            disasm_textbox,
            command_textbox,
            stack_textbox
        }
}
