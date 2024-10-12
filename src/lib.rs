use std::io::Cursor;
use webp::*;
use neon::prelude::*;
use image_dds::ddsfile;
use neon::types::buffer::TypedArray;
use image::{imageops::FilterType, DynamicImage, ImageFormat};
use base64::{Engine as _, engine::general_purpose};

fn convert_dds(mut cx: FunctionContext) -> JsResult<JsString> {
    let buffer = cx.argument::<JsArrayBuffer>(0)?;
    let max_size = cx.argument::<JsNumber>(1)?;
    let input_vector = Cursor::new(buffer.as_slice(&mut cx));

    let dds = ddsfile::Dds::read(input_vector).unwrap();
    let original_image = image_dds::image_from_dds(&dds, 0).unwrap();

    let dims = original_image.dimensions();
    let max_size_u32 = max_size.value(&mut cx) as u32;

    if dims.0 > max_size_u32 || dims.1 > max_size_u32 {
        let scaled_image = DynamicImage::ImageRgba8(original_image).resize(max_size_u32, max_size_u32, FilterType::Nearest);
        let encoder: Encoder = Encoder::from_image(&scaled_image).unwrap();
        let webp: WebPMemory = encoder.encode(75f32);

        let b64 = general_purpose::STANDARD.encode(webp.as_ref());

        Ok(cx.string(format!("data:image/webp;base64, {b64}")))
        
    } else {
        let unscaled_image = DynamicImage::ImageRgba8(original_image);
        let encoder: Encoder = Encoder::from_image(&unscaled_image).unwrap();
        let webp: WebPMemory = encoder.encode(75f32);

        let b64 = general_purpose::STANDARD.encode(webp.as_ref());

        Ok(cx.string(format!("data:image/webp;base64, {b64}")))
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("convert_dds", convert_dds)?;
    Ok(())
}


