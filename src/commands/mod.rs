use log::info;

use crate::error::HMSimError;
use self::generate_tape_trace::TapeTrace;

pub mod origin_to_sim;
pub mod trace_property;
// pub mod generate_trace;
pub mod generate_tape_trace;

pub struct Pine;

impl Pine {
    
    pub fn generate_trace(&self) -> Result<(), HMSimError> {
        Ok(())
    }


    pub fn trace_property(&self, file: &str) -> Result<(), HMSimError> {
        let (footprint, volume, jumping, entropy) = trace_property::trace_property(file).unwrap();
        info!("TraceFile: {}", file);
        info!("Footprint: {:>7}", footprint);
        info!("Volume: {:>10}", volume);
        info!("AJD: {:>13.0}", jumping);
        info!("Entropy: {:>9.4}", entropy);
        Ok(())
    }


    pub fn origin_to_sim(&self, file: &str, timestamp: bool) -> Result<(), HMSimError> {
        origin_to_sim::origin_to_sim(file, timestamp)
    }

    pub fn generate_tape_trace(&self, tape_trace_struct: &mut TapeTrace) -> Result<(), HMSimError> {
        generate_tape_trace::generate_tape_trace(tape_trace_struct)
    }
}
