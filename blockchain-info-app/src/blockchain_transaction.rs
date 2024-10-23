#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vin { 
    pub n: i64,
    pub addresses: Vec<String>, 
    pub is_address: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Vout {
    pub value: String,
    pub n: i64,
    pub addresses: Vec<String>,
    pub is_address: bool,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainTransaction {
    pub txid: String, 
    pub vin: Vec<Vin>,
    pub vout: Vec<Vout>,
    pub fees: String,
}