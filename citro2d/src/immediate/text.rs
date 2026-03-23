//! Immediate mode text rendering functions.

use std::{cell::LazyCell, sync::Mutex};

use crate::{
    font::Font,
    render::Blit as _,
    text::{Text, TextDrawStyle},
};

pub static mut SHARED_FONT: Font = Font::get_shared();
thread_local! {
    /// Global text object for immediate mode text blitting
    #[allow(static_mut_refs)]
    static IMMEDIATE_TEXT: LazyCell<Mutex<Text<'static>>> =
        LazyCell::new(|| Mutex::new({
            Text::new((0., 0.).into(), TextDrawStyle::default(), unsafe { &SHARED_FONT }).unwrap()
        }));
}

/// Draws text with the default system font according to the system's region.
pub fn draw_text(text: &str, at: crate::Point, style: TextDrawStyle) {
    IMMEDIATE_TEXT.with(|mutex| {
        let mut shared_text = mutex.lock().unwrap();

        shared_text.position = at;
        shared_text.style = style;

        shared_text.parse(text).unwrap();

        shared_text.blit().unwrap();
    });
}

/// Draws text with a font.
pub fn draw_text_font(text: &str, at: crate::Point, style: TextDrawStyle, font: &Font) {
    let mut temp_text = Text::new(at, style, font).unwrap();
    temp_text.parse(text).unwrap();
    temp_text.blit().unwrap();
}
