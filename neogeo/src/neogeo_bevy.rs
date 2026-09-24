
pub mod neogeo {

    use std::sync::Arc;

    use bevy::asset::RenderAssetUsages;
    use bevy::color::{palettes::css};
    use bevy::prelude::*;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    use crate::neogeo_console::neogeo::NeoGeoConsole;

    const IMAGE_WIDTH: u32 = 320;
    const IMAGE_HEIGHT: u32 = 224;

    /// Store the image handle that we will draw to, here.
    #[derive(Resource)]
    pub struct MyProcGenImage(Handle<Image>);

    #[derive(Resource)]
    pub struct NeoGeo(NeoGeoConsole);

    #[derive(Resource)]
    pub struct NeoGeoRomFile(String);

    impl NeoGeoRomFile {

        pub fn new (rom_file: String) -> NeoGeoRomFile {
            Self {
                0: rom_file,
            }
        }
    }

    pub struct NeoGeoBevy {
    }

    impl NeoGeoBevy {
    
        pub fn setup(
            mut commands: Commands, 
            mut images: ResMut<Assets<Image>>,
            rom_file:  ResMut<NeoGeoRomFile>,
            windows: Query<&mut Window>) {

            let neogeo_console = NeoGeoConsole::new(rom_file.0.clone());
            commands.insert_resource(NeoGeo(neogeo_console));

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
            mut audio_assets: ResMut<Assets<AudioSource>>,
            video_handle: Res<MyProcGenImage>,
            mut video_assets: ResMut<Assets<Image>>,
            mut neogeo_console: ResMut<NeoGeo>
        ) {

            //let (video, audio) = neogeo_console.0.run_frame();
            neogeo_console.0.run_frame();
/*
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
*/
         }

         pub fn gamepad_system(gamepads: Query<(Entity, &Gamepad)>,
            mut nes_console: ResMut<NeoGeo>
        ) {
        }

    }
}