use types::impl_string_value;
use garde::rules::url::Url;

impl_string_value!(ThumbnailUrl);

impl Url for ThumbnailUrl {
    type Error = garde::Error;
    fn validate_url(&self) -> Result<(), Self::Error> {
        // Assuming the ThumbnailUrl is a valid URL string, we can use the garde crate to validate it.
        // If the URL is invalid, it will return an error.
        garde::rules::url::Url::validate_url(&self.0)
            .map_err(|_| garde::Error::new("Invalid Thumbnail URL"))
    }
}