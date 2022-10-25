#![no_std]
#![no_main]

use rtic::app;
use panic_halt as _;

// mod usb_manager;

#[app(device = feather_m0::hal::pac, peripherals = true, dispatchers = [EVSYS])]
mod app {

    // Board specific imports
    use feather_m0::hal as hal;
    use feather_m0::{
        pin_alias,
    };

    use hal::{
        clock::GenericClockController,
        // thumbv6m::usb::UsbBus,
    };

    use embedded_hal::digital::v2::{
        // StatefulOutputPin,
        OutputPin,
    };

    /**************************************************************************
    DATA STRUCTURE setup
    **************************************************************************/
    use usb_device::class_prelude::*;
    // use crate::usb_manager::UsbManager;
    #[shared]
    struct DataCommon {
        // usb_manager: UsbManager,
    }

    #[local]
    struct DataLocal {
        led: feather_m0::RedLed,
        state: bool,
    }

    use systick_monotonic::ExtU64;
    #[monotonic(binds = SysTick, default = true)]
    type MyMono = systick_monotonic::Systick<1000>;

    /**************************************************************************
    INIT ROUTINE
    **************************************************************************/
    #[init(local = [usb_bus: Option<usb_device::bus::UsbBusAllocator<hal::usb::UsbBus>> = None])]
    fn init(cx: init::Context) -> (DataCommon, DataLocal, init::Monotonics) {
        
        let mut device = cx.device;
        let mut clocks = GenericClockController::with_external_32kosc(
            device.GCLK,
            &mut device.PM,
            &mut device.SYSCTRL,
            &mut device.NVMCTRL,
        );

        /**********************************************************************
        Setup the GPIO and led pin 
        **********************************************************************/
        // initialize the pins to default state
        let pins = feather_m0::Pins::new(
            device.PORT,
        );
        let led_pin: feather_m0::RedLed = pin_alias!(pins.red_led).into();

        /**********************************************************************
        Setup the USB driver and USB Manager for serial port printing
        **********************************************************************/
        let usb_bus: &'static _ =
            cx.local.usb_bus.insert(UsbBusAllocator::new(hal::usb::UsbBus::new(
                clocks.usb_clock,
                &mut device.PM,
                pins.usb_dm,
                pins.usb_dp,
                device.USB,
            )));
        // let mut usb_manager = UsbManager::new(usb_bus);

        /**********************************************************************
        Setup tasks!
        **********************************************************************/
        // Blink 5 times on startup, and print a welcome message
        blink::spawn(5).unwrap();
        // print::spawn_after(1000.millis(), "Welcome to Carsten's RTIC App!\n").unwrap();
        // Start the heartbeat in 3 seconds
        heartbeat::spawn_after(3.secs()).unwrap();

        // Return the resource structs
        (
            DataCommon {
                // usb_manager: usb_manager,
            },
            DataLocal {
                led: led_pin,
                state: false,
            },
            init::Monotonics(systick_monotonic::Systick::new(cx.core.SYST, 64_000_000)),
        )
    }


    /**************************************************************************
    USB Interrupt task -- keeps the host happy and reads any available serial data
    **************************************************************************/
    // #[task(binds = USBCTRL_IRQ, shared = [usb_manager])]
    // fn usb_task(cx: usb_task::Context) {
    //     let mut usb_manager = cx.shared.usb_manager;
    //     (usb_manager).lock(
    //         |usb_manager_l| {
    //             usb_manager_l.interrupt();
    //         }
    //     );
    // }

    /**************************************************************************
    Print Task -- toggle the LED and prints the state to the serial port.
    **************************************************************************/
    // #[task(shared = [usb_manager])]
    // fn print(cx: print::Context, s: &'static str) {
    //     let mut usb_manager = cx.shared.usb_manager;
    //     usb_manager.lock(
    //         |usb_manager_l| {
    //             usb_manager_l.write(s);
    //             write!(usb_manager_l, "test {}", 1.2_f64);
    //         }
    //     );
    // }

    // #[task(shared = [usb_manager])]
    // fn print_fmt(cx: print_fmt::Context, args: (u8, f64)) {
    //     let mut usb_manager = cx.shared.usb_manager;
    //     usb_manager.lock(
    //         |usb_manager_l| {
    //             write!(usb_manager_l, "{}; {}\n", args.0, args.1);
    //         }
    //     );
    // }


    /**************************************************************************
    Heartbeat Task -- once started, the heartbeat will print to serial port
        every 2 seconds.
    **************************************************************************/
    #[task]
    fn heartbeat(_cx: heartbeat::Context) {
        blink::spawn(3).unwrap();
        // print::spawn("<3 heartbeat\n").unwrap();
        heartbeat::spawn_after(2500.millis()).unwrap();
    }

    /**************************************************************************
    LED Task -- Blinks the onboard LED n times
    **************************************************************************/
    const BLINK_DUR: u64 = 120;  // = on_time = off_time (in ms)
    #[task(local = [led, state])]
    fn blink(cx: blink::Context, n: u8) {
        if n == 0 {
            return;
        } else if *cx.local.state == false {
            cx.local.led.set_high().unwrap();
            *cx.local.state = true;
            blink::spawn_after(BLINK_DUR.millis(), n).unwrap();
        } else {
            cx.local.led.set_low().unwrap();
            *cx.local.state = false;
            blink::spawn_after(BLINK_DUR.millis(), n-1).unwrap();
        }
    }

} // mod app