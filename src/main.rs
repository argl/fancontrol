use anyhow::Result;
use rppal::pwm::{Channel, Polarity, Pwm};
use std::{fs, thread, time::Duration};

fn get_cpu_temp() -> Result<f32> {
    let temp_str = fs::read_to_string("/sys/class/thermal/thermal_zone0/temp")?;
    let temp: f32 = temp_str.trim().parse::<f32>()? / 1000.0;
    Ok(temp)
}

fn temp_to_duty_cycle(temp: f32) -> f64 {
    match temp {
        t if t < 50.0 => 0.0, // Fan off below 40°C
        t if t < 55.0 => 0.2, // 20% duty cycle
        t if t < 63.0 => 0.5, // 50% duty cycle
        t if t < 70.0 => 0.7, // 70% duty cycle
        _ => 1.0,             // Full speed over 70°C
    }
}

fn main() -> Result<()> {
    // PWM frequency for fan control should typically be 25kHz
    let pwm = Pwm::with_frequency(
        Channel::Pwm2, // GPIO18 uses PWM0
        25000.0,       // 25 kHz PWM frequency
        0.0,           // initial duty cycle (0%)
        Polarity::Normal,
        true, // enable immediately
    )?;

    loop {
        let temp = get_cpu_temp()?;
        let duty_cycle = temp_to_duty_cycle(temp);
        pwm.set_duty_cycle(duty_cycle)?;

        println!(
            "Temp: {:.1}°C, Duty Cycle: {:.0}%",
            temp,
            duty_cycle * 100.0
        );

        thread::sleep(Duration::from_secs(5));
    }
}
