#[derive(Debug, Clone)]
pub enum Languages {
    Java,
    Kotlin
}

impl Languages {
    pub fn as_str(self: &Self) -> &'static str {
        match self {
            Languages::Java => "java",
            Languages::Kotlin => "kotlin",
        }
    }
}
