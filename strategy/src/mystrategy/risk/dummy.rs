use async_trait::async_trait;

use crate::risk::Risk;

pub struct Dummy;

impl Dummy {
    pub fn new() -> Self {
        Self {}
    }
}

#[async_trait]
impl Risk for Dummy {
    fn description(&self) -> String {
        String::from(
            r#"Dummy 风控策略

不做任何风控，完全放飞自我。"#,
        )
    }
    fn name(&self) -> String {
        String::from("Dummy -- 无风控")
    }
}
