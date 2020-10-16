
const KEY: &str = "asdf.zxcv";
use metrics_runtime::{
    Receiver,
};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let receiver = Receiver::builder().build().unwrap();
    let controller = receiver.controller();
    metrics::set_boxed_recorder(Box::new(receiver))?;
    generate_metrics();
    loop {
        let _snap = controller.snapshot();
    }
}

fn generate_metrics() {
    std::thread::spawn(|| {
        loop {
            metrics::timing!(KEY, std::time::Duration::from_millis(rand::random()));
            metrics::gauge!(KEY, rand::random());
            metrics::counter!(KEY, 1);
        }
    });
}
