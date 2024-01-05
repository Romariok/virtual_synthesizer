
use std::{sync::Arc, io::BufReader, path::Path, fs::File};

use flacenc::component::BitRepr;
use rodio::OutputStreamHandle;
use winit::event::VirtualKeyCode;

use crate::data::{virtual_keycode_to_string, BasicPiano};




pub async fn play_sound(handle: Arc<OutputStreamHandle>, keycode: VirtualKeyCode, style: String) {
   let file_name = virtual_keycode_to_string(keycode) + ".flac";
   let file_path = format!("sounds/{}/{}", style, file_name.clone());

   let file = File::open(Path::new(&file_path)).unwrap();
   let source = rodio::Decoder::new(BufReader::new(file)).unwrap();

   let sink = rodio::Sink::try_new(&handle).unwrap();
   sink.append(source);
   println!("{:?}", virtual_keycode_to_string(keycode));
   sink.sleep_until_end();
}

pub fn generate_flacs(styles: Vec<BasicPiano>) {
   for style in styles {
     for (key, value) in style.as_map() {
       let input_file = format!("sounds/{}/{}.flac", style.name ,key);
       let (channels, bits_per_sample, sample_rate) = (1, 16, 41100);
       let config = flacenc::config::Encoder::default();
       let source = flacenc::source::MemSource::from_samples(
          value.as_slice(),
          channels,
          bits_per_sample,
          sample_rate,
       );
       let flac_stream =
          flacenc::encode_with_fixed_block_size(&config, source, config.block_sizes[0])
             .expect("Encode failed.");
       let mut sink = flacenc::bitsink::ByteSink::new();
       flac_stream.write(&mut sink).expect("Write failed.");
       std::fs::create_dir_all(Path::new(&input_file).parent().unwrap()).unwrap();
       std::fs::write(input_file, sink.as_slice()).expect("Write failed.");
    }
   }
}