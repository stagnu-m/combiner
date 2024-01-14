// custom modules
mod args;
mod image_helper;
mod floating_image;

// standard imports
use image::GenericImageView;

// custom imports
use crate::floating_image::FloatingImage;
use crate::args::Args;
use crate::image_helper::image_helper:: {
    ImageDataErrors,
    find_image_from_path,
    standardise_size,
    combine_images
};

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:?}", args);
  
    let (image_1, image_1_format) = find_image_from_path(args.image_1);
    let (image_2, image_2_format) = find_image_from_path(args.image_2);
  
    if image_1_format != image_2_format {
      return Err(ImageDataErrors::DifferentImageFormats);
    }
    let (image_1, image_2) = standardise_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.output);
    let combined_data = combine_images(image_1, image_2);
    output.set_data(combined_data)?;
    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_1_format,
      )
      .unwrap();
    Ok(())
}