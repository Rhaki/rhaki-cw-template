use rhaki_cw_plus::deploy::Deploier;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Data {
    pub code_id: CodeId,
    pub addresses: Addresses,
    pub variables: Variables,
}

impl Deploier for Data {
    const PATH_ARTIFACTS: &'static str = "./artifacts";
    const PATH_CONFIG: &'static str = "./deploy";
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CodeId {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Addresses {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Variables {}
