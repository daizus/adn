#[derive(Debug)]
pub struct AppliedChange {
    pub iface: String,
    pub action: ApplyAction,
}

#[derive(Debug)]
pub enum ApplyAction {
    CreatedInterface,
    AssignedStaticIp(String),
    RanDhcp,
}
