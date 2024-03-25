use crate::error::HMSimError;

use self::generate_tape_trace::TapeTrace;

pub mod origin_to_sim;
pub mod trace_foot_size;
// pub mod generate_trace;
pub mod generate_tape_trace;

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

    pub fn generate_tape_trace(&self, tape_trace_struct: TapeTrace) -> Result<(), HMSimError> {
        generate_tape_trace::generate_tape_trace(tape_trace_struct)
    }
}
