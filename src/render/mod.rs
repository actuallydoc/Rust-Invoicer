use image::ImageFormat;
use pdfium_render::prelude::*;
use pdfium_render::{prelude::PdfiumError, render_config::PdfRenderConfig};
pub fn export_pdf_to_jpegs(
    path: &str,
    file_path: &str,
    password: Option<&str>,
    invoice_number: i32,
) -> Result<(), PdfiumError> {
    // Renders each page in the PDF file at the given path to a separate JPEG file.

    // Bind to a Pdfium library in the same directory as our Rust executable;
    // failing that, fall back to using a Pdfium library provided by the operating system.

    let pdfium = Pdfium::new(
        Pdfium::bind_to_library(Pdfium::pdfium_platform_library_name_at_path("./pdfium/"))
            .or_else(|_| Pdfium::bind_to_system_library())?,
    );

    // Load the document from the given path...

    let document = pdfium.load_pdf_from_file(path, password)?;

    // ... set rendering options that will be applied to all pages...

    let render_config = PdfRenderConfig::new()
        .set_target_width(2000)
        .set_maximum_height(2000)
        .rotate_if_landscape(PdfBitmapRotation::Degrees90, true);

    // ... then render each page to a bitmap image, saving each image to a JPEG file.

    for (_, page) in document.pages().iter().enumerate() {
        page.render_with_config(&render_config)?
            .as_image() // Renders this page to an image::DynamicImage...
            .as_rgba8() // ... then converts it to an image::Image...
            .ok_or(PdfiumError::ImageError)?
            .save_with_format(
                format!("{}\\račun {}.jpg", file_path, invoice_number),
                ImageFormat::Jpeg,
            ) // ... and saves it to a file.
            .map_err(|_| PdfiumError::ImageError)?;
    }
   
    Ok(())
}
