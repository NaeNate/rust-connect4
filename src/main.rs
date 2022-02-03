use sdl2::{
    event::Event::{KeyDown, Quit},
    gfx::{framerate::FPSManager, primitives::DrawRenderer},
    keyboard::Keycode,
    pixels::Color,
    render::WindowCanvas,
    Sdl,
};

fn create_canvas(context: &Sdl) -> WindowCanvas {
    context
        .video()
        .unwrap()
        .window("Connect 4", 750, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap()
        .into_canvas()
        .present_vsync()
        .build()
        .unwrap()
}

fn draw_board(canvas: &mut WindowCanvas) {
    canvas.set_draw_color(Color::RGB(60, 60, 60));
    canvas.clear();

    for i in 0..7 {
        for j in 0..6 {
            canvas
                .filled_circle(
                    105 * (i % 7) + 60,
                    100 * (j % 7) + 50,
                    45,
                    Color::RGB(80, 80, 80),
                )
                .unwrap()
        }
    }

    canvas.present();
}

fn draw_circle(canvas: &mut WindowCanvas, moves: &String) {
    canvas
        .filled_circle(325, 300, moves.len() as i16, Color::RGB(255, 255, 0))
        .unwrap();

    canvas.present();
}

fn main() {
    FPSManager::new().set_framerate(60).unwrap();

    let context = sdl2::init().unwrap();
    let mut canvas = create_canvas(&context);

    draw_board(&mut canvas);

    let mut moves = String::new();

    'game_loop: loop {
        for event in context.event_pump().unwrap().poll_iter() {
            match event {
                Quit { .. }
                | KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'game_loop,
                KeyDown { keycode, .. } => {
                    let count = moves.chars().count();

                    match keycode {
                        Some(Keycode::Num1) => moves += "1",
                        Some(Keycode::Num2) => moves += "2",
                        Some(Keycode::Num3) => moves += "3",
                        Some(Keycode::Num4) => moves += "4",
                        Some(Keycode::Num5) => moves += "5",
                        Some(Keycode::Num6) => moves += "6",
                        Some(Keycode::Num7) => moves += "7",
                        _ => {}
                    }

                    if moves.chars().count() > count {
                        draw_circle(&mut canvas, &moves)
                    }
                }
                _ => {}
            }
        }
    }
}
