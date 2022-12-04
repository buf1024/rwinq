use crate::MarketType;

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
