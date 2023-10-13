mod data;
mod services;

use data::collection::BasicPiano;

extern crate hound; // Библиотека для записи аудиофайлов

fn main() {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    {
        let basic: BasicPiano = BasicPiano::initBasic();
        // let note_basic: Vec<Vec<i16>> = Vec::from([basic.A, basic.B, basic.C, basic.D, basic.E, basic.F, basic.G]);


        for (key, value) in basic.as_map() {
            let mut writer = hound::WavWriter::
            create("resources/basic/".to_string()+key+".wav", spec).unwrap();
            for sample in value{
                writer.write_sample(sample).unwrap();
            }

        }
    }
}
