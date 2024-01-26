use crate::error::HMSimError;

pub mod origin_to_sim;
pub mod trace_foot_size;
// pub mod generate_trace;
pub mod today;

pub struct Pine;

impl Pine {
    
    pub fn generate_trace(&self) -> Result<(), HMSimError> {
        Ok(())
    }


    pub fn trace_foot_size(&self, file: &str) -> Result<(), HMSimError> {
        let (footprint, volume) = trace_foot_size::trace_foot_size(file).unwrap();
        println!("tracefile: {}\nfootprint: {:>7}\nvolume: {:>10}", file, footprint, volume);
        Ok(())
    }


    pub fn origin_to_sim(&self, file: &str, timestamp: bool) -> Result<(), HMSimError> {
        origin_to_sim::origin_to_sim(file, timestamp)
    }

    pub fn today(&self) -> Result<(), HMSimError> {
        today::today();
        Ok(())
    }
}
