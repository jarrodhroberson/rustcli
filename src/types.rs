use serde::{Deserialize, Serialize, Serializer};

#[derive(Debug, Serialize, Deserialize)]
pub struct Directory {
    pub name: String,
    pub contents: Vec<File>,
}

fn as_string<S,N: ToString>(n: N, s: S) -> Result<S::Ok, S::Error>
                            where
                                S: Serializer,
{
    s.serialize_str(&*n.to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct File {
    pub name: String,
    #[serde(serialize_with = "as_string")]
    pub size: u64,
    #[serde(serialize_with = "as_string")]
    pub permissions: u16,
    pub last_modified: String,
}

