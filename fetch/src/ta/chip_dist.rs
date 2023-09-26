use std::collections::BTreeMap;

use crate::{Error, Result};
use rwqcmm::Bar;
use serde::{Deserialize, Serialize};

/// 筹码分布计算。
///
/// 每家券商计算的筹码分布算法不一样，这里计算的和通达信，同花顺和东方财富大致相同，会有10%误差。
///
/// 筹码分布有两种方式，一种称为平均分配，误差较大，一般不使用，另外一种称为三角分布，如下图所示。  
/// ``` shell
///             _           ___  
///            /|\           ↑  
///       y2 /  | \  
///     y1 /|   |  \        h = 2/d  
///      /| |   |   \          
///     /_|_|___|____\      _↓_  
///   low x1 x2 avg  high
///       |st|  
///    |←     d     →|  
/// 三角形的面积是1，高度表示在对于价格的分布。
/// low 最低价 high 最高价 avg 平均价
/// 那么：d = high - low, h*d/2 = 1, y1=avg*(x1-low)/(avg-x1)
/// 因此，x2价上的分布为(面积)：area = st*(y1+y2)/2
/// ```
/// 历史衰减系数类似于加权，把最近日期的突出

/// float 不能做为key，所以保留2位后转换为int作为key。表示该价格下的筹码分布
pub type Chip = BTreeMap<i32, f64>;
/// 某日的筹码分布，key为yyyymmdd格式，转换为int
pub type ChipList = BTreeMap<i32, Chip>;
/// 筹码分布
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ChipDist {
    pub chip: Chip,
    pub chip_list: ChipList,
}

/// 计算筹码分布对于股票而已
///
/// `data` 为未计算的数据， ac衰减系数，不传默认为1 chip_dist,为算过的筹码，在次基础上计算不用太
pub fn calc_chip_dist(
    data: &Vec<Bar>,
    ac: Option<f32>,
    chip_dist: Option<ChipDist>,
) -> Result<ChipDist> {
    let (mut chip, mut chip_list) = chip_dist.map_or_else(
        || (BTreeMap::new(), BTreeMap::new()),
        |v| (v.chip.clone(), v.chip_list.clone()),
    );
    let last_trade_date = if let Some(entry) = chip_list.last_entry() {
        Some(*entry.key())
    } else {
        None
    };
    let ac = ac.unwrap_or(1.0);
    let st: f32 = 0.01;
    data.iter()
        .filter(|&bar| {
            if let Some(last_trade_date) = last_trade_date {
                let trade_date: i32 = bar.trade_date.format("%Y%m%d").to_string().parse().unwrap();
                trade_date > last_trade_date
            } else {
                true
            }
        })
        .for_each(|bar| {
            // 三角分布
            let st_len: isize = ((bar.high - bar.low) / st) as isize;
            let h = 2.0 / (bar.high - bar.low);
            let avg = (bar.amount / (bar.volume * 100) as f64) as f32;

            let mut die_chip = Vec::new();
            chip.iter_mut().for_each(|(key, value)| {
                *value = *value * (1.0 - (bar.turnover / 100.0) * ac) as f64;
                if *value < 1.0 {
                    die_chip.push(*key);
                }
            });
            die_chip.into_iter().for_each(|key| {
                chip.remove(&key);
            });

            for i in 0..st_len {
                let x1 = i as f32 * st + bar.low;
                let x2 = x1 + st;

                let (y1, y2) = if x1 < avg {
                    (
                        h * (x1 - bar.low) / (avg - bar.low),
                        h * (x2 - bar.low) / (avg - bar.low),
                    )
                } else {
                    (
                        h * (bar.high - x1) / (bar.high - avg),
                        h * (bar.high - x2) / (bar.high - avg),
                    )
                };

                let area = st * (y1 + y2) / 2.0;
                let dist = (area as f64 * bar.volume as f64) as f64;
                let key = (x1 * 100.0) as i32;
                let value = dist * (bar.turnover / 100.0 * ac) as f64;
                if value >= 1.0 {
                    if let Some(v) = chip.get_mut(&key) {
                        *v += value;
                    } else {
                        chip.insert(key, value);
                    }
                }
            }
            let trade_date: i32 = bar.trade_date.format("%Y%m%d").to_string().parse().unwrap();
            chip_list.insert(trade_date, chip.clone());
        });

    Ok(ChipDist { chip, chip_list })
}

/// 获利盘，计算每天的获利盘
///
/// chip_dist 筹码分布 data 原始数据(相对close盈利), price 指定价格盈利 优先使用data
pub fn calc_winner(
    chip_dist: &ChipDist,
    data: Option<&Vec<Bar>>,
    price: Option<f32>,
) -> Result<BTreeMap<i32, f64>> {
    let get_price = |i: usize| {
        if let Some(data) = data {
            data.get(i).map_or(0.0, |v| v.close)
        } else {
            price.unwrap_or(0.0)
        }
    };
    if data.is_none() && price.is_none() {
        return Err(Error::Custom(
            "data / price should not both none".to_owned(),
        ));
    }
    if data.is_some() && price.is_some() {
        return Err(Error::Custom(
            "data / price should not both some".to_owned(),
        ));
    }

    let profit = chip_dist
        .chip_list
        .iter()
        .enumerate()
        .map(|(index, (trade_date, chip))| {
            let mut profit = 0.0;
            let mut total = 0.0;
            let close = get_price(index);
            chip.iter().for_each(|(price, dist)| {
                total += dist;
                let price = *price as f32 / 100.0;
                if price < close {
                    profit += dist
                }
            });
            let ratio = if total > 0.0 { profit / total } else { 0.0 };
            (*trade_date, ratio)
        })
        .collect();

    Ok(profit)
}

/// 成本分布, `ratio`， 百分之几的成本的价格
pub fn calc_cost(chip_dist: &ChipDist, ratio: isize) -> Result<BTreeMap<i32, f64>> {
    let ratio = ratio as f64 / 100.0;
    let cost = chip_dist
        .chip_list
        .iter()
        .map(|(trade_date, chip)| {
            let sum: f64 = chip.values().sum();
            let mut sum_to_ratio = 0.0;
            let mut price = 0.0;
            for (key, dist) in chip {
                sum_to_ratio += *dist;
                if sum_to_ratio / sum >= ratio {
                    price = *key as f64 / 100.0;
                    break;
                }
            }
            (*trade_date, price)
        })
        .collect();
    Ok(cost)
}

#[cfg(test)]
mod tests {
    use chrono::NaiveDateTime;

    use crate::{calc_chip_dist, calc_cost, calc_winner, fetch_stock_bar};
    #[test]
    fn test_calc_winner() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let end = NaiveDateTime::parse_from_str("20230901 00:00:00", "%Y%m%d %H:%M:%S")
                    .unwrap()
                    .date();
                let bar = fetch_stock_bar("sz301421", None, None, None, Some(end), true)
                    .await
                    .unwrap();
                let bar = bar.bars.unwrap();
                let dist = calc_chip_dist(&bar, None, None).unwrap();

                let winner = calc_winner(&dist, Some(&bar), None).unwrap();
                winner.iter().for_each(|(key, value)| {
                    println!("winner1 key: {}, value: {}", *key, *value);
                });

                let bar = fetch_stock_bar("sz301421", None, None, None, None, true)
                    .await
                    .unwrap();
                let bar = bar.bars.unwrap();
                let dist = calc_chip_dist(&bar, None, Some(dist)).unwrap();

                let winner = calc_winner(&dist, Some(&bar), None).unwrap();
                winner.iter().for_each(|(key, value)| {
                    println!("winner2 key: {}, value: {}", *key, *value);
                });
            })
    }

    #[test]
    fn test_calc_cost() {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                let bar = fetch_stock_bar("sz301421", None, None, None, None, true)
                    .await
                    .unwrap();
                let bar = bar.bars.unwrap();
                let dist = calc_chip_dist(&bar, None, None).unwrap();

                let cost = calc_cost(&dist, 90).unwrap();
                cost.iter().for_each(|(key, value)| {
                    println!("cost key: {}, value: {}", *key, *value);
                });
            })
    }
}
