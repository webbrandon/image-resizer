use std::fmt;
use std::time::SystemTime;

pub struct ElapsedMetrics {
    load:String,
    resize:String,
    save:String,
    timer:SystemTime,
}

impl ElapsedMetrics {
    pub fn new() -> ElapsedMetrics {
        ElapsedMetrics {
            load: String::new(),
            resize: String::new(),
            save: String::new(),
            timer: SystemTime::now(),
        }
    }
    pub fn start(&mut self) {
        self.timer = SystemTime::now();
    }
    pub fn set_load_time(&mut self) {
        match self.timer.elapsed() {
           Ok(elapsed) => {
               self.load = format!("{}.{:03}",elapsed.as_secs(),elapsed.subsec_millis());
           }
           Err(e) => {
               // an error occurred!
               println!("Error: {:?}", e);
           }
       }
    }
    pub fn set_resize_time(&mut self) {
        match self.timer.elapsed() {
           Ok(elapsed) => {
               self.resize = format!("{}.{:03}",elapsed.as_secs(),elapsed.subsec_millis());
           }
           Err(e) => {
               // an error occurred!
               println!("Error: {:?}", e);
           }
       }
    }
    pub fn set_save_time(&mut self) {
        match self.timer.elapsed() {
           Ok(elapsed) => {
               self.save = format!("{}.{:03}",elapsed.as_secs(),elapsed.subsec_millis());
           }
           Err(e) => {
               // an error occurred!
               println!("Error: {:?}", e);
           }
       }
    }
}

impl fmt::Debug for ElapsedMetrics {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "load: {}\nresize: {}\nsave: {}",
        self.load, self.resize, self.save)
    }
}