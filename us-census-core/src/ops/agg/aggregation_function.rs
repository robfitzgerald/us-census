use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NumericAggregation {
    Sum,
    Mean,
}

impl NumericAggregation {
    pub fn aggregate(&self, values: &mut dyn Iterator<Item = f64>) -> f64 {
        use NumericAggregation as Fn;
        match self {
            Fn::Sum => values.fold(0.0, |acc, v| acc + v),
            Fn::Mean => {
                let (acc, n) = values.fold((0.0, 0.0), |(acc, n), v| (acc + v, n + 1.0));
                if n == 0.0 {
                    0.0
                } else {
                    acc / n
                }
            }
        }
    }
}
