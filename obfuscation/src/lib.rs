pub mod string_obfuscation;
pub mod control_flow;
pub mod anti_debug;
pub mod binary_protection;
pub mod advanced_anti_analysis;
pub mod integration_example;

pub use string_obfuscation::*;
pub use control_flow::*;
pub use anti_debug::*;
pub use binary_protection::*;
pub use advanced_anti_analysis::*;
pub use integration_example::*;

/// Obfuscation configuration levels
#[derive(Clone, Copy, Debug)]
pub enum ObfuscationLevel {
    Light,
    Medium,
    Heavy,
    Maximum,
}

impl ObfuscationLevel {
    pub fn should_apply_string_obfuscation(&self) -> bool {
        matches!(self, ObfuscationLevel::Medium | ObfuscationLevel::Heavy | ObfuscationLevel::Maximum)
    }

    pub fn should_apply_control_flow_obfuscation(&self) -> bool {
        matches!(self, ObfuscationLevel::Heavy | ObfuscationLevel::Maximum)
    }

    pub fn should_apply_anti_debug(&self) -> bool {
        matches!(self, ObfuscationLevel::Maximum)
    }
}
