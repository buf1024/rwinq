use std::collections::HashMap;

use rwqcmm::RtQuot;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuotEvent {
    Open,
    MorningOpen,
    MorningClose,
    NoonOpen,
    NoonClose,
    Close,
    Quot(HashMap<String, RtQuot>),
}
