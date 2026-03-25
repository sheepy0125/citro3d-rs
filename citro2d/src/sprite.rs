//! Citro2D spritesheet and sprite management.

use std::{
    ffi::CString,
    io,
    os::{fd::AsRawFd, unix::ffi::OsStrExt as _},
    path::Path,
};

use citro2d_sys::{
    C2D_DrawSprite, C2D_Sprite, C2D_SpriteFromSheet, C2D_SpriteRotate, C2D_SpriteRotateDegrees,
    C2D_SpriteSetCenter, C2D_SpriteSetCenterRaw, C2D_SpriteSetDepth, C2D_SpriteSetPos,
    C2D_SpriteSetRotation, C2D_SpriteSetRotationDegrees, C2D_SpriteSetScale, C2D_SpriteSheet,
    C2D_SpriteSheetCount, C2D_SpriteSheetFree, C2D_SpriteSheetFromFD, C2D_SpriteSheetLoad,
    C2D_SpriteSheetLoadFromMem,
};

use crate::{
    Point, Size,
    error::{Error, Result},
    render::Blit,
};

/// A citro2d spritesheet.
///
/// You can get the inner [`C2D_SpriteSheet`] pointer using [`SpriteSheet::get_inner`].
#[derive(Debug, Clone)]
pub struct SpriteSheet(C2D_SpriteSheet);

impl SpriteSheet {
    /// Load a spritesheet from a file path.
    ///
    /// This expects a `.t3x` Tex3DS spritesheet. For more information,
    /// [see DevkitPro's program](https://github.com/devkitPro/tex3ds).
    ///
    /// # Safety
    /// The spritesheet is not checked for validity. Ensuring the validity of
    /// the spritesheet data is up to the caller.
    #[doc(alias = "C2D_SpriteSheetLoad")]
    pub unsafe fn from_file_path_unchecked(path: impl AsRef<Path>) -> io::Result<Self> {
        // Turn the path into a CString
        let c_path = CString::new(path.as_ref().as_os_str().as_bytes())
            .map_err(|_| io::ErrorKind::InvalidFilename)?;

        // Actually load the spritesheet
        Self::pointer_or_last_error(unsafe { C2D_SpriteSheetLoad(c_path.as_ptr()) })
    }

    /// Load a spritesheet from a file descriptor.
    ///
    /// # Safety
    /// The spritesheet is not checked for validity. Ensuring the validity of
    /// the spritesheet data is up to the caller.
    ///
    /// Ensuring the safety of actually using the file descriptor is also up to
    /// the caller.
    #[doc(alias = "C2D_SpriteSheetFromFD")]
    pub unsafe fn from_file_descriptor_unchecked(fd: impl AsRawFd) -> io::Result<Self> {
        Self::pointer_or_last_error(unsafe { C2D_SpriteSheetFromFD(fd.as_raw_fd()) })
    }

    /// Load a spritesheet from a slice.
    ///
    /// # Safety
    /// The spritesheet is not checked for validity. Ensuring the validity of
    /// the spritesheet data is up to the caller.
    #[doc(alias = "C2D_SpriteSheetLoadFromMem")]
    pub unsafe fn from_slice_unchecked(slice: &[u8]) -> io::Result<Self> {
        Self::pointer_or_last_error(unsafe {
            C2D_SpriteSheetLoadFromMem(slice.as_ptr().cast(), slice.len())
        })
    }

    fn pointer_or_last_error(spritesheet_pointer: C2D_SpriteSheet) -> io::Result<Self> {
        if spritesheet_pointer.is_null() {
            Err(io::Error::last_os_error())
        } else {
            Ok(Self(spritesheet_pointer))
        }
    }

    /// Gets a sprite from an index of the spritesheet, starting at 0.
    ///
    /// For more information, see [`Sprite::from_index`].
    pub fn sprite_at_index<'a>(&'a self, index: usize, center: Point) -> Result<Sprite<'a>> {
        Sprite::from_index(self, index, center, (1., 1.))
    }

    /// Gets the number of textures in the spritesheet.
    #[doc(alias = "C2D_SpriteSheetCount")]
    pub fn number_of_sprites(&self) -> usize {
        unsafe { C2D_SpriteSheetCount(self.0) }
    }

    /// Gets a copy of the inner [`C2D_SpriteSheet`] pointer.
    pub fn get_inner(&self) -> C2D_SpriteSheet {
        self.0
    }
}

impl Drop for SpriteSheet {
    /// Dropping a `SpriteSheet` will free the underlying data using [`C2D_SpriteSheetFree`].
    #[doc(alias = "C2D_SpriteSheetFree")]
    fn drop(&mut self) {
        unsafe { C2D_SpriteSheetFree(self.0) };
    }
}

/// A citro2d sprite.
///
/// Sprites are references to textures stored in [`SpriteSheet`]s.
///
/// You can get the inner [`C2D_Sprite`] pointer using [`Sprite::get_inner`].
pub struct Sprite<'a> {
    _spritesheet: &'a SpriteSheet,
    inner: C2D_Sprite,
    /// The position of the sprite as per its center. By default, this is its center.
    ///
    /// Use [`Sprite::set_center`] and [`Sprite::set_center_absolute`] to change
    /// the offset.
    pub position: Point,
    pub scale: (f32, f32),
    size: Size,
}

impl<'a> Sprite<'a> {
    /// Loads a sprite from an index of a spritesheet, starting at 0.
    ///
    /// # Errors
    ///
    /// Fails if the sprite's index is out of bounds. Use
    /// [`SpriteSheet::number_of_sprites`] to prevnet this.
    ///
    /// # Safety
    ///
    /// The sprite is not checked for validity. Ensuring the validity of the
    /// sprite and spritesheet is up to the caller.
    #[doc(alias = "C2D_SpriteFromSheet")]
    pub fn from_index(
        sheet: &'a SpriteSheet,
        index: usize,
        center: impl Into<Point>,
        scale: (f32, f32),
    ) -> Result<Self> {
        let center = center.into();

        let count = sheet.number_of_sprites();
        if index >= count {
            return Err(Error::IndexOutOfBounds {
                idx: index,
                len: count,
            });
        }

        let mut sprite = unsafe { core::mem::zeroed::<C2D_Sprite>() };
        unsafe { C2D_SpriteFromSheet(&raw mut sprite, sheet.get_inner(), index) };

        let subtex = unsafe { &*sprite.image.subtex };
        let size = (subtex.width as f32, subtex.height as f32).into();

        let mut s = Self {
            _spritesheet: sheet,
            inner: sprite,
            position: center,
            scale,
            size,
        };

        s.set_center_uv(0.5, 0.5);

        Ok(s)
    }

    /// Gets the width and height of the sprite, not taking into account the scale.
    pub fn size(&self) -> Size {
        self.size
    }

    /// Rotate the sprite clockwise by radians around its center.
    #[doc(alias = "C2D_SpriteRotate")]
    pub fn rotate_radians_by(&mut self, radians: f32) {
        unsafe { C2D_SpriteRotate(&raw mut self.inner, radians) };
    }

    /// Rotate the sprite clockwise by degrees around its center.
    #[doc(alias = "C2D_SpriteRotateDegrees")]
    pub fn rotate_degrees_by(&mut self, degrees: f32) {
        unsafe { C2D_SpriteRotateDegrees(&raw mut self.inner, degrees) };
    }

    /// Set the sprite's rotation in radians around its center.
    #[doc(alias = "C2D_SpriteSetRotation")]
    pub fn set_rotation_radians(&mut self, radians: f32) {
        unsafe { C2D_SpriteSetRotation(&raw mut self.inner, radians) };
    }

    /// Set the sprite's rotation in degrees around its center.
    #[doc(alias = "C2D_SpriteSetRotationDegrees")]
    pub fn set_rotation_degrees(&mut self, degrees: f32) {
        unsafe { C2D_SpriteSetRotationDegrees(&raw mut self.inner, degrees) };
    }

    /// Set the sprite's center in (U, V) coordinates, e.g. (0.5, 0.5) is its center.
    ///
    /// This affects rotation and its position offset.
    #[doc(alias = "C2D_SpriteSetCenter")]
    pub fn set_center_uv(&mut self, u: f32, v: f32) {
        unsafe { C2D_SpriteSetCenter(&raw mut self.inner, u, v) };
    }

    /// Set the sprite's center in (X, Y) coordinates from its top left point.
    ///
    /// This affects rotation and its position offset.
    #[doc(alias = "C2D_SpriteSetCenterRaw")]
    pub fn set_center_absolute(&mut self, x: f32, y: f32) {
        unsafe { C2D_SpriteSetCenterRaw(&raw mut self.inner, x, y) };
    }

    /// Get a reference to the inner [`C2D_Sprite`] struct.
    pub fn get_inner(&self) -> &C2D_Sprite {
        &self.inner
    }

    /// Gets a mutable reference to the inner [`C2D_Sprite`] struct.
    pub fn get_inner_mut(&mut self) -> &mut C2D_Sprite {
        &mut self.inner
    }
}

impl Blit for Sprite<'_> {
    type Err = ();

    #[doc(alias = "C2D_DrawSprite")]
    #[doc(alias = "C2D_SpriteSetPos")]
    #[doc(alias = "C2D_SpriteSetDepth")]
    #[doc(alias = "C2D_SpriteSetScale")]
    fn blit(&mut self) -> core::result::Result<(), Self::Err> {
        // Positioning
        let Point { x, y, z } = self.position;
        unsafe { C2D_SpriteSetPos(&raw mut self.inner, x, y) };
        unsafe { C2D_SpriteSetDepth(&raw mut self.inner, z) };

        let (sx, sy) = self.scale;
        unsafe { C2D_SpriteSetScale(&raw mut self.inner, sx, sy) };

        unsafe { C2D_DrawSprite(&self.inner) };

        Ok(())
    }
}
