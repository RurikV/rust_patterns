trait Strategy {
    fn execute(&self, data: &str) -> String;
}

struct ConcreteStrategyA;
impl Strategy for ConcreteStrategyA {
    fn execute(&self, data: &str) -> String {
        format!("Strategy A applied to {}", data)
    }
}

struct ConcreteStrategyB;
impl Strategy for ConcreteStrategyB {
    fn execute(&self, data: &str) -> String {
        format!("Strategy B applied to {}", data)
    }
}

struct Context<'a> {
    strategy: &'a dyn Strategy,
}

impl<'a> Context<'a> {
    fn new(strategy: &'a dyn Strategy) -> Self {
        Context { strategy }
    }

    fn execute_strategy(&self, data: &str) -> String {
        self.strategy.execute(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strategy_a() {
        let strategy_a = ConcreteStrategyA;
        let context = Context::new(&strategy_a);
        assert_eq!(
            context.execute_strategy("test"),
            "Strategy A applied to test"
        );
    }

    #[test]
    fn test_strategy_b() {
        let strategy_b = ConcreteStrategyB;
        let context = Context::new(&strategy_b);
        assert_eq!(
            context.execute_strategy("test"),
            "Strategy B applied to test"
        );
    }
}

fn main() {
    let strategy_a = ConcreteStrategyA;
    let context = Context::new(&strategy_a);
    println!("{}", context.execute_strategy("test"));

    let strategy_b = ConcreteStrategyB;
    let context = Context::new(&strategy_b);
    println!("{}", context.execute_strategy("test"));
}
