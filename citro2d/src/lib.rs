#![feature(custom_test_frameworks)]
#![test_runner(test_runner::run_gdb)]
#![feature(doc_cfg)]
#![doc(html_root_url = "https://rust3ds.github.io/citro2d-rs/crates")]
#![doc(
    html_favicon_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![doc(
    html_logo_url = "https://user-images.githubusercontent.com/11131775/225929072-2fa1741c-93ae-4b47-9bdf-af70f3d59910.png"
)]
#![feature(try_trait_v2)]
#![feature(slice_ptr_get)]

//! Safe Rust bindings to `citro2d`. This crate wraps `citro2d-sys` to provide
//! safer APIs for graphics programs targeting the 3DS.
//!
//! ## Feature flags
#![doc = document_features::document_features!()]

pub mod error;
pub mod font;
pub mod image;
pub mod immediate;
pub mod render;
pub mod sprite;
pub mod text;
pub mod types;

use std::cell::RefMut;

use citro2d_sys::{C2D_CreateScreenTarget, C2D_DEFAULT_MAX_OBJECTS, C2D_Init, C2D_Prepare};
use citro3d::render::ScreenTarget;
use citro3d_sys::{C3D_GetCmdBufUsage, C3D_GetDrawingTime, C3D_GetProcessingTime};
use ctru::services::gfx::Screen;
use error::{Error, Result};
pub use types::*;

/// The single instance for using `citro2d`. This is the base type that an application
/// should instantiate to use this library.
#[non_exhaustive]
#[must_use]
pub struct Instance {
    pub citro3d_instance: citro3d::Instance,
}

impl Instance {
    /// Create a new instance of `citro2d`.
    /// This also initializes `citro3d` since it is required for `citro2d`.
    pub fn new() -> Result<Self> {
        let citro3d_instance = citro3d::Instance::new().expect("failed to initialize Citro3D");
        Self::with_max_objects(C2D_DEFAULT_MAX_OBJECTS as usize, citro3d_instance)
    }

    /// You have to initialize citro3d before using citro2d, but some cases you may
    /// have initialized citro3d already, so you can use this function to initialize.
    /// You pass in the citro3d instance you already initialized to ensure it's lifetime is the same as citro2d
    /// **Note** The above statement may not work, and may not be able to switch between the two without api changes
    /// but currently working on that assumption and to allow for flexibility for the developer
    pub fn new_without_c3d_init(citro3d_instance: citro3d::Instance) -> Result<Self> {
        Self::with_max_objects(C2D_DEFAULT_MAX_OBJECTS as usize, citro3d_instance)
    }

    /// Create a new instance of `citro2d` with a custom maximum number of objects.
    #[doc(alias = "C2D_Init")]
    #[doc(alias = "C2D_Prepare")]
    pub fn with_max_objects(
        max_objects: usize,
        citro3d_instance: citro3d::Instance,
    ) -> Result<Self> {
        let new_citro_2d = match unsafe { C2D_Init(max_objects) } {
            true => Ok(Self { citro3d_instance }),
            false => Err(Error::FailedToInitialize),
        };
        unsafe { C2D_Prepare() };
        new_citro_2d
    }

    /// Create a new render target for a screen.
    #[doc(alias = "C2D_CreateScreenTarget")]
    pub fn create_screen_target<'screen>(
        &self,
        screen: RefMut<'screen, dyn Screen>,
    ) -> Result<ScreenTarget<'screen>> {
        unsafe {
            self.citro3d_instance
                .create_screen_target_from_raw(
                    C2D_CreateScreenTarget(screen.as_raw(), screen.side().into()),
                    screen,
                )
                .map_err(|_| Error::FailedToInitialize)
        }
    }

    /// Render 2D graphics.
    ///
    /// # Example
    ///
    /// ```rs
    /// instance.render_frame_with(|mut frame| {
    ///     target.clear_with_color(Color::new(0xfb, 0xdb, 0x65));
    ///     frame.select_render_target(&target).unwrap();
    ///
    ///     frame
    /// });
    #[doc(alias = "C3D_FrameBegin")]
    #[doc(alias = "C2D_SceneBegin")]
    #[doc(alias = "C2D_Flush")]
    #[doc(alias = "C3D_FrameEnd")]
    pub fn render_frame_with<'instance: 'frame, 'frame>(
        &'instance mut self,
        f: impl FnOnce(crate::render::Frame<'frame>) -> crate::render::Frame<'frame>,
    ) -> citro3d::Result<()> {
        self.citro3d_instance.render_frame_with(|frame| {
            let frame = crate::render::Frame::new(frame);
            let mut ret = f(frame);
            ret.flush();
            ret.consume()
        });
        Ok(())
    }

    /// Returns some stats about the 3Ds's graphics
    // TODO this may be more appropriate in citro3d
    #[doc(alias = "C3D_GetProcessingTime")]
    #[doc(alias = "C3D_GetDrawingTime")]
    #[doc(alias = "C3D_GetCmdBufUsage")]
    pub fn get_3d_stats(&self) -> Citro3DStats {
        //TODO should i check for NaN?
        let processing_time = unsafe { C3D_GetProcessingTime() };
        let drawing_time = unsafe { C3D_GetDrawingTime() };
        let cmd_buf_usage = unsafe { C3D_GetCmdBufUsage() };
        Citro3DStats {
            processing_time,
            drawing_time,
            cmd_buf_usage,
        }
    }
}

/// Stats about the 3Ds's graphics
#[derive(Debug, Clone, Copy)]
pub struct Citro3DStats {
    pub processing_time: f32,
    pub drawing_time: f32,
    pub cmd_buf_usage: f32,
}
