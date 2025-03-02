use std::io::Cursor;

use electric_eyes::easy_servo::ServoController;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::i2c::config::SlaveConfig;
use esp_idf_svc::hal::i2c::I2cSlaveDriver;
use esp_idf_svc::hal::ledc::LedcTimer;
use esp_idf_svc::hal::peripherals::Peripherals;
// use esp_idf_svc::hal::usb_serial::UsbSerialDriver;
use prost::Message;
use map_range_int::MapRange;

use eye_math::eyes::EyeState;
use eye_math::blink::Blink;
use eye_math::constants::*;

use eye_msgs::EyeRequest;

mod constants;
use constants::*;

pub mod eye_msgs {
    include!(concat!(env!("OUT_DIR"), "/eye_msgs.rs"));
}


fn main() {
    // It is necessary to call this function once. Otherwise some patches to the runtime
    // implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    log::info!("Hello, world!");

    /*****  SETUP CODE *****/
    // Obtain a handle for the device peripherals
    let io = Peripherals::take().unwrap();

    // Create I2C Slave Controller to handle inputs
    let mut input = I2cSlaveDriver::new(
        io.i2c0,
        io.pins.gpio21,
        io.pins.gpio22,
        I2C_ADDRESS,
        &SlaveConfig::default(),
    )
    .unwrap();

    // Create USB Serial driver for print debugging
    // let mut usb = UsbSerialDriver::new(
    //     io.uart0
    // )

    // Create servo controller and add 2 servos
    let mut controller = ServoController::new(io.ledc.timer0).unwrap();
    controller
        .add("look_left_right", io.ledc.channel0, io.pins.gpio14)
        .unwrap();
    controller
        .add("look_up_down", io.ledc.channel1, io.pins.gpio27)
        .unwrap();
    controller
        .add("eyelid_left_top", io.ledc.channel2, io.pins.gpio12)
        .unwrap();
    controller
        .add("eyelid_left_bottom", io.ledc.channel3, io.pins.gpio13)
        .unwrap();
    controller
        .add("eyelid_right_top", io.ledc.channel4, io.pins.gpio2)
        .unwrap();
    controller
        .add("eyelid_right_bottom", io.ledc.channel5, io.pins.gpio15)
        .unwrap();

    let mut buf  = [0_u8; 10];

    // Define eyes
    let mut eyes = EyeState::new();
    let mut blink = Blink::new();

    loop {
        if let Ok(_usize) = input.read(&mut buf, 10) {
            let buff = Cursor::new(buf);

            if let Ok(eye_request) = eye_msgs::EyeRequest::decode(buff.get_ref().as_slice()) { 
                if TUNE_MODE {
                    tune_servos(&mut controller, eye_request);
                } else {
                    control_servos(&mut controller, eye_request, &mut eyes, &mut blink);
                }               
            }
        };
    }
}

fn tune_servos<T: LedcTimer>(controller: &mut ServoController<'_, T>, eye_request: EyeRequest) {
    let names = ["look_left_right", "look_up_down", "eyelid_left_top", "eyelid_left_bottom", "eyelid_right_top", "eyelid_right_bottom"];
    let mut current_servo = 0;

    loop {
        if eye_request.blink {
            current_servo += 1;
            if current_servo >= names.len() {
                current_servo = 0;
            }
        } 
        let angle = eye_request.vert_angle.map_range((-MAX_VERT_ANGLE, MAX_VERT_ANGLE), (LOOK_UP_DOWN_MIN, LOOK_UP_DOWN_MAX)).unwrap_or(0);
        controller.write(names[current_servo], angle);
    }
}

fn control_servos<T: LedcTimer>(controller: &mut ServoController<'_, T>, eye_request: EyeRequest, eyes: &mut EyeState, blink: &mut Blink) {
    /*****  LOOP CODE *****/
    loop {
        eyes.look(eye_request.left_horiz_angle, eye_request.right_horiz_angle, eye_request.vert_angle);
        eyes.move_eyelids(eye_request.left_eyelid_gap, eye_request.right_eyelid_gap);
        if eye_request.blink {
            blink.start_blink();
        }

        // tick automatic eye transformations
        let new_eyes = eyes
            .transform( blink);
            // apply additional transforms here

        controller.write("look_up_down", new_eyes.get_vert_angle() as u32);
        controller.write("look_left_right", new_eyes.get_left_horiz_angle() as u32);
        controller.write("eyelid_left_top", new_eyes.get_left_eyelid_gap());

        FreeRtos::delay_ms(12);
    }
}
