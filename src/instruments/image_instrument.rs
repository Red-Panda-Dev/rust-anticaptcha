use std::fs;
use std::io::ErrorKind;
use std::io::Read;

use base64::alphabet::STANDARD;
use base64::engine::general_purpose;
use base64::engine::Engine;
use base64::engine::GeneralPurpose;

use crate::core::request_interface::RequestInterface;

/// Struct help in encoding image from file/url to base64 string
///
/// # Examples
///
/// ```
/// let image_instrument = ImageInstrument::new();
/// ```
pub struct ImageInstrument {
    engine: GeneralPurpose,
}
impl ImageInstrument {
    /// Method init new ImageInstrument struct with base64 engine
    ///
    /// # Examples
    ///
    /// ```
    /// let image_instrument = ImageInstrument::new();
    /// ```
    ///
    /// # Returns
    /// Method return `ImageInstrument` instance
    ///
    pub fn new() -> Self {
        ImageInstrument {
            engine: GeneralPurpose::new(&STANDARD, general_purpose::PAD),
        }
    }

    /// Method read image file and return base64 string
    ///
    /// # Arguments
    /// `file_path` - path to image on local system
    ///
    /// # Examples
    ///
    /// ```
    /// let image_instrument = ImageInstrument::new();
    /// let base64_str = image_instrument.read_image_file("files/image.jpg".to_string());
    /// ```
    ///
    /// # Returns
    /// Method return image as base64 string
    ///
    pub fn read_image_file(&self, file_path: String) -> String {
        let contents = match fs::read(&file_path) {
            Ok(content) => content,
            Err(error) => match error.kind() {
                ErrorKind::NotFound => panic!("File `{file_path}` is not exists!"),
                _ => panic!("Can't read file - {error}"),
            },
        };
        self.engine.encode(contents)
    }

    /// Method read image file and return base64 string
    ///
    /// # Arguments
    /// `file_path` - path to image on local system
    ///
    /// # Examples
    ///
    /// ```
    /// let image_instrument = ImageInstrument::new();
    /// let base64_str = image_instrument.read_image_link("https://some-file-url.jpg".to_string());
    /// ```
    ///
    /// # Returns
    /// Method return image as base64 string
    ///
    pub async fn read_image_link(&self, file_url: String) -> String {
        let request_client = RequestInterface::new();
        let result = match request_client.send_get_request(&file_url).await {
            Ok(result) => result,
            Err(error) => panic!("{}", error),
        };
        self.engine.encode(result.bytes().await.unwrap())
    }
}
