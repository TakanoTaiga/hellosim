use safe_drive::{
    context::Context, error::DynError , logger::Logger, pr_info
};
use std::time::Duration;

fn main() -> Result<(), DynError> {
    let ctx = Context::new()?;

    let node = ctx.create_node("hellosimnode", None, Default::default())?;

    let publisher_1 = node.create_publisher::<std_msgs::msg::Float32>("hellosim/motor_1", None)?;
    let publisher_2 = node.create_publisher::<std_msgs::msg::Float32>("hellosim/motor_2", None)?;
    let publisher_3 = node.create_publisher::<std_msgs::msg::Float32>("hellosim/motor_3", None)?;
    let publisher_4 = node.create_publisher::<std_msgs::msg::Float32>("hellosim/motor_4", None)?;

    let mut msg = std_msgs::msg::Float32::new().unwrap();
    let logger = Logger::new("hellosimnode");
    loop {
        msg.data = 0.0;
        publisher_1.send(&msg)?;
        publisher_2.send(&msg)?;

        msg.data = 0.0;
        publisher_3.send(&msg)?;
        publisher_4.send(&msg)?;
        pr_info!(logger, "send"); // Print log.
        std::thread::sleep(Duration::from_secs(1));
    }
}

