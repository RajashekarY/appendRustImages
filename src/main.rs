mod args;
use args::Args;
use image::{io::Reader,DynamicImage,ImageFormat, imageops::FilterType::Triangle,GenericImageView};
use std::{io::BufReader,fs::File};

#[derive(Debug)]
enum ImageDataErrors{
    DifferentImageFormats,
    BufferTooSmall,
}

struct FloatingImage{
    width: u32,
    height: u32,
    data: Vec<u8>,
    name:String,
}

impl FloatingImage{
    fn new(width:u32,height:u32,name:String)-> Self{
        let buffer_capacity = 9_000_000;//3_655_744;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage{
            width,
            height,
            data:buffer,
            name,
        }
    }
    fn set_data(&mut self, data:Vec<u8>) -> Result<(),ImageDataErrors>{
        if data.len() > self.data.capacity(){
            println!("DataLen : {}",data.len());

            return Err(ImageDataErrors::BufferTooSmall);
        }
        self.data = data;
        Ok(())
    }
}

// fn main() -> Result<(), ImageDataErrors>  {
//     let args = Args::new();
//     let (image1,image1_format) = find_image_from_path(args.image1);
//     let (image2, image2_format) = find_image_from_path(args.image2);
//     // println!("{:?}",args);
//     if image1_format != image2_format{
//         return Err(ImageDataErrors::DifferentImageFormats);
//     }

//     let (image1,image2) = standardise_size(image1,image2);
//     let mut output = FloatingImage::new(image1.width(),image1.height(),args.output);
    
//     let combined_data = combine_images(image1,image2);
//     output.set_data(combined_data)?;

//     image::save_buffer_with_format(output.name, &output.data, output.width,output.height, image::ColorType::Rgba8,image1_format).unwrap();
//     Ok(());
// }

fn main() -> Result<(), ImageDataErrors> {
    let args = Args::new();
    println!("{:?}", args);
  
    let (image1, image1_format) = find_image_from_path(args.image1);
    let (image2, image2_format) = find_image_from_path(args.image2);
  
    if image1_format != image2_format {
      return Err(ImageDataErrors::DifferentImageFormats);
    }
  
    let (image1, image2) = standardise_size(image1, image2);
    let mut output = FloatingImage::new(image1.width(), image1.height(), args.output);
  
    let combined_data = combine_images(image1, image2);
  
    output.set_data(combined_data)?;
  
    image::save_buffer_with_format(
      output.name,
      &output.data,
      output.width,
      output.height,
      image::ColorType::Rgba8,
      image1_format,
    )
    .unwrap();
    Ok(())
  }

fn find_image_from_path(path:String) -> (DynamicImage,ImageFormat){
    let image_reader:Reader<BufReader<File>> = Reader::open(path).unwrap();
    let image_format:ImageFormat = image_reader.format().unwrap();
    let image:DynamicImage = image_reader.decode().unwrap();
    (image,image_format)
}

fn get_smallest_dimension(dim1:(u32,u32),dim2:(u32,u32))->(u32,u32){
    let pix1 = dim1.0 * dim1.1;
    let pix2 = dim2.0 * dim2.1;

    return if pix1 < pix2 {dim1} else {dim2};
}

fn standardise_size(image1:DynamicImage,image2:DynamicImage) -> (DynamicImage,DynamicImage){
    let (width,height) = get_smallest_dimension(image1.dimensions(),image2.dimensions());
    println!("Width:{} height: {}\n",width,height);


    if image2.dimensions() == (width,height){
        (image1.resize_exact(width,height,Triangle), image2)
    } else {
        (image1,image2.resize_exact(width,height,Triangle))
    }
}

fn combine_images(image1:DynamicImage, image2:DynamicImage)-> Vec<u8>{
    let vec_1 = image1.to_rgba8().into_vec();
    let vec_2 = image2.to_rgba8().into_vec();

    alternate_pixels(vec_1,vec_2)
}

fn alternate_pixels(vec_1:Vec<u8>,vec_2:Vec<u8>)->Vec<u8>{
    let mut combine_data = vec![0u8;vec_1.len()];
    let mut i = 0;
    while i < vec_1.len(){
        if i % 8 ==0{
            combine_data.splice(i..=i+3,set_rgba(&vec_1,i,i+3));
        } else {
            combine_data.splice(i..=i+3,set_rgba(&vec_2,i,i+3));
        }
        i+=4;
    }
    combine_data
}


fn set_rgba(vec:&Vec<u8>,start:usize,end:usize) -> Vec<u8>{
    let mut rgba = Vec::new();
    for i in start..=end{
        let val:u8 = match vec.get(i){
            Some(d)=>*d,
            None=> panic!("Index Out of Bounds"),
        };
        rgba.push(val);
    }
    rgba
}