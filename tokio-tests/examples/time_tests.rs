use anyhow::Result;
use tokio::time;

async fn my_task(num: u8) -> Result<u16> {
    let x = num * 10;
    println!("task {} -> {}", num, x);

    time::sleep(time::Duration::from_millis(1)).await;

    Ok(x as u16)
}

/// follows cron attributes: see https://www.ibm.com/docs/en/db2oc?topic=task-unix-cron-format for definitions
#[derive(Debug, Default, Clone)]
pub struct RunAt {
    pub minutes: Vec<u8>, // 0..59
    pub hours: Vec<u8>, // 0..23
    pub days_of_month: Vec<u8>, // 0..31
    pub days_of_week: Vec<u8>, // 1..7
    pub months: Vec<u8>, // 1..12
    pub years: Vec<u16>,  // current..2900
}

impl RunAt {
    pub fn new() -> RunAt {
        RunAt {
            minutes: Vec::new(),
            hours: Vec::new(),
            days_of_month: Vec::new(),
            days_of_week: Vec::new(),
            months: Vec::new(),
            years: Vec::new(),
        }
    }

    pub fn with_minutes(minutes: Vec<u8>) -> RunAt {
        let mut at = RunAt::new();

        for minute in minutes {
            if at.valid_minute(minute) {
                at.minutes.push(minute);
            } else {
                eprintln!("range error: {}, ignored", minute);
            }
        }

        at
    }

    pub fn is_valid(&self) -> bool {
        true
    }

    pub fn valid_minute(&self, minute: u8) -> bool {
        self.validate_range(minute, 0u8, 59u8)
    }

    pub fn validate_range(&self, value: u8, min: u8, max: u8) -> bool {
        value >= min && value <= max
    }

}

#[tokio::main]
async fn main() {
    let mut interval = time::interval(time::Duration::from_millis(1000));

    let minutes: Vec<u8> = vec![5, 20, 35, 50];
    let runat = RunAt::with_minutes(minutes);

    println!("runat: {:?}", runat);

    for idx in 1..=5 {
        interval.tick().await;

        println!("request# {}", idx);

        let r = my_task(idx as u8).await;

        println!("response: {:?}", r);
    }
}