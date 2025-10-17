use fancy_regex::Regex;
use errors::AppResult;
use domains::entities::{
    video::VideoEntity,
    video_timestamp::VideoTimestampEntity,
};
use domains::value_objects::{
    video_description::VideoDescription,
    timestamp::TimeStamp,
    timestamp_description::TimeStampDescription,
    elapsed_time::ElapsedTime
};

#[derive(Default)]
pub struct TimeStampParserService{}

impl TimeStampParserService {
    pub fn new() -> Self { Self{} }

    pub fn parse_video(&self, video: &VideoEntity) -> AppResult<Vec<VideoTimestampEntity>> {
        let mut result = vec![];

        let tss = Self::parse(&video.description)?;
        for ts in tss {
            let v = VideoTimestampEntity::new(video.id.clone(), ts);
            result.push(v);
        }

        Ok(result)
    }


    fn parse(description: &VideoDescription) -> AppResult<Vec<TimeStamp>> {
        let re = Regex::new(r"([0-9]{0,2}:*[0-9]{1,2}:[0-9]{1,2})\s+([\s\S]*?)(?=\n{2}|[0-9]{0,2}:*[0-9]{1,2}:[0-9]{1,2}|$)").unwrap();
        let ret: Vec<TimeStamp> = re
            .captures_iter(description)
            .filter(|caps| caps.is_ok())
            .filter_map(|caps| {
                let caps = caps.ok()?;
                let time = caps.get(1)?.as_str().trim();
                let text = caps.get(2)?.as_str().trim();
                let time = ElapsedTime::from_hhmmss(time).ok()?;
                let d = TimeStampDescription::new(text).ok()?;
                Some(TimeStamp::new(time, d).unwrap())
            })
            .collect();
        Ok(ret)
    }
}

#[cfg(test)]
mod unit_tests{
    use super::*;
    use domains::value_objects::video_description::VideoDescription;

    #[rstest::rstest]
    #[test]
    #[case::single_line_single_item("01:10 test Description.", 1)]
    #[case::multi_line_single_item(r#"01:10 test Description.
    Second rows"#, 1)]
    #[case::multi_line_multi_item(r#"01:10 test Description.
    01:12 test Description."#, 2)]
    #[case::single_line_multi_item("01:10 test Description. 01:12 test Description.", 2)]
    fn valid_parse(#[case] description: &str, #[case] expected_num: usize) {
        let d = VideoDescription::new(description).unwrap();

        let v = TimeStampParserService::parse(&d);
        assert!(v.is_ok());
        assert_eq!(expected_num, v.unwrap().len());
    }
}