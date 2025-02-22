use electric_eyes::easy_servo::ServoController;
use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::i2c::config::SlaveConfig;
use esp_idf_svc::hal::i2c::I2cSlaveDriver;
use esp_idf_svc::hal::peripherals::Peripherals;
use esp_idf_svc::sys::TickType_t;

const I2C_ADDRESS: u8 = 0x43;

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
    let input = I2cSlaveDriver::new(
        io.i2c0,
        io.pins.gpio0,
        io.pins.gpio2,
        I2C_ADDRESS,
        &SlaveConfig::default(),
    )
    .unwrap();

    // Create servo controller and add 2 servos
    let mut controller = ServoController::new(io.ledc.timer0).unwrap();
    controller
        .add("LR", io.ledc.channel0, io.pins.gpio15)
        .unwrap();
    controller
        .add("UD", io.ledc.channel1, io.pins.gpio14)
        .unwrap();

    // Motor position initialization
    // Define Starting Position
    controller.write("LR", 90);
    controller.write("UD", 90);

    // Give servo some time to update
    FreeRtos::delay_ms(500);

    /*****  LOOP CODE *****/
    loop {
        // Sweep from 0 degrees to 180 degrees
        for angle in 90..181 {
            // Print Current Angle for visual verification
            println!("Current Angle {} Degrees", angle);
            // Set the desired duty cycle
            controller.write("LR", angle);
            controller.write("UD", angle);

            // Give servo some time to update
            FreeRtos::delay_ms(12);
        }

        // Sweep from 180 degrees to 90 degrees
        for angle in (90..181).rev() {
            // Print Current Angle for visual verification
            println!("Current Angle {} Degrees", angle);
            // Set the desired duty cycle
            controller.write("LR", angle);
            controller.write("UD", angle);

            // Give servo some time to update
            FreeRtos::delay_ms(12);
        }
    }

}
