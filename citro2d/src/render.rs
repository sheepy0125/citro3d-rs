//! Citro2D render target state management.

use citro2d_sys::{
    C2D_Flush, C2D_SceneBegin, C2D_TargetClear, C2D_ViewReset, C2D_ViewRestore, C2D_ViewRotate,
    C2D_ViewRotateDegrees, C2D_ViewSave, C2D_ViewScale, C2D_ViewShear, C2D_ViewTranslate,
};
use citro3d::{
    math::Matrix4,
    render::{ScreenTarget, Target, TextureTarget},
};
use citro3d_sys::{C3D_Mtx, C3D_SetViewport};
pub use ctru::services::gfx::{BottomScreen, Screen, TopScreen};

use crate::{
    Color,
    error::{Error, Result},
};

/// Blit trait to draw an object to a render target.
pub trait Blit {
    type Err;

    fn blit(&mut self) -> std::result::Result<(), Self::Err>;
}

pub struct Frame<'instance>(citro3d::render::Frame<'instance>);

impl<'instance> Frame<'instance> {
    pub(crate) fn new(c3d: citro3d::render::Frame<'instance>) -> Self {
        Self(c3d)
    }

    /// Select the given render target for the following draw calls.
    ///
    /// # Errors
    ///
    /// Fails if the given target cannot be used for drawing.
    #[doc(alias = "C3D_FrameDrawOn")]
    #[doc(alias = "C2D_SceneBegin")]
    pub fn select_render_target<T: citro3d::render::Target>(
        &mut self,
        target: &'instance T,
    ) -> Result<()> {
        self.0
            .select_render_target(target)
            .map_err(|_| Error::InvalidRenderTarget)?;
        unsafe { C2D_SceneBegin(target.as_raw()) };
        Ok(())
    }

    /// Get the inner [`citro3d::render::Frame`].
    pub fn get_inner_mut(&mut self) -> &mut citro3d::render::Frame<'instance> {
        &mut self.0
    }

    /// Ensure all 2D objects so far have been drawn.
    #[doc(alias = "C2D_Flush")]
    pub fn flush(&mut self) {
        unsafe { C2D_Flush() };
    }

    /// Consume the frame, returning the inner [`citro3d::render::Frame`].
    pub(crate) fn consume(self) -> citro3d::render::Frame<'instance> {
        self.0
    }

    /// Set the scene size to (width, heght) pixels. `tilt` treats the screens as sideways.
    pub fn set_scene_size(&mut self, width: u32, height: u32, tilt: bool) {
        unsafe { citro2d_sys::C2D_SceneSize(width, height, tilt) }
    }

    /// Set the viewport.
    /// (x, y) translations are inverted, e.g. a positive X moves the viewport left.
    #[doc(alias = "C3D_SetViewport")]
    pub fn set_viewport(&mut self, x: i32, y: i32, w: u32, h: u32) {
        // citro3d internally sets
        //  ctx->viewport[4] = (y << 16) | (x & 0xFFFF);
        // this accepts signed integers, but the function only accepts u32s.
        // no operations are performed to the x and y values, so this is stupid but works.
        let x = x.cast_unsigned();
        let y = y.cast_unsigned();
        // acts on 90 degree rotated screens
        unsafe { C3D_SetViewport(y, x, h, w) }
    }

    /// Reset the view transformation matrix. Transformations are saved across `.begin()` and `.end()`.
    #[doc(alias = "C2D_ViewReset")]
    pub fn reset_transformations(&mut self) {
        unsafe { C2D_ViewReset() }
    }

    /// Shear the view matrix by scale factors, e.g. (1.0, 0.0) shears by the full width.
    #[doc(alias = "C2D_ViewShear")]
    pub fn view_shear_by(&mut self, x: f32, y: f32) {
        unsafe { C2D_ViewShear(x, y) }
    }

    /// Scale the view matrix by scale factors, e.g. (0.5, 0.5) is half the size.
    #[doc(alias = "C2D_ViewScale")]
    pub fn view_scale_by(&mut self, x: f32, y: f32) {
        unsafe { C2D_ViewScale(x, y) }
    }

    /// Translate the view matrix by pixel quantities.
    #[doc(alias = "C2D_ViewTranslate")]
    pub fn view_translate_by(&mut self, x: f32, y: f32) {
        unsafe { C2D_ViewTranslate(x, y) }
    }

    /// Rotate the view matrix clockwise by radians around its top left point.
    #[doc(alias = "C2D_ViewRotate")]
    pub fn view_rotate_radians_by(&mut self, radians: f32) {
        unsafe { C2D_ViewRotate(radians) }
    }

    /// Rotate the view matrix clockwise by degrees around its top left point.
    #[doc(alias = "C2D_ViewRotateDegrees")]
    pub fn view_rotate_degrees_by(&mut self, degrees: f32) {
        unsafe { C2D_ViewRotateDegrees(degrees) }
    }

    /// Save the view matrix.
    #[doc(alias = "C2D_ViewSave")]
    pub fn save_view_matrix(&self) -> Matrix4 {
        let mut raw = unsafe { core::mem::zeroed::<C3D_Mtx>() };
        unsafe { C2D_ViewSave(&raw mut raw) };
        Matrix4::from_raw(raw)
    }

    /// Restore the view matrix.
    #[doc(alias = "C2D_ViewRestore")]
    pub fn restore_view_matrix(&self, matrix: &Matrix4) {
        unsafe { C2D_ViewRestore(matrix.as_raw()) };
    }
}

pub trait TargetExt: Target {
    /// Clear the target to a [`Color`].
    #[doc(alias = "C2D_TargetClear")]
    fn clear_with_color(&self, color: Color) {
        unsafe { C2D_TargetClear(self.as_raw(), color.inner) };
    }
}

impl TargetExt for ScreenTarget<'_> {}

impl TargetExt for TextureTarget {}
