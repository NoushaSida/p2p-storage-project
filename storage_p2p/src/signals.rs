use std::thread;
use signal_hook::iterator::Signals;
use std::io::Error;
use log::info;

use crate::Storage;

pub fn signals_handling(signal_to_handle: i32, s: &Storage) -> Result<(), Error> {
    let mut signal = Signals::new(&[signal_to_handle])?;
    /*thread::spawn(move || {
        for sig in signal.forever() {
            info!("Received signal {:?}", sig);
            s.close();
        }
    });*/

    Ok(())
}
