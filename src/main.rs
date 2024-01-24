use std::time::SystemTime;

use env_logger::Env;
use log::info;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::PixelFormatEnum;
use sdl2::video::FullscreenType;

use crate::emulator::emulator::Emulator;

const DEFAULT_SCREEN_SCALE: u32 = 4;
const SCREEN_WIDTH: u32 = 320;
const SCREEN_HEIGHT: u32 = 200;

mod emulator;

const COLORS: [u32; 16] = [
    0x000000,
    0xFFFFFF,
    0x880000,
    0xAAFFEE,
    0xCC44CC,
    0x00CC55,
    0x0000AA,
    0xEEEE77,
    0xDD8855,
    0x664400,
    0xFF7777,
    0x333333,
    0x777777,
    0xAAFF66,
    0x0088FF,
    0xBBBBBB,
];

pub fn main() -> Result<(), String> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    // error!("starting up");
    // warn!("starting up");
    info!("starting up");
    // debug!("starting up");
    // trace!("starting up");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Commodore64", SCREEN_WIDTH * DEFAULT_SCREEN_SCALE, SCREEN_HEIGHT * DEFAULT_SCREEN_SCALE)
        .position_centered()
        .resizable()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas()
        .accelerated()
        .present_vsync()
        .build().map_err(|e| e.to_string())?;

    let creator = canvas.texture_creator();
    let mut texture = creator
        .create_texture_streaming(PixelFormatEnum::RGB24, SCREEN_WIDTH, SCREEN_HEIGHT)
        .map_err(|e| e.to_string())?;

    // let f_name = "/home/vanja/___devel/emulator/roms/games/Blinky [Hans Christian Egeberg, 1991].ch8";
    // let vec = fs::read(f_name)
    //     .map_err(|e| format!("Error loading file '{}': {}", f_name, e.to_string()))?;
    let mut emulator = Emulator::new();

    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;
    let start_time = SystemTime::now();
    canvas.present();

    // let audio_subsystem = sdl_context.audio().unwrap();
    // let desired_spec = AudioSpecDesired {
    //     freq: Some(48000),
    //     channels: Some(1),  // mono
    //     samples: Some(2048)       // default sample size
    // };
    // let device = audio_subsystem.open_playback(None, &desired_spec, |spec| {
    //     // initialize the audio callback
    //     SquareWave {
    //         phase_inc: 400.0 / spec.freq as f32,
    //         phase: 0.0,
    //         volume: 0.25
    //     }
    // }).unwrap();

    // let mut start = SystemTime::now();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                    break 'running,
                Event::KeyDown { keycode: Some(Keycode::F11), .. } => {
                    let fullscreen_type = canvas.window().fullscreen_state();
                    canvas.window_mut().set_fullscreen(if fullscreen_type == FullscreenType::Off {
                        FullscreenType::Desktop
                    } else {
                        FullscreenType::Off
                    }).unwrap();
                }
                // Event::KeyDown { keycode: Some(keycode), repeat: false, .. } =>
                //     match decode(keycode) {
                //         Some(key) => emulator.key_down(key),
                //         None => ()
                //     }
                // Event::KeyUp { keycode: Some(keycode), repeat: false, .. } =>
                //     match decode(keycode) {
                //         Some(key) => emulator.key_up(key),
                //         None => ()
                //     }
                _ => ()
            }
        }

        let elapsed = SystemTime::now().duration_since(start_time).unwrap();
        emulator.step(elapsed)?;

        // if emulator.is_sound_active() {
        //     device.resume();
        // } else {
        //     device.pause();
        // }

        let _ = &texture.with_lock(None, |buffer: &mut [u8], pitch: usize| {
            for y in 0..SCREEN_HEIGHT as usize {
                for x in 0..SCREEN_WIDTH as usize {
                    let offset = y * pitch + x * 3;
                    let value = emulator.gpu.display[y][x] as usize;
                    let col = COLORS[value];
                    buffer[offset] = (col >> 16) as u8;
                    buffer[offset + 1] = (col >> 8) as u8;
                    buffer[offset + 2] = (col >> 0) as u8;
                }
            }
        })?;

        canvas.copy(&texture, None, None)?;
        canvas.present();

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 1_000));
        // let now = SystemTime::now();
        // let fps = 1000000f64 / now.duration_since(start).unwrap().as_micros() as f64;
        // println!("FPS: {:.2}", fps);
        // start = now;
    }

    Ok(())
}