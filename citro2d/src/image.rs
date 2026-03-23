use std::pin::Pin;

use citro2d_sys::{C2D_DrawImageAt, C2D_Image, Tex3DS_SubTexture};
use citro3d::texture::{ColorFormat, Face, Texture, TextureParameters};
use citro3d::texture::{MAX_TEX_SIZE, MIN_TEX_SIZE};
pub use swizzle_3ds::pix::ImageFormat;
use swizzle_3ds::pix::ImageView;

use crate::Point;
use crate::Size;
use crate::error::{Error, Result};
use crate::render::Blit;

pub trait Image {
    /// Get the subtexture's width.
    fn width(&self) -> u16;

    /// Get the subtexture's height.
    fn height(&self) -> u16;

    /// Get the inner image.
    fn get_inner(&self) -> &C2D_Image;

    /// Get the inner image as mutable.
    fn get_inner_mut(&mut self) -> &mut C2D_Image;

    /// Get the inner texture.
    fn get_texture(&self) -> &Texture;
}

/// A citro2d image that holds onto its own [`Texture`].
///
/// This is useful for using it as an actual image.
pub struct ImageOwned {
    inner: C2D_Image,
    // Referenced in `self.inner`.
    texture: Pin<Box<Texture>>,
    // Referenced in `self.inner`. Constant.
    subtex: Pin<Box<Tex3DS_SubTexture>>,
    pub position: Point,
    pub scale: (f32, f32),
}

impl ImageOwned {
    pub fn new(position: Point, size: Size) -> Result<Self> {
        let width = size.width as u16;
        let height = size.height as u16;

        let squared_width = (width.max(MIN_TEX_SIZE) - 1)
            .next_power_of_two()
            .min(MAX_TEX_SIZE);
        let squared_height = (height.max(MIN_TEX_SIZE) - 1)
            .next_power_of_two()
            .min(MAX_TEX_SIZE);
        let squared_size = squared_width.max(squared_height);

        let texture = Box::pin(
            Texture::new(TextureParameters::new_2d(
                squared_size,
                squared_size,
                ColorFormat::Rgb8,
            ))
            // we check the size between the boundaries ourselves already,
            // so the only error can be...
            .map_err(|_| Error::FailedToInitialize)?,
        );

        let tex_width = texture.width();
        let tex_height = texture.height();

        let subtex = Box::pin(Tex3DS_SubTexture {
            width,
            height,
            left: 0.0,
            top: 1.0,
            right: (width as f32) / (tex_width as f32),
            bottom: 1.0 - ((height as f32) / (tex_height as f32)),
        });

        let inner = C2D_Image {
            tex: texture.as_raw(),
            subtex: &raw const *subtex,
        };

        Ok(Self {
            inner,
            texture,
            subtex,
            position,
            scale: (1.0, 1.0),
        })
    }

    pub fn get_texture_mut(&mut self) -> &mut Texture {
        &mut self.texture
    }

    /// Load the texture by swizzling it.
    pub fn swizzle_load_image(&mut self, image: &image::DynamicImage) -> Result<()> {
        let image_view = swizzle_3ds::pix::ImageView::<&[u8]>::try_from(image)
            .map_err(|_| Error::InvalidFormat)?;
        let swizzled = swizzle_3ds::swizzle_image(&image_view);

        self.load_texture(swizzled.as_raw())?;

        Ok(())
    }

    /// Load an image into the texture by swizzling it.
    pub fn swizzle_load_texture_raw(
        &mut self,
        data: &[u8],
        format: ImageFormat,
    ) -> Result<ImageView<Vec<u8>>> {
        let image_view =
            swizzle_3ds::pix::ImageView::new(data, self.width() as _, self.height() as _, format);
        let swizzled = swizzle_3ds::swizzle_image(&image_view);

        self.load_texture(swizzled.as_raw())?;

        Ok(swizzled)
    }

    /// Load a texture in Morton Z-order format. See [`Image::load_rgba`] for loading in RGBA format.
    pub fn load_texture(&mut self, data: &[u8]) -> Result<()> {
        self.texture
            .load_image(data, Face::default())
            .map_err(|_| Error::InvalidSize)?;

        Ok(())
    }
}

impl Image for ImageOwned {
    fn width(&self) -> u16 {
        self.subtex.width
    }

    fn height(&self) -> u16 {
        self.subtex.height
    }

    fn get_inner(&self) -> &C2D_Image {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut C2D_Image {
        &mut self.inner
    }

    fn get_texture(&self) -> &Texture {
        &self.texture
    }
}

impl Blit for ImageOwned {
    type Err = ();
    fn blit(&mut self) -> std::result::Result<(), Self::Err> {
        let Point { x, y, z } = self.position;
        let (sx, sy) = self.scale;
        unsafe { C2D_DrawImageAt(self.inner, x, y, z, core::ptr::null_mut(), sx, sy) };
        Ok(())
    }
}

pub struct ImageRef<'a> {
    inner: C2D_Image,
    texture: &'a Texture,
    subtex: Pin<Box<Tex3DS_SubTexture>>,
    pub position: Point,
    pub scale: (f32, f32),
}

impl<'a> ImageRef<'a> {
    /// Create a new image with a reference to a texture.
    pub fn new(position: Point, size: Size, texture: &'a Texture) -> Self {
        let tex_width = texture.width();
        let tex_height = texture.height();

        let width = (size.width as u16).min(tex_width);
        let height = (size.height as u16).min(tex_height);

        let subtex = Box::pin(Tex3DS_SubTexture {
            width,
            height,
            left: 0.0,
            top: 1.0,
            right: (width as f32) / (tex_width as f32),
            bottom: 1.0 - ((height as f32) / (tex_height as f32)),
        });

        let inner = C2D_Image {
            tex: texture.as_raw(),
            subtex: &raw const *subtex,
        };

        Self {
            texture,
            subtex,
            inner,
            position,
            scale: (1.0, 1.0),
        }
    }
}

impl Image for ImageRef<'_> {
    fn width(&self) -> u16 {
        self.subtex.width
    }

    fn height(&self) -> u16 {
        self.subtex.height
    }

    fn get_inner(&self) -> &C2D_Image {
        &self.inner
    }

    fn get_inner_mut(&mut self) -> &mut C2D_Image {
        &mut self.inner
    }

    fn get_texture(&self) -> &Texture {
        self.texture
    }
}

impl Blit for ImageRef<'_> {
    type Err = ();
    fn blit(&mut self) -> std::result::Result<(), Self::Err> {
        let Point { x, y, z } = self.position;
        let (sx, sy) = self.scale;
        unsafe { C2D_DrawImageAt(self.inner, x, y, z, core::ptr::null_mut(), sx, sy) };
        Ok(())
    }
}
