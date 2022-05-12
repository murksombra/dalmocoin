pub struct Output {
    pub destination: Address,
    pub value: u64,
}

pub struct Transaction {
    pub inputs: Vec<Output>,
    pub outputs: Vec<Output>,
}
