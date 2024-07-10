use ola::{DmxBuffer, StreamingClient};
use std::{cell::RefCell, io::Write, rc::Rc};
use anyhow::Result;

#[derive(Debug)]
pub struct RGBW{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub w: u8,
}

impl From<[u8; 4]> for RGBW {
    fn from(value: [u8; 4]) -> Self {
        RGBW { r: value[0], g: value[1], b: value[2], w: value[3] }
    }
}

pub struct Dmx<S: Write> {
    pub buf: DmxBuffer,
    pub client: StreamingClient<S>,
    pub universe: u32,
}

impl <S: Write> Dmx<S> {
    pub fn send_dmx(&mut self) -> Result<()> {
        self.client.send_dmx(self.universe, &self.buf)?;
        Ok(())
    }
}

pub struct WBRGLight<S: Write> {
    pub dmx: Rc<RefCell<Dmx<S>>>,
    pub addr: usize,
}

impl <S: Write> WBRGLight<S> {
    pub fn set_rgbw(&mut self, colour: &RGBW) -> Result<()> {
        let mut dmx = self.dmx.try_borrow_mut()?;

        dmx.buf.set_channel(self.addr_red(), colour.r);
        dmx.buf.set_channel(self.addr_green(), colour.g);
        dmx.buf.set_channel(self.addr_blue(), colour.b);
        dmx.buf.set_channel(self.addr_white(), colour.w);

        dmx.send_dmx()?;
        Ok(())
    }

    pub fn rgbw(&mut self) -> Result<RGBW> {
        let mut dmx = self.dmx.try_borrow_mut()?;

        Ok(RGBW {
            r: dmx.buf.get_channel(self.addr_red()),
            g: dmx.buf.get_channel(self.addr_green()),
            b: dmx.buf.get_channel(self.addr_blue()),
            w: dmx.buf.get_channel(self.addr_white()),
        })
    }

    fn addr_red(&self) -> usize {
        self.addr + 2
    }

    fn addr_green(&self) -> usize {
        self.addr + 3
    }

    fn addr_blue(&self) -> usize {
        self.addr + 1
    }

    fn addr_white(&self) -> usize {
        self.addr
    }
}

pub struct MonoLight<S: Write> {
    pub dmx: Rc<RefCell<Dmx<S>>>,
    pub addr: usize,
}

impl <S: Write> MonoLight<S> {
    pub fn set_intensity(&mut self, intensity: u8) -> Result<()> {
        let mut dmx = self.dmx.try_borrow_mut()?;

        dmx.buf.set_channel(self.addr, intensity);

        dmx.send_dmx()?;
        Ok(())
    }
}

