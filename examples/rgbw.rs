use dmx512::{Dmx, WBRGLight};
use ola::DmxBuffer;
use std::{cell::RefCell, rc::Rc, thread::sleep, time::Duration};
use anyhow::Result;
use rand::{thread_rng, Rng};

fn main() -> Result<()>{
    let mut rng = thread_rng();
    let dmx = Rc::new(RefCell::new(Dmx {
        buf: DmxBuffer::new(),
        client: ola::connect()?,
        universe: 0,
    }));
    println!("Connected to OLA");

    let mut lights1_3 = WBRGLight {
        dmx: Rc::clone(&dmx),
        addr: 0
    };
    let mut lights4_6 = WBRGLight {
        dmx: Rc::clone(&dmx),
        addr: 4
    };

    loop {
        lights1_3.set_rgbw(&rng.gen::<[u8; 4]>().into())?;
        println!("Set lights for columns 1-3 to {:?}", lights1_3.rgbw());
        sleep(Duration::from_secs(1));

        lights4_6.set_rgbw(&rng.gen::<[u8; 4]>().into())?;
        println!("Set lights for columns 4-6 to {:?}", lights4_6.rgbw());
        sleep(Duration::from_secs(1));
    }
}
