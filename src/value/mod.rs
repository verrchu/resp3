mod simple_string;
pub use simple_string::SimpleString;

mod simple_error;
pub use simple_error::SimpleError;

mod blob_string;
pub use blob_string::BlobString;

static DELIMITER: &str = "\r\n";
