use sysinfo::{
    Networks, System,
};

use chrono;

fn main() {
    let mut sys = System::new_all();

    sys.refresh_all();

    println!("System name:             {:?}", System::name());
    println!("System OS version:       {:?}", System::os_version());
    println!("System host name:        {:?}", System::host_name());

    println!("NB CPUs: {}", sys.cpus().len());

    let networks = Networks::new_with_refreshed_list();
    println!("=> networks:");
    for (interface_name, data) in &networks {
        println!(
            "{interface_name}: {} B (down) / {} B (up)",
            data.total_received(),
            data.total_transmitted(),
        );
    }


    let timestamp = chrono::offset::Local::now();

    let mut number = 3;

    while number != 0 {
        number -= 1;
    }

    let currentTimeStamp = chrono::offset::Local::now();

    println!("{}", currentTimeStamp - timestamp);
}
