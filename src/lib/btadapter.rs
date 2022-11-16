mod btadapter {

    use btleplug::api::{Central, Manager as _, ScanFilter};
    use btleplug::platform::{Manager, Peripheral};
    use std::error;

    pub async fn get_peripherals() -> Result<Vec<Peripheral>, Box<dyn error::Error>> {
        let manager = Manager::new().await?;
        let adapter_list = manager.adapters().await?;
        if adapter_list.is_empty() {
            return Err("List is empty".into());
        }
        for adapter in adapter_list.iter() {
            println!("Starting scan on {}", adapter.adapter_info().await?);
            adapter
                .start_scan(ScanFilter::default())
                .await
                .expect("Can't scan BLE adapter for devices");
            let peripherals = adapter.peripherals().await?;
            if peripherals.is_empty() {
                eprintln!("We couldn't find any peripherals nearby");
                return Err("peripherals is empty".into());
            } else {
                return Ok(peripherals);
            }
        }
        Ok(vec![])
    }

}

pub use btadapter::get_peripherals;
