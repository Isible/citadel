use std::fmt::Debug;

use crate::experimental::elements::Operand;

pub(crate) trait VecDisplay: Debug {
    fn to_string(&self) -> String;
}

impl VecDisplay for Vec<Operand> {
    fn to_string(&self) -> String {
        self.iter()
            .map(|op| {
                let mut str = op.to_string();
                str.push(',');
                str
            })
            .collect()
    }
}
