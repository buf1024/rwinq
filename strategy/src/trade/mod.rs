use rwqdata::RtQuot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Open,
    MorningOpen,
    MorningClose,
    NoonOpen,
    NoonClose,
    Close,
    Quot(Vec<RtQuot>),
}
