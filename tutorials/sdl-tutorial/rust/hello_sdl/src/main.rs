extern crate sdl2;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("hello", 800, 600)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let mut event_pump = sdl_context.event_pump().unwrap();
    // 'main is nani
    'main: loop {
        for event in event_pump.poll_iter() {
            println!("event: {:?}", event);
            match event {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }
    }
}
