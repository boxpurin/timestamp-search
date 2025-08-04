use errors::{AppError, AppResult};
types::impl_string_value!(SearchQueryText);

impl SearchQueryText {
    /// Note: 将来的にクエリの正規化やトークン化を行う可能性があるため、
    /// 現在は単純なラッパーとして実装しています。
    pub fn new(text: &str) -> AppResult<Self> {
        Ok(SearchQueryText(text.to_string()))
    }
}
