use std::io::Cursor;
use neon::prelude::*;
use image_dds::ddsfile;
use neon::types::buffer::TypedArray;
use image::{imageops::FilterType, DynamicImage, ImageFormat};
use base64::{Engine as _, engine::general_purpose};

static IMAGE_FORMAT: ImageFormat = image::ImageFormat::Png;
// static IMAGE_FORMAT: ImageFormat = image::ImageFormat::WebP;

fn convert_dds(mut cx: FunctionContext) -> JsResult<JsString> {
    let buffer = cx.argument::<JsArrayBuffer>(0)?;
    let max_size = cx.argument::<JsNumber>(1)?;
    let input_vector = Cursor::new(buffer.as_slice(&mut cx));

    let dds = ddsfile::Dds::read(input_vector).unwrap();
    let original_image = image_dds::image_from_dds(&dds, 0).unwrap();

    let dims = original_image.dimensions();
    let max_size_u32 = max_size.value(&mut cx) as u32;

    let mut output_cursor = Cursor::new(Vec::new());

    if dims.0 > max_size_u32 || dims.1 > max_size_u32 {
        let scaled_image = DynamicImage::ImageRgba8(original_image).resize(max_size_u32, max_size_u32, FilterType::Nearest);
        let result = scaled_image.write_to(&mut output_cursor, IMAGE_FORMAT);

        if result.is_ok() {
            let b64 = general_purpose::STANDARD.encode(output_cursor.into_inner());

            Ok(cx.string(format!("data:image/webp;base64, {b64}")))
        } else {
            Ok(cx.string(""))
        }
    } else {
        let result = original_image.write_to(&mut output_cursor, IMAGE_FORMAT);

        if result.is_ok() {
            let b64 = general_purpose::STANDARD.encode(output_cursor.into_inner());

            Ok(cx.string(format!("data:image/png;base64, {b64}")))
        } else {
            Ok(cx.string(""))
        }
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("convert_dds", convert_dds)?;
    Ok(())
}


