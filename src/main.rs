mod lib;
use btleplug::api::Peripheral;
use std::error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    println!("Hello, world!");
    let peripherals = lib::btadapter::get_peripherals().await?;
    for periph in peripherals {
        let properties = periph.properties().await?;
        println!("{:?}", properties);
        let local_name = properties.unwrap().local_name.unwrap_or(String::from("Unknown Name"));
        println!("{:?}", local_name);
    }
    println!("Get the heart rate monitor to use");
    println!("Get the trainer to use");
    println!("Get the target heart rate");
    println!("Wait to start");
    println!("Run workout loop");
    println!("Wait for stop signal");
    Ok(())
}
