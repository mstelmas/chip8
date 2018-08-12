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
use conrod::event;

const HORIZONTAL_SPLIT_RATIO: f32 = 0.5;
const MEM_VIEW_WIDTH_FONT_RATIO: f32 = 0.0175;
const MEM_VIEW_HEIGHT_FONT_RATIO: f32 = 0.04;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 600;

struct UIState {
    cpu_status_textbox: String,
    stack_textbox: String,
    disasm_textbox: String,
    mem_dump_textbox: String,

    window_height: u32,
    window_width: u32,
}

impl Default for UIState {
    fn default() -> Self {
        UIState {
            cpu_status_textbox: String::new(),
            stack_textbox: String::new(),
            disasm_textbox: String::new(),
            mem_dump_textbox: String::new(),

            window_height: HEIGHT,
            window_width: WIDTH,
        }
    }
}

// TODO: Refactor when GUI becomes more "stable"
pub fn run(mut cli: Cli) {
    let mut events_loop = glium::glutin::EventsLoop::new();
    let window = glium::glutin::WindowBuilder::new()
        .with_title("Chip8VM Debugger!")
        .with_dimensions((WIDTH, HEIGHT).into())
        .with_resizable(false);
    let context = glium::glutin::ContextBuilder::new()
        .with_vsync(true)
        .with_multisampling(4);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut ui = conrod::UiBuilder::new([WIDTH as f64, HEIGHT as f64])..build();

    const FONT_PATH: &'static str = "src/debugger/assets/fonts/Consolas-Bold.ttf";
    ui.fonts.insert_from_file(FONT_PATH).unwrap();

    let mut renderer = conrod::backend::glium::Renderer::new(&display).unwrap();

    let ids = &mut Ids::new(ui.widget_id_generator());

    let image_map = conrod::image::Map::<glium::texture::Texture2d>::new();

    let mut command_text = "".to_owned();
    let mut ui_state = UIState::default();

    let mut events = Vec::new();
    synchronize_vm_state(&mut cli, &mut ui_state);

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
                        glium::glutin::WindowEvent::KeyboardInput {
                            input: glium::glutin::KeyboardInput {
                                virtual_keycode: Some(glium::glutin::VirtualKeyCode::F8),
                                ..
                            },
                            ..
                        } => {
                            cli.step();
                            synchronize_vm_state(&mut cli, &mut ui_state);
                        },
                        _ => (),
                    }
                },
                _ => (),
            };

            let input = match conrod::backend::winit::convert_event(event, &display) {
                None => continue,
                Some(input) => input,
            };

            ui.handle_event(input);

            set_widgets(ui.set_widgets(), ids, &mut command_text, &mut ui_state, &mut cli);
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

fn synchronize_vm_state(cli: &mut Cli, chip8_state: &mut UIState) {
    let cpu_state = cli.cpu();
    let code_at_pc = cli.mem(cpu_state.pc, 64);

    update_cpu_state_view(&cpu_state, chip8_state);
    update_stack_state_view(&cpu_state, chip8_state);
    update_disasm_view(cpu_state.pc, code_at_pc, chip8_state);

    // TEMP
    update_mem_dump_view(cpu_state.pc, cli.mem(cpu_state.pc, 100), chip8_state)
}

fn set_widgets(ref mut ui: conrod::UiCell, ids: &mut Ids, command_text: &mut String, chip8_state: &mut UIState, cli: &mut Cli) {
    use conrod::{color, widget, Colorable, Labelable, Positionable, Sizeable, Widget};

    let mut s = widget::canvas::Style::default();
    s.border_color = Some(color::GREEN);
    s.border = Some(3.0);

    let memory_dump_canvas = (ids.mem_canvas, widget::Canvas::new().with_style(s).color(color::BLACK));

    widget::Canvas::new().flow_down(&[
        (ids.body, widget::Canvas::new().length_weight(0.94).flow_right(&[
            (ids.left_column, widget::Canvas::new().with_style(s).color(color::BLACK)),
            (ids.right_column, widget::Canvas::new().flow_down(&[
                (ids.cpu_stack_canvas, widget::Canvas::new().with_style(s).flow_right(&[
                    (ids.cpu_canvas, widget::Canvas::new().with_style(s).color(color::BLACK)),
                    (ids.stack_canvas, widget::Canvas::new().with_style(s).color(color::BLACK))
                ])),
                (memory_dump_canvas),
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
        .padded_h_of(ids.mem_canvas, 8.0)
        .mid_top_with_margin_on(ids.mem_canvas, 10.0)
        .center_justify()
        .line_spacing(10.0)
        .font_size(12)
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
                        // We'll ignore this command for now, as debugger should always be synchronized
                        // with VM's state therefore there should be no need for manual CPU state
                        // retrieval
                    },
                    Ok(Commands::Disasm(addr)) => {
                        update_disasm_view(addr, cli.mem(addr, 100), chip8_state);
                    },
                    Ok(Commands::Mem(addr)) => {
                        update_mem_dump_view(addr, cli.mem(addr, 100), chip8_state);
                    },
                    Ok(Commands::Start) => {
                        cli.start()
                    },
                    Ok(Commands::Step) => {
                        cli.step();
                        synchronize_vm_state(cli, chip8_state);

                    }
                    Ok(Commands::Stop) => {
                        cli.stop()
                    },
                    Err(ref e) => println!("{}", e)
                }
            },
            widget::text_box::Event::Update(s) => {
                *command_text = s;
            }
        }
    }
}

fn update_cpu_state_view(cpu_state: &CpuSnapshot, chip8_state: &mut UIState) {
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
}

fn update_stack_state_view(cpu_state: &CpuSnapshot, chip8_state: &mut UIState) {
    chip8_state.stack_textbox.clear();

    let v: Vec<String> = cpu_state.stack.iter().enumerate()
        .map(|(i, s)| format!("{}{:02X}", if i as u8 == cpu_state.sp { "->" } else { "  " }, s)).collect()
    ;

    chip8_state.stack_textbox.push_str(&v.join("\n"));
}

fn update_disasm_view(addr: u16, bytes: Vec<u8>, chip8_state: &mut UIState) {
    let a: Vec<(Opcode, u16)> = Disasm::disasm(&bytes);

    let b: Vec<String> = a.iter().enumerate().map(|(i, op)| format!("0x{:03x} {:02X?} {:02X?}  {}", addr as usize + i * 2,
                                                                    (op.1 >> 8) as u8, (op.1 & 0xFF) as u8, op.0.repr())).collect();
    chip8_state.disasm_textbox.clear();
    chip8_state.disasm_textbox.push_str(&b.join("\n"));
}

fn update_mem_dump_view(addr: u16, bytes: Vec<u8>, ui_state: &mut UIState) {
    let mem_dump_canvas_width = ui_state.window_width as f32 * HORIZONTAL_SPLIT_RATIO;
    let mem_dump_canvas_height = ui_state.window_height as f32 * HORIZONTAL_SPLIT_RATIO;

    let width = (mem_dump_canvas_width * MEM_VIEW_WIDTH_FONT_RATIO) as usize;
    let height = (mem_dump_canvas_height * MEM_VIEW_HEIGHT_FONT_RATIO) as usize;

    let mut f = String::new();
    let mut add = String::new();
    let mut byt = String::new();
    let mut ascii = String::new();

    let mut aaa = 0;
    for i in 0..(bytes.len() / width) {

        if aaa == height {
            break;
        }

        aaa += 1;

        add.clear();
        byt.clear();
        ascii.clear();
        add.push_str(&format!("{:03X}", addr as usize + (i * width)));

        for i in &bytes[width * i..(width * i) + width] {
            byt.push_str(&format!("{:02X?} ", i));
        }

        for i in &bytes[width * i..(width * i) + width] {
            ascii.push_str(&format!("{0: <1} ", if i.is_ascii() { i.clone() as char } else { '.' }));
        }
        f.push_str(&format!("{0}  {1} {2}\n", add, byt, ascii));
    }

    ui_state.mem_dump_textbox.clear();
    ui_state.mem_dump_textbox.push_str(&f);
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
