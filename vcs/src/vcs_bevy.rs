

use std::time::Duration;

use bevy::app::AppExit;
use bevy::asset::RenderAssetUsages;
use bevy::audio::{Source, ChannelCount, SampleRate};   
use bevy::color::{palettes::css};
use bevy::{picking::hover::Hovered};
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::ui_widgets::{Activate, Button};
use bevy::prelude::*;

use crate::vcs_console::VcsConsole;

const IMAGE_WIDTH: u32 = 160;
const IMAGE_HEIGHT: u32 = 210;

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.35, 0.35);

#[derive(Component)]
struct PowerButton;
#[derive(Component)]
struct TvTypeButton;
#[derive(Component)]
struct DifficultyLeftButton;
#[derive(Component)]
struct DifficultyRightButton;
#[derive(Component)]
struct PauseButton;
#[derive(Component)]
struct SelectButton;
#[derive(Component)]
struct ResetButton;

/// Store the image handle that we will draw to, here.
#[derive(Resource)]
pub struct MyProcGenImage(Handle<Image>);

#[derive(Resource)]
pub struct Vcs(VcsConsole);

#[derive(Resource)]
pub struct VcsRomFile(String);

impl VcsRomFile {

    pub fn new (rom_file: String) -> VcsRomFile {
        Self(rom_file)
    }

}

#[derive(Asset, TypePath)]
pub struct VcsAudio {
    samples: Vec<f32>,
}

pub struct VcsDecoder {
    current_progress: usize,
    sample_length: usize,
    samples: Vec<f32>,
    sample_rate: SampleRate,
    sample_value: f32,
}

impl VcsDecoder {
    fn new(samples: Vec<f32>) -> Self {
        let sample_rate = 44_100;
        VcsDecoder  {
            current_progress: 0,
            sample_length: samples.len(),
            samples,
            sample_rate: SampleRate::new(sample_rate).unwrap(),
            sample_value: 0.,
        }
    }
}

impl Iterator for VcsDecoder {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_progress >= self.sample_length {
            return None;
        }

        self.sample_value = self.samples[self.current_progress];
        self.current_progress += 1;
        Some(self.sample_value)
    }
}

impl Source for VcsDecoder {
    fn current_span_len(&self) -> Option<usize> { None }
    fn channels(&self) -> ChannelCount { ChannelCount::new(1).unwrap() }
    fn sample_rate(&self) -> SampleRate { self.sample_rate }
    fn total_duration(&self) -> Option<Duration> { None }
}

impl Decodable for VcsAudio {
    type Decoder = VcsDecoder;

    fn decoder(&self) -> Self::Decoder {
        VcsDecoder::new(self.samples.clone())
    }
}

pub struct VcsBevy {
}

impl VcsBevy {

    pub fn setup(
        mut commands: Commands, 
        mut images: ResMut<Assets<Image>>,
        rom_file:  ResMut<VcsRomFile>,
        ) {

        let vcs_console = VcsConsole::new(rom_file.0.clone());
        commands.insert_resource(Vcs(vcs_console));

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

        let image_handle = images.add(image);
        let mut sprite = ImageNode::new(image_handle.clone());
        sprite.image_mode = NodeImageMode::Stretch;

        commands.insert_resource(MyProcGenImage(image_handle));

        // Power button
        commands.spawn((
            Button,
            PowerButton,
            Hovered::default(),
            Node {
                position_type: PositionType::Absolute,
                width: px(150),
                height: px(65),
                top: px(0),
                left: px(0),
                ..default()
            },                
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Power"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )])
        )
            .observe(|_activate: On<Activate>, mut exit: MessageWriter<AppExit>| {
                exit.write(AppExit::Success); 
            }
        );

        // Tv type button
        commands.spawn((
            Button,
            TvTypeButton,
            Hovered::default(),
            Node {
                position_type: PositionType::Absolute,
                width: px(150),
                height: px(65),
                top: px(0),
                left: px(150),
                ..default()
            },                
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Color"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )])
        )
            .observe(|_activate: On<PointerPress>,mut vcs_console: ResMut<Vcs>| {
                let is_color = !vcs_console.0.get_color();
                vcs_console.0.set_color(is_color);
            }
        );

        // Difficulty Left
        commands.spawn((
            Button,
            DifficultyLeftButton,
            Hovered::default(),
            Node {
                position_type: PositionType::Absolute,
                width: px(150),
                height: px(65),
                top: px(0),
                left: px(300),
                ..default()
            },                
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Left Diff"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )])
        )
            .observe(|_activate: On<PointerPress>,mut vcs_console: ResMut<Vcs>| {
                let difficulty = !vcs_console.0.get_difficulty_left();
                vcs_console.0.set_difficulty_left(difficulty);
            }
        );

        // Pause button
        commands.spawn((
            Button,
            PauseButton,
            Hovered::default(),
            Node {
                position_type: PositionType::Absolute,
                width: px(150),
                height: px(65),
                top: px(0),
                left: px(450),
                ..default()
            },                
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Pause"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )])
        )
            .observe(|_activate: On<PointerPress>,mut vcs_console: ResMut<Vcs>| {
                let pause = !vcs_console.0.get_paused();
                vcs_console.0.pause(pause);
            }
        );

        // Difficulty Right
        commands.spawn((
            Button,
            DifficultyRightButton,
            Hovered::default(),
            Node {
                position_type: PositionType::Absolute,
                width: px(150),
                height: px(65),
                top: px(0),
                left: px(600),
                ..default()
            },                
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Right Diff"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )])
        )
            .observe(|_activate: On<PointerPress>,mut vcs_console: ResMut<Vcs>| {
                let difficulty = !vcs_console.0.get_difficulty_right();
                vcs_console.0.set_difficulty_right(difficulty);
            }
        );

        // Select button
        commands.spawn((
            Button,
            SelectButton,
            Hovered::default(),
            Node {
                position_type: PositionType::Absolute,
                width: px(150),
                height: px(65),
                top: px(0),
                left: px(750),
                ..default()
            },                
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Select"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )])
        )
            .observe(|_activate: On<PointerPress>,mut vcs_console: ResMut<Vcs>| {
                vcs_console.0.select(true);
            }
        )
            .observe(|_activate: On<PointerRelease>,mut vcs_console: ResMut<Vcs>| {
                vcs_console.0.select(false);
            }
        );

        // Reset button
        commands.spawn((
            Button,
            ResetButton,
            Hovered::default(),
            Node {
                position_type: PositionType::Absolute,
                width: px(150),
                height: px(65),
                top: px(0),
                left: px(900),
                ..default()
            },                
            BackgroundColor(NORMAL_BUTTON),
            children![(
                Text::new("Reset"),
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            )])
        )
            .observe(|_activate: On<PointerPress>,mut vcs_console: ResMut<Vcs>| {
                vcs_console.0.reset(true);
            }
        )
            .observe(|_activate: On<PointerRelease>,mut vcs_console: ResMut<Vcs>| {
                vcs_console.0.reset(false);
            }
        );

        commands.spawn((
                sprite,
                Node {
                    position_type: PositionType::Absolute,
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    top: px(65),
                    left: px(0),
                    ..default()
                },             
            )
        );
    }

    pub fn frame(
        mut commands: Commands, 
        mut audio_assets: ResMut<Assets<VcsAudio>>,            
        video_handle: Res<MyProcGenImage>,
        mut video_assets: ResMut<Assets<Image>>,
        mut vcs_console: ResMut<Vcs>) {

        let (video, audio) = vcs_console.0.run_frame();

        match video {
            Some(video) => {
                let mut image = video_assets.get_mut(&video_handle.0).expect("Image not found");
    
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
                
                let vcs_audio: VcsAudio = VcsAudio { samples: audio.clone() };
                let audio_handle = audio_assets.add(vcs_audio);
                commands.spawn((AudioPlayer(audio_handle), PlaybackSettings::DESPAWN));
            },
            None => {
                eprintln!("No frame audio available");
            }
        }
    }

    pub fn gamepad_system(gamepads: Query<(Entity, &Gamepad)>,
        mut vcs_console: ResMut<Vcs>) {

        for (_entity, gamepad) in &gamepads {
            if gamepad.just_pressed(GamepadButton::Select) {
                vcs_console.0.select(true);
            } else if gamepad.just_released(GamepadButton::Select) {
                vcs_console.0.select(false);
            }

            if gamepad.just_pressed(GamepadButton::Start) {
                vcs_console.0.reset(true);
            } else if gamepad.just_released(GamepadButton::Start) {
                vcs_console.0.reset(false);
            }

            if gamepad.just_pressed(GamepadButton::South) {
                vcs_console.0.left_controler_a(true);
            } else if gamepad.just_released(GamepadButton::South) {
                vcs_console.0.left_controler_a(false);
            }

            let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
            if left_stick_x > 0.01 {
                vcs_console.0.left_controler_left_right(1);
            }
            else if left_stick_x < -0.01 {
                vcs_console.0.left_controler_left_right(-1);
            }
            else {
                vcs_console.0.left_controler_left_right(0);
            }

            let left_stick_y = gamepad.get(GamepadAxis::LeftStickY).unwrap();
            if left_stick_y > 0.01 {
                vcs_console.0.left_controler_up_down(1);
            }
            else if left_stick_y < -0.01 {
                vcs_console.0.left_controler_up_down(-1);
            }
            else {
                vcs_console.0.left_controler_up_down(0);
            }
        }
    }
}
