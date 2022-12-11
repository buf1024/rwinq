use crate::MarketType;

/// 原始的代码表示为内部的代码格式，内部格式为sz/sh + 代码
/// 
/// # Examples
/// ```
/// use hiq_fetch::{to_std_code, MarketType};
/// let code = to_std_code(MarketType::Stock, "002805");
/// assert_eq!(code.as_str(), "sz002805");
/// ```
#[inline]
pub fn to_std_code(typ: MarketType, code: &str) -> String {
    if code.len() != 6 {
        return code.to_owned();
    }
    match typ {
        MarketType::Bond => {
            if code.starts_with("12") {
                format!("sz{}", code)
            } else {
                format!("sh{}", code)
            }
        }
        MarketType::Fund => {
            if code.starts_with("15") {
                format!("sz{}", code)
            } else {
                format!("sh{}", code)
            }
        }
        MarketType::Stock => {
            if code.starts_with("6") {
                format!("sh{}", code)
            } else {
                format!("sz{}", code)
            }
        }
    }
}
