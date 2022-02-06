mod simple_string;
pub use simple_string::SimpleString;

mod simple_error;
pub use simple_error::SimpleError;

mod blob_string;
pub use blob_string::BlobString;

mod number;
pub use number::Number;

mod null;
pub use null::Null;

mod double;
pub use double::Double;

static DELIMITER: &str = "\r\n";
