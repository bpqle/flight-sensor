use std::thread;
use std::time:Duration;
use vl54l4cd::Vl53l4cd;

#[tokio::main]
async fn main() {
     let sensor = Vl53l4cd::new(bus)
}
