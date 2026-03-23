use citro2d::{
    Color, GradientColor,
    font::Font,
    image::{Image as _, ImageOwned, ImageRef},
    immediate::{
        shape::{self, draw_line_solid},
        text::{draw_text, draw_text_font},
    },
    render::{Blit as _, TargetExt},
    sprite::SpriteSheet,
    text::TextDrawStyle,
};
use citro3d::texture::{self, Face};
use ctru::{prelude::*, services::romfs::RomFS};

fn create_texture_target() -> texture::Texture {
    // Texture targets have to be in vram
    let params = texture::TextureParameters::new_2d_in_vram(64, 64, texture::ColorFormat::Rgb8);
    let mut tex = texture::Texture::new(params).unwrap();
    tex.set_filter(texture::Filter::Linear, texture::Filter::Nearest);
    tex
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    ctru::set_panic_hook(false);

    let apt = Apt::new()?;
    let mut hid = Hid::new()?;
    let gfx = Gfx::new()?;
    let _romfs = RomFS::new().unwrap();

    let _c = Console::new(gfx.top_screen.borrow_mut());

    let mut instance = citro2d::Instance::new().expect("Couldn't obtain citro2d instance");

    // let top = gfx.top_screen.borrow_mut();
    // let mut top_target = citro2d_instance.create_screen_target(top).unwrap();
    let bottom = gfx.bottom_screen.borrow_mut();
    let bottom_target = instance.create_screen_target(bottom).unwrap();

    let spritesheet =
        unsafe { SpriteSheet::from_file_path_unchecked("romfs:/sprite.t3x").unwrap() };
    let mut smile = spritesheet.sprite_at_index(0, (300., 100.).into()).unwrap();
    smile.scale = (4., 4.);

    let texture = create_texture_target();
    let tex_target = instance
        .citro3d_instance
        .render_target_texture(texture, Face::default(), None)
        .unwrap();
    let mut tex_image = ImageRef::new((40., 40.).into(), (64., 64.).into(), tex_target.texture());

    while apt.main_loop() {
        hid.scan_input();
        if hid.keys_held().contains(KeyPad::START) {
            break;
        }

        instance
            .render_frame_with(|mut frame| {
                tex_target.clear_with_color(Color::new_with_alpha(0, 0, 0, 0));
                frame.select_render_target(&tex_target).unwrap();
                shape::draw_rect_solid((0., 0.).into(), (64., 64.).into(), Color::new(255, 0, 0));

                bottom_target.clear_with_color(Color::new(0xfb, 0xdb, 0x65));
                frame.select_render_target(&bottom_target).unwrap();
                tex_image.blit().unwrap();

                frame
            })
            .unwrap();

        gfx.wait_for_vblank();
    }

    drop(bottom_target);

    Ok(())
}
