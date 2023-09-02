pub struct TaskInfo{
    pub name: String,
    pub refresh_rate_hz: u32
}

pub trait UAVTask{
    fn get_info(&self) -> TaskInfo;
    fn boot(&mut self);
    fn init(&mut self);
    fn update(&mut self);
    fn shutdown(&mut self);
}