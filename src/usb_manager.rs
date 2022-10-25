use feather_m0::hal as hal;
use usb_device::{
    bus::UsbBusAllocator,
    device::{UsbDevice, UsbDeviceBuilder, UsbVidPid},
};
use usbd_serial::{SerialPort, USB_CLASS_CDC};

pub struct UsbManager {
    device: UsbDevice<'static, hal::usb::UsbBus>,
    serial: SerialPort<'static, hal::usb::UsbBus>,
}

impl UsbManager {
    pub fn new(usb_bus: &'static UsbBusAllocator<hal::usb::UsbBus>) -> Self {
    
        let serial = usbd_serial::SerialPort::new(usb_bus);
        let device = UsbDeviceBuilder::new(usb_bus, UsbVidPid(0x2E8A, 0x000a))
            .manufacturer("Shulltronics")
            .product("LoRa GPS Module")
            .serial_number("0.0")
            .device_class(USB_CLASS_CDC)
            .build();

        UsbManager { device, serial }
    }

    pub fn interrupt(&mut self) {
        if self.device.poll(&mut [&mut self.serial]) {
            let mut buf = [0u8; 64];
            match self.serial.read(&mut buf) {
                Err(_e) => {
                    // Do nothing
                }
                Ok(_count) => {
                    // Do nothing
                }
            }
        }
    }

    pub fn write(&mut self, s: &str) {
        match self.serial.write(s.as_bytes()) {
            Err(_e) => {
                // Do nothing
            }
            Ok(_) => {
                // Do nothing
            }
        };
    }

}

impl core::fmt::Write for UsbManager {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.serial.write(s.as_bytes()) {
            Err(_e) => {
                // Do nothing
            }
            Ok(_) => {
                // Do nothing
            }
        };
        Ok(())
    }
}