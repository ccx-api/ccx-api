use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Default, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Page {
    cursor: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sort_direction: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct NextPage {
    /// Cursor to navigate to next page
    next_cursor: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    limit: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    sort_direction: Option<String>,
    has_next: bool,
}

impl Page {
    pub fn with_limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(&self) -> Option<&str> {
        self.cursor.as_deref()
    }

    pub fn limit(&self) -> Option<&u32> {
        self.limit.as_ref()
    }

    pub fn sort_direction(&self) -> Option<&str> {
        self.sort_direction.as_deref()
    }
}

impl NextPage {
    pub fn next(self) -> Option<Page> {
        let cursor = if self.next_cursor.is_empty() {
            None
        } else {
            Some(self.next_cursor)
        };
        if self.has_next && cursor.is_some() {
            Some(Page {
                cursor,
                limit: self.limit,
                sort_direction: self.sort_direction,
            })
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub(crate) fn fix_limit(&mut self, limit: Option<u32>) {
        self.limit = limit;
    }
}
