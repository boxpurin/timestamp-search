use domains::value_objects::thumbnail::Thumbnail as ThumbnailDomain;
use domains::value_objects::thumbnail_url::ThumbnailUrl;
use errors::AppError::{self, DomainParseError};
use google_youtube3::api::{Thumbnail, ThumbnailDetails, Video};

/// Converter from YouTube Video to ThumbnailDetails
pub struct VideoToThumbnailConverter(pub Video);
pub struct ThumbnailsToThumbnailConverter(pub ThumbnailDetails);
pub struct ThumbnailToThumbnailConverter(pub Thumbnail);

impl TryInto<ThumbnailDomain> for VideoToThumbnailConverter {
    type Error = AppError;

    fn try_into(self) -> Result<ThumbnailDomain, Self::Error> {
        let inner = self.0;
        let thumbnail_details = inner
            .snippet
            .and_then(|s| s.thumbnails)
            .ok_or(DomainParseError("Thumbnails are missing".to_string()))?;

        ThumbnailsToThumbnailConverter(thumbnail_details).try_into()
    }
}

impl TryInto<ThumbnailDomain> for ThumbnailsToThumbnailConverter {
    type Error = AppError;

    fn try_into(self) -> Result<ThumbnailDomain, Self::Error> {
        let inner = self.0;
        let default_thumbnail = inner
            .default
            .ok_or(DomainParseError("Default thumbnail is missing".to_string()))?;

        ThumbnailToThumbnailConverter(default_thumbnail).try_into()
    }
}

impl TryInto<ThumbnailDomain> for ThumbnailToThumbnailConverter {
    type Error = AppError;

    fn try_into(self) -> Result<ThumbnailDomain, Self::Error> {
        let inner = self.0;
        let url = inner
            .url
            .map(|s| ThumbnailUrl::new(&s).unwrap())
            .ok_or(DomainParseError("Thumbnail URL is missing".to_string()))?;
        let width = inner
            .width
            .ok_or(DomainParseError("Thumbnail width is missing".to_string()))?;
        let height = inner
            .height
            .ok_or(DomainParseError("Thumbnail height is missing".to_string()))?;

        Ok(ThumbnailDomain::new(url, width, height)?)
    }
}
