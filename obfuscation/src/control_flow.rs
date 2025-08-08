use rand::{Rng, SeedableRng};
use rand::rngs::StdRng;

/// Generate opaque predicates for control flow obfuscation
pub struct OpaquePredicateGenerator {
    seed: u64,
}

impl OpaquePredicateGenerator {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Generate a complex condition that always evaluates to true
    pub fn always_true_condition(&self) -> String {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let x = rng.gen_range(1..100);
        let y = rng.gen_range(1..100);
        
        format!("({} * {} + {}) % 2 == (({} + {}) * 2) % 2", x, y, x + y, x, y)
    }

    /// Generate a complex condition that always evaluates to false
    pub fn always_false_condition(&self) -> String {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let x = rng.gen_range(10..100);
        let y = rng.gen_range(10..100);
        
        format!("({} * {}) == (({} + 1) * ({} + 1))", x, y, x, y)
    }

    /// Generate dummy computational code
    pub fn generate_dummy_computation(&self) -> String {
        let mut rng = StdRng::seed_from_u64(self.seed);
        let operations = ["^", "&", "|", "+", "-", "*"];
        let mut computation = String::new();
        
        let var_count = rng.gen_range(3..8);
        for i in 0..var_count {
            let val = rng.gen_range(1..255);
            computation.push_str(&format!("let dummy_{} = {};\n", i, val));
        }
        
        for i in 0..var_count - 1 {
            let op = operations[rng.gen_range(0..operations.len())];
            let next_var = (i + 1) % var_count;
            computation.push_str(&format!(
                "let dummy_{}_result = dummy_{} {} dummy_{};\n", 
                i, i, op, next_var
            ));
        }
        
        computation
    }
}

/// Control flow flattening helpers
pub struct ControlFlowFlattener {
    state_counter: usize,
}

impl ControlFlowFlattener {
    pub fn new() -> Self {
        Self { state_counter: 0 }
    }

    pub fn generate_state_machine_skeleton(&mut self) -> String {
        let initial_state = self.next_state();
        format!(
            r#"
let mut obf_state = {};
loop {{
    match obf_state {{
        // States will be inserted here
        {} => break,
        _ => unreachable!(),
    }}
}}
"#,
            initial_state,
            self.next_state()
        )
    }

    pub fn next_state(&mut self) -> usize {
        self.state_counter += 1;
        self.state_counter
    }
}

impl Default for ControlFlowFlattener {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_opaque_predicates() {
        let generator = OpaquePredicateGenerator::new(12345);
        let true_condition = generator.always_true_condition();
        let false_condition = generator.always_false_condition();
        
        assert!(!true_condition.is_empty());
        assert!(!false_condition.is_empty());
        assert_ne!(true_condition, false_condition);
    }
}
