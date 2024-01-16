pub mod origin_to_sim;
pub mod trace_foot_size;
pub mod generate_trace;

pub struct Pine;

impl Pine {
    pub fn generate_trace(&self) {

    }

    pub fn trace_foot_size(&self, file: String) {

    }

    pub fn origin_to_sim(&self, file: String, time: bool) {
        println!("{:?}", time);
    }
}
