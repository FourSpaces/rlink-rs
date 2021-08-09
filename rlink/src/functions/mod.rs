pub mod filter;
pub mod flat_map;
pub mod key_selector;
pub mod percentile;
pub mod reduce;
pub mod sink;
pub mod source;
pub mod system;
pub mod watermark;
pub mod window;

pub trait FunctionSchema {
    fn schema_types(&self) -> Vec<u8>;
}
