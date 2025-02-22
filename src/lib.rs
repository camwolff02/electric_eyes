pub mod easy_servo {
    use std::collections::HashMap;

    use esp_idf_svc::hal::gpio::OutputPin;
    use esp_idf_svc::hal::ledc::{config::TimerConfig, LedcDriver, LedcTimerDriver, Resolution};
    use esp_idf_svc::hal::ledc::{LedcChannel, LedcTimer};
    use esp_idf_svc::hal::peripheral::Peripheral;
    use esp_idf_svc::hal::prelude::*;

    use esp_idf_svc::sys::EspError;
    use map_to_range::MapRange;

    pub struct ServoController<'a, T: LedcTimer> {
        timer_driver: LedcTimerDriver<'a, T>,
        servos: HashMap<String, Servo<'a>>,
    }

    struct Servo<'a> {
        driver: LedcDriver<'a>,
        min_limit: u32,
        max_limit: u32,
    }

    impl<'a, T: LedcTimer> ServoController<'a, T> {
        pub fn new(timer: impl Peripheral<P = T> + 'a) -> Result<Self, EspError> {
            // Configure Pins that Will Read the Square Wave as Inputs
            LedcTimerDriver::new(
                timer,
                &TimerConfig::default()
                    .frequency(50.Hz())
                    .resolution(Resolution::Bits14),
            )
            .and_then(|timer_driver| {
                Ok(ServoController {
                    timer_driver,
                    servos: HashMap::new(),
                })
            })
        }

        pub fn add<C: LedcChannel<SpeedMode = T::SpeedMode>>(
            &mut self,
            name: &str,
            channel: impl Peripheral<P = C> + 'a,
            pin: impl Peripheral<P = impl OutputPin> + 'a,
        ) -> Result<(), EspError> {
            // Obtain a handle and configure the LEDC peripheral
            LedcDriver::new(
                channel,
                &self.timer_driver,
                pin, // bind to GPIO port 7
            )
            .and_then(|driver| {
                // Get Max Duty and Calculate Upper and Lower Limits for Servo
                let max_duty = driver.get_max_duty();
                let min_limit = max_duty * 25 / 1000;
                let max_limit = max_duty * 125 / 1000;

                self.servos.insert(
                    name.to_string(),
                    Servo {
                        driver,
                        min_limit,
                        max_limit,
                    },
                );

                Ok(())
            })
        }

        pub fn write(&mut self, name: &str, angle: u32) -> Option<()> {
            self.servos.get_mut(name).and_then(|servo| {
                angle
                    .map_range((0, 180), (servo.min_limit, servo.max_limit))
                    .and_then(|mapped_angle| servo.driver.set_duty(mapped_angle).ok())
            })
        }
    }
}
