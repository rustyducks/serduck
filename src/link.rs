use anyhow::Result;


pub trait Link {
    fn send_msg(&self, t: usize) -> Result<()>;
    fn stop(self);
}
