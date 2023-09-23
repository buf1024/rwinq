use chrono::NaiveDateTime;

pub struct Position {
    pub id: String,
    pub name: String,        // 股票名称
    pub code: String,        // 股票代码
    pub time: NaiveDateTime, // 首次建仓时间

    pub volume: i32,           // 持仓量
    pub volume_available: i32, // 可用持仓量
    pub volume_frozen: i32,    // 可用持仓量

    pub fee: f32,   // 持仓费用
    pub price: f32, // 平均持仓价

    pub now_price: f32, // 最新价
    pub max_price: f32, // 最高价
    pub min_price: f32, // 最低价

    pub profit_rate: f32,     // 盈利比例
    pub max_profit_rate: f32, // 最大盈利比例
    pub min_profit_rate: f32, // 最小盈利比例

    pub profit: f32,     // 盈利
    pub max_profit: f32, // 最大盈利
    pub min_profit: f32, // 最小盈利

    pub max_profit_time: Option<NaiveDateTime>, // 最大盈利时间
    pub min_profit_time: Option<NaiveDateTime>, // 最小盈利时间
}
