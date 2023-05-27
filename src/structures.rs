use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CmdDocumentation {
    pub command: String,
    pub information: String
}

