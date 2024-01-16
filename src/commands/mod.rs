use crate::error::HMSimError;

pub mod origin_to_sim;
pub mod trace_foot_size;
pub mod generate_trace;

pub struct Pine;

impl Pine {
    
    pub fn generate_trace(&self) -> Result<(), HMSimError> {
        Ok(())
    }


    pub fn trace_foot_size(&self, file: String) -> Result<(), HMSimError> {
        Ok(())
    }


    pub fn origin_to_sim(&self, file: &str, timestamp: bool) -> Result<(), HMSimError> {
        origin_to_sim::origin_to_sim(file, timestamp)
    }
}
