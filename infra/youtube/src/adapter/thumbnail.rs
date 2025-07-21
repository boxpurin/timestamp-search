use domains::value_objects::thumbnail::Thumbnail as ThumbnailDomain;
use domains::value_objects::thumbnail_url::ThumbnailUrl;
use google_youtube3::api::{Thumbnail, ThumbnailDetails, Video};

/// Converter from YouTube Video to ThumbnailDetails
pub struct VideoToThumbnailConverter(pub Video);
pub struct ThumbnailsToThumbnailConverter(pub ThumbnailDetails);
pub struct ThumbnailToThumbnailConverter(pub Thumbnail);

impl TryInto<ThumbnailDomain> for VideoToThumbnailConverter {
    type Error = String;

    fn try_into(self) -> Result<ThumbnailDomain, Self::Error> {
        let inner = self.0;
        let thumbnail_details = inner
            .snippet
            .and_then(|s| s.thumbnails)
            .ok_or("Thumbnails are missing")?;

        ThumbnailsToThumbnailConverter(thumbnail_details).try_into()
    }
}

impl TryInto<ThumbnailDomain> for ThumbnailsToThumbnailConverter {
    type Error = String;

    fn try_into(self) -> Result<ThumbnailDomain, Self::Error> {
        let inner = self.0;
        let default_thumbnail = inner.default.ok_or("Default thumbnail is missing")?;

        ThumbnailToThumbnailConverter(default_thumbnail).try_into()
    }
}

impl TryInto<ThumbnailDomain> for ThumbnailToThumbnailConverter {
    type Error = String;

    fn try_into(self) -> Result<ThumbnailDomain, Self::Error> {
        let inner = self.0;
        let url = inner
            .url
            .map(ThumbnailUrl::new)
            .ok_or("Thumbnail URL is missing")?;
        let width = inner.width.ok_or("Thumbnail width is missing")?;
        let height = inner.height.ok_or("Thumbnail height is missing")?;

        Ok(ThumbnailDomain::new(url, width, height))
    }
}
