use chrono::{Local};
pub struct Log4rs;

impl Log4rs{
    pub fn new()->Log4rs{
        Log4rs{}.init()
    }
    fn init (self)->Self{
        let _ =log4rs::init_file("./conf/log4rs.yaml", Default::default()).unwrap();
        log::info!("开始加载项目, 时间为:[{}]...", get_date_time());
        self
    }
}

pub const FMT_Y_M_D_H_M_S: &str = "%Y-%m-%d %H:%M:%S";


pub fn get_date_time() -> String {
    let date_time = Local::now();
    date_time.format(FMT_Y_M_D_H_M_S).to_string()
}
