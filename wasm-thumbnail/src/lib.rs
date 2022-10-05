use image::{
    self, imageops::FilterType, DynamicImage, GenericImage, GenericImageView, ImageOutputFormat,
};

use crate::wasm_thumbnail::Error;

wit_bindgen_rust::export!("wasm-thumbnail.wit");

struct WasmThumbnail;

impl wasm_thumbnail::WasmThumbnail for WasmThumbnail {
    fn resize_and_pad(
        image: Vec<u8>,
        width: u32,
        height: u32,
        quality: u8,
    ) -> Result<Vec<u8>, Error> {
        let img = image::load_from_memory(&image)?;

        // Resize preserves aspect ratio
        let img = img.resize(width, height, FilterType::Lanczos3);

        // Copy pixels only
        let mut result = DynamicImage::new_rgba8(img.width(), img.height());
        result.copy_from(&img, 0, 0)?;

        let mut out = Vec::new();
        result.write_to(&mut out, ImageOutputFormat::Jpeg(quality))?;

        Ok(out)
    }
}

impl<E: std::error::Error> From<E> for Error {
    fn from(e: E) -> Self {
        let message = e.to_string();

        let causes = std::iter::successors(e.source(), |e| e.source())
            .map(|e| e.to_string())
            .collect();

        Error { message, causes }
    }
}
