use std::fs;
use std::io::ErrorKind;
use std::io::Read;

use base64::alphabet::STANDARD;
use base64::engine::general_purpose;
use base64::engine::Engine;
use base64::engine::GeneralPurpose;

pub struct ImageInstrument {
    engine: GeneralPurpose,
}
impl ImageInstrument {
    pub fn new() -> Self {
        /// Method init new ImageInstrument struct with base64 engine
        ///
        /// # Examples
        ///
        /// ```
        /// let image_instrument = ImageInstrument::new();
        /// ```
        ImageInstrument {
            engine: GeneralPurpose::new(&STANDARD, general_purpose::PAD),
        }
    }
    pub fn read_image_file(&self, file_path: String) -> String {
        /// Method make able to read image file as base64 string
        ///
        /// # Examples
        ///
        /// ```
        /// let image_instrument = ImageInstrument::new();
        /// let base64_str = image_instrument.read_image_file("files/image.jpg".to_string());
        /// ```
        let contents = match fs::read(&file_path) {
            Ok(content) => content,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("File `{file_path}` is not exists!"),
                _ => panic!("Can't read file - {error}"),
            },
        };
        self.engine.encode(contents)
    }
}
