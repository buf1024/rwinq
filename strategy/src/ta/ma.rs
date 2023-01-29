/// 从最近到最久
#[allow(non_snake_case)]
pub fn MA(bar: &Vec<f32>, ma_type: usize) -> Vec<f32> {
    let mut ma = Vec::new();
    let mut sum = 0.0;
    let len = bar.len();
    for (index, field) in bar.iter().rev().enumerate() {
        sum += *field;
        if index + 1 < ma_type {
            ma.push(0.0);
            continue;
        }
        ma.push(sum / ma_type as f32);
        sum -= *bar.get(len - 1 - (index + 1 - ma_type)).unwrap();
    }
    ma.reverse();
    ma
}


#[cfg(test)]
mod tests {
    use super::MA;

    #[test]
    fn test_ma() {
        let mut bar = Vec::new();
        for i in (1..=20).rev() {
            bar.push(i as f32);
        }
        let ma5 = MA(&bar, 5);
        println!("Orig: {:?}", bar);
        println!("MA5: {:?}", ma5);
    }
}