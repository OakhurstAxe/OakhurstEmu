
pub mod nes {

    use std::time::Duration;

    use bevy::asset::RenderAssetUsages;
    use bevy::color::{palettes::css};
    use bevy::prelude::*;
    use bevy::audio::{Source, ChannelCount, SampleRate};    
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
    
    use crate::nes_console::nes::NesConsole;
    use crate::nes_ppu::nes::{ NTSC_X_RESOLUTION, NTSC_Y_RESOLUTION };

    const IMAGE_WIDTH: u32 = NTSC_X_RESOLUTION;
    const IMAGE_HEIGHT: u32 = NTSC_Y_RESOLUTION;

    /// Store the image handle that we will draw to, here.
    #[derive(Resource)]
    pub struct MyProcGenImage(Handle<Image>);

    #[derive(Resource)]
    pub struct Nes(NesConsole);

    #[derive(Resource)]
    pub struct NesRomFile(String);

    impl NesRomFile {

        pub fn new (rom_file: String) -> NesRomFile {
            Self {
                0: rom_file,
            }
        }
    }

    #[derive(Asset, TypePath)]
    pub struct NesAudio {
        samples: Vec<u8>,
    }

    pub struct NesDecoder {
        current_progress: usize,
        sample_length: usize,
        samples: Vec<u8>,
        sample_rate: SampleRate,
        sample_value: f32,
    }

    impl NesDecoder {
        fn new(samples: Vec<u8>) -> Self {
            let sample_rate = 44_100;
            NesDecoder  {
                current_progress: 0,
                sample_length: samples.len(),
                samples: samples,
                sample_rate: SampleRate::new(sample_rate).unwrap(),
                sample_value: 0.,
            }
        }
    }

    impl Iterator for NesDecoder {
        type Item = f32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.current_progress >= self.sample_length {
                return None;
            }

            self.sample_value = self.samples[self.current_progress] as f32 / 128. - 1.;
            self.current_progress += 1;
            Some(self.sample_value)
        }
    }

    impl Source for NesDecoder {
        fn current_span_len(&self) -> Option<usize> { None }
        fn channels(&self) -> ChannelCount { ChannelCount::new(1).unwrap() }
        fn sample_rate(&self) -> SampleRate { self.sample_rate }
        fn total_duration(&self) -> Option<Duration> { None }
    }

    impl Decodable for NesAudio {
        type Decoder = NesDecoder;

        fn decoder(&self) -> Self::Decoder {
            NesDecoder::new(self.samples.clone())
        }
    }

    pub struct NesBevy {
    }

    impl NesBevy {
    
        pub fn setup(
            mut commands: Commands, 
            mut images: ResMut<Assets<Image>>,
            rom_file:  ResMut<NesRomFile>,
            windows: Query<&mut Window>) {

            let nes_console = NesConsole::new(rom_file.0.clone());
            commands.insert_resource(Nes(nes_console));

            let image = Image::new_fill(
                Extent3d {
                    width: IMAGE_WIDTH,
                    height: IMAGE_HEIGHT,
                    depth_or_array_layers: 1,
                },
                TextureDimension::D2,
                &(css::BLACK.to_u8_array()),
                TextureFormat::Rgba8UnormSrgb,
                RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
            );

            let handle = images.add(image);
            let mut sprite = Sprite::from_image(handle.clone());
            sprite.custom_size = Some(Vec2::new(windows.single().unwrap().width(), windows.single().unwrap().height()));
            commands.spawn(sprite);
            commands.insert_resource(MyProcGenImage(handle));

        }

        pub fn frame(
            mut commands: Commands, 
            mut audio_assets: ResMut<Assets<NesAudio>>,
            video_handle: Res<MyProcGenImage>,
            mut video_assets: ResMut<Assets<Image>>,
            mut nes_console: ResMut<Nes>
        ) {
            let (video, audio) = nes_console.0.run_frame();
            let mut image = video_assets.get_mut(&video_handle.0).expect("Image not found");
 
            match video {
                Some(video) => {
                    for y in 0..IMAGE_HEIGHT {
                        for x in 0..IMAGE_WIDTH {
                            let color = Color::srgb_u8(
                                video[((y * IMAGE_WIDTH + x) * 3) as usize],
                                video[((y * IMAGE_WIDTH + x) * 3 + 1) as usize],
                                video[((y * IMAGE_WIDTH + x) * 3 + 2) as usize],
                            );
                            _ = image.set_color_at(x, y, color);
                        }
                    }
                },
                None => {
                    eprintln!("No frame video available");
                }
            }

            match audio {
                Some(audio) => {
                    
                    let nes_audio: NesAudio = NesAudio { samples: audio.clone() };
                    let audio_handle = audio_assets.add(nes_audio);
                    commands.spawn((AudioPlayer(audio_handle), PlaybackSettings::DESPAWN));
                },
                None => {
                    eprintln!("No frame audio available");
                }
            }
        }

        pub fn gamepad_system(gamepads: Query<(Entity, &Gamepad)>,
            mut nes_console: ResMut<Nes>
        ) {
            for (_entity, gamepad) in &gamepads {
                if gamepad.just_pressed(GamepadButton::Select) {
                    nes_console.0.left_controler_select(true);
                } else if gamepad.just_released(GamepadButton::Select) {
                    nes_console.0.left_controler_select(false);
                }

                if gamepad.just_pressed(GamepadButton::Start) {
                    nes_console.0.left_controler_start(true);
                } else if gamepad.just_released(GamepadButton::Start) {
                    nes_console.0.left_controler_start(false);
                }

                if gamepad.just_pressed(GamepadButton::South) {
                    nes_console.0.left_controler_a(true);
                } else if gamepad.just_released(GamepadButton::South) {
                    nes_console.0.left_controler_a(false);
                }

                if gamepad.just_pressed(GamepadButton::East) {
                    nes_console.0.left_controler_b(true);
                } else if gamepad.just_released(GamepadButton::East) {
                    nes_console.0.left_controler_b(false);
                }

                let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
                if left_stick_x > 0.01 {
                    nes_console.0.left_controler_left_right(1);
                }
                else if left_stick_x < -0.01 {
                    nes_console.0.left_controler_left_right(-1);
                }
                else {
                    nes_console.0.left_controler_left_right(0);
                }

                let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
                if left_stick_y > 0.01 {
                    nes_console.0.left_controler_up_down(-1);
                }
                else if left_stick_y < -0.01 {
                    nes_console.0.left_controler_up_down(1);
                }
                else {
                    nes_console.0.left_controler_up_down(0);
                }
            }
        }
    }
}