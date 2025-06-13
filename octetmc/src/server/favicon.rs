use crate::util::dirty::Dirtyable;
use std::io::Cursor;
use bevy_ecs::resource::Resource;
use image::{ GenericImageView, PixelWithColorType, Pixel, EncodableLayout, ImageFormat, imageops };
use base64::Engine as B64Engine;
use base64::engine::general_purpose as b64gp;



/// The server favicon.
///
/// The favicon is displayed in server list.
#[derive(Resource)]
pub struct ServerFavicon {
    b64_png : String,
    dirty   : bool
}

impl ServerFavicon {

    /// ### Safety
    /// The given `b64_png` string must be a valid base 64 string of a, `64x64` `PNG` format image.
    ///
    /// Misusing this function is not inherently unsafe, but game clients will not render the image properly.
    pub unsafe fn from_b64_png_unchecked(b64_png : String) -> Self {
        Self { b64_png, dirty : false }
    }

    /// Set this favicon to some image.
    ///
    /// The image will be rescaled and converted to base 64 string of a `64x64` `PNG` format image.
    pub fn set<T>(&mut self, icon : T)
    where
        T : Into<Self>
    {
        *self = <T as Into<Self>>::into(icon);
    }

    /// Set this favicon using some base 64 image string.
    ///
    /// Many formats and any size is supported.
    ///
    /// The image will be rescaled and converted to base 64 string of a 64x64 `PNG` format image.
    pub fn try_set<T>(&mut self, icon : T) -> Result<(), T::Error>
    where
        T : TryInto<Self>
    {
        *self = <T as TryInto<Self>>::try_into(icon)?;
        Ok(())
    }

    /// ### Safety
    /// The given `b64_png` string must be a valid base 64 string of a 64x64 `PNG` format image.
    ///
    /// Misusing this function is not inherently unsafe, but game clients will not render the image properly.
    pub unsafe fn set_b64_png_unchecked(&mut self, b64_png : String) {
        self.b64_png = b64_png;
    }

    /// Get the contained image as a base 64 string of a 64x64 `PNG` format image.
    pub fn as_b64_png(&self) -> &str { &self.b64_png }

}

impl Dirtyable for ServerFavicon {
    #[inline]
    fn is_dirty(&self) -> bool { self.dirty }
    #[inline]
    fn dirty_mut(&mut self) -> &mut bool { &mut self.dirty }
}

impl<I> From<&I> for ServerFavicon
where
    I                                                     : GenericImageView,
    <I as GenericImageView>::Pixel                        : PixelWithColorType + 'static,
    <<I as GenericImageView>::Pixel as Pixel>::Subpixel   : 'static,
    [<<I as GenericImageView>::Pixel as Pixel>::Subpixel] : EncodableLayout,
{
    fn from(value : &I) -> Self {
        let mut bytes   = Vec::new();
        let     img     = imageops::resize(value, 64, 64, imageops::FilterType::Gaussian);
        let     _       = img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png);
        let     b64_png = b64gp::STANDARD.encode(bytes);
        Self { b64_png, dirty : false }
    }
}
impl TryFrom<&str> for ServerFavicon {
    type Error = InvalidB64Image;
    fn try_from(value : &str) -> Result<Self, Self::Error> {
        let mut bytes   = b64gp::STANDARD.decode(value)?;
        let     img     = image::load_from_memory(value.as_bytes())?;
        let     img     = img.resize_exact(64, 64, imageops::FilterType::Gaussian);
        bytes.clear();
        let     _       = img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png);
        let     b64_png = b64gp::STANDARD.encode(bytes);
        Ok(Self { b64_png, dirty : false })
    }
}


/// An error returned by `<ServerFavicon as TryFrom<&str>>::try_from`.
pub enum InvalidB64Image {
    /// The provided base 64 string is invalid.
    B64Decode(base64::DecodeError),
    /// The provided base 64 string image is not in a recognised image format.
    Image(image::ImageError)
}
impl From<base64::DecodeError> for InvalidB64Image {
    fn from(value : base64::DecodeError) -> Self { Self::B64Decode(value) }
}
impl From<image::ImageError> for InvalidB64Image {
    fn from(value : image::ImageError) -> Self { Self::Image(value) }
}
