#[cfg(not(target_os = "macos"))]
compile_error!("works only on macOS");

use macsmc::{Celsius, Error as SmcError, Smc, Watt};
use std::{
    cmp::Ordering,
    env,
    error::Error as StdError,
    fmt::{self, Display},
    time::Duration,
};

type Result<T> = std::result::Result<T, Error>;
#[derive(Debug)]
enum Error {
    Smc(SmcError),
    UnknownStatsSelector(String),
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        if let Error::Smc(smc) = self {
            Some(smc)
        } else {
            None
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Smc(e) => write!(f, "{}", e),
            Error::UnknownStatsSelector(cmd) => write!(f, "The command `{}` is not known", cmd),
        }
    }
}

impl From<SmcError> for Error {
    fn from(e: SmcError) -> Self {
        Error::Smc(e)
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(u8)]
enum Printables {
    Cpu = 1,
    Gpu = 2,
    Other = 4,
    Fan = 8,
    Battery = 16,
    Power = 32,
    Debug = 64,
}

fn run() -> Result<()> {
    use Printables::*;

    let mut args = env::args();
    let _ = args.next().expect("missing program name");

    let mut commands = 0;
    for item in args {
        match &item[..] {
            "temp" | "temps" => commands |= Cpu as u8 | Gpu as u8 | Other as u8,
            "cpu" | "CPU" | "hot" => commands |= Cpu as u8,
            "gpu" | "GPU" => commands |= Gpu as u8,
            "other" | "others" => commands |= Other as u8,
            "fan" | "fans" | "speed" | "fast" => commands |= Fan as u8,
            "battery" | "batt" | "ac" => commands |= Battery as u8,
            "power" => commands |= Power as u8,
            "debug" => commands |= Debug as u8,
            "all" | "EVERYTHING" => {
                commands |=
                    Cpu as u8 | Gpu as u8 | Other as u8 | Fan as u8 | Battery as u8 | Power as u8
            }
            _ => return Err(Error::UnknownStatsSelector(item)),
        }
    }

    if commands == 0 {
        commands = Cpu as u8 | Fan as u8 | Battery as u8 | Power as u8
    }

    let mut smc = Smc::connect()?;
    if commands & Debug as u8 != 0 {
        print_all_keys(&mut smc)?;
        return Ok(());
    }

    let mut printed_something = false;
    for &item in [Cpu, Gpu, Other, Fan, Battery, Power].iter() {
        if commands & item as u8 != 0 {
            if printed_something {
                println!();
                println!();
            }
            match item {
                Cpu => print_cpu_temps(&mut smc)?,
                Gpu => print_gpu_temps(&mut smc)?,
                Other => print_other_temps(&mut smc)?,
                Fan => print_fan_speeds(&mut smc)?,
                Battery => print_battery_info(&mut smc)?,
                Power => print_power_consumption(&mut smc)?,
                Debug => {}
            }
            printed_something = true;
        }
    }

    Ok(())
}

fn print_cpu_temps(smc: &mut Smc) -> Result<()> {
    println!("--- CPU Temperatures [cpu] ---");
    println!();
    let cpu_temp = smc.cpu_temperature()?;
    print_temp("CPU Proximity", cpu_temp.proximity);
    print_temp("CPU Die", cpu_temp.die);
    print_temp("CPU Graphics", cpu_temp.graphics);
    print_temp("CPU System Agent", cpu_temp.system_agent);
    println!();

    for (core_num, core_temp) in smc.cpu_core_temps()?.enumerate() {
        print_temp(format!("CPU Core {}", core_num + 1), core_temp?);
    }

    Ok(())
}

fn print_gpu_temps(smc: &mut Smc) -> Result<()> {
    println!("--- GPU Temperatures [gpu] ---");
    println!();
    let gpu_temp = smc.gpu_temperature()?;
    print_temp("GPU Proximity", gpu_temp.proximity);
    print_temp("GPU Die", gpu_temp.die);

    Ok(())
}

fn print_other_temps(smc: &mut Smc) -> Result<()> {
    println!("--- Other Temperatures [other] ---");
    println!();
    let other_temp = smc.other_temperatures()?;
    print_temp("Mainboard Proximity", other_temp.mainboard_proximity);
    print_temp("Platform CHD", other_temp.platform_controller_hub_die);
    print_temp("Airport", other_temp.airport);
    print_temp("Airflow Left", other_temp.airflow_left);
    print_temp("Airflow Right", other_temp.airflow_right);
    print_temp("Thunderbolt Left", other_temp.thunderbolt_left);
    print_temp("Thunderbolt Right", other_temp.thunderbolt_right);
    print_temp("Heatpipe 1", other_temp.heatpipe_1);
    print_temp("Heatpipe 2", other_temp.heatpipe_2);
    print_temp("Palm rest 1", other_temp.palm_rest_1);
    print_temp("Palm rest 2", other_temp.palm_rest_2);

    Ok(())
}

fn print_fan_speeds(smc: &mut Smc) -> Result<()> {
    println!("--- Fan Speeds [fan] ---");
    println!();
    for (fan_num, fan_speed) in smc.fans()?.enumerate() {
        let fan_speed = fan_speed?;
        print_value(
            format!("Fan {} speed", fan_num + 1),
            fan_speed.actual,
            "RPM",
            fan_speed.thresholds(),
        );
    }

    Ok(())
}

struct Time(Duration);

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let secs = self.0.as_secs();
        let hours = secs / 3600;
        let mins = (secs % 3600) / 60;
        let secs = secs % 60;
        if hours > 0 {
            write!(f, "{}h ", hours)?;
        }
        write!(f, "{:02}m {:02}s", mins, secs)
    }
}

fn print_battery_info(smc: &mut Smc) -> Result<()> {
    println!("--- Battery Info [battery] ---");
    println!();
    let battery_info = smc.battery_info()?;
    let running_on = match (
        battery_info.battery_powered,
        battery_info.ac_present,
        battery_info.charging,
    ) {
        (_, true, true) => "AC (Charging Battery)",
        (_, true, false) => "AC",
        (true, ..) => "Battery",
        _ => "Magic Dust",
    };
    print_line(
        "Battery healthy",
        if battery_info.health_ok { "OK" } else { "💥" },
    );
    print_line("Running on", running_on);
    for battery in smc.battery_details()? {
        let battery = battery?;
        if !battery_info.ac_present {
            if let Some(remaining) = battery.time_remaining() {
                print_line("Time remainging", Time(remaining));
            }
        }
        if battery_info.charging {
            if let Some(until_full) = battery.time_until_full() {
                print_line("Time until full", Time(until_full));
            }
        }
        print_line("Cycle count", battery.cycles);
        print_percentage("Charge", battery.percentage());
        print_value_unit("Current Capacity", *battery.current_capacity, "mAh");
        print_value_unit("Full Capacity", *battery.full_capacity, "mAh");
        print_value_unit("Amperage", *battery.amperage, "mA");
        print_value_unit("Voltage", *battery.voltage, "V");
        if *battery.power > 0.0 {
            print_value_unit("Power Delivery", *battery.power, "W");
        }
        if *battery.power < 0.0 {
            print_value_unit("Charging rate", -*battery.power, "W");
        }
    }
    print_temp("Battery Sensor 1", battery_info.temperature_1);
    print_temp("Battery Sensor 2", battery_info.temperature_2);

    Ok(())
}

fn print_power_consumption(smc: &mut Smc) -> Result<()> {
    println!("--- Power consumption [power] ---");
    println!();
    let cpu_power = smc.cpu_power()?;
    print_power("CPU Core", cpu_power.core);
    print_power("CPU DRAM", cpu_power.dram);
    print_power("CPU Graphics", cpu_power.gfx);
    print_power("CPU Total", cpu_power.total);
    print_power("CPU Rail", cpu_power.rail);
    let gpu_power = smc.gpu_power()?;
    print_power("GPU", gpu_power);
    let dc_in = smc.power_dc_in()?;
    print_power("DC Input", dc_in);
    let system_total = smc.power_system_total()?;
    print_power("System Total", system_total);

    Ok(())
}

fn print_all_keys(smc: &mut Smc) -> Result<()> {
    let number_of_keys = smc.number_of_keys()?;
    for i in 0..number_of_keys {
        let info = smc.key_info_by_index(i)?;
        if let Ok(data) = smc.key_data_by_index(i) {
            println!("{} == {:?}", info, data.1);
        } else {
            println!("{} xxx", info);
        }
    }

    Ok(())
}

fn print_temp(label: impl AsRef<str>, temp: Celsius) {
    print_value(label, temp, "°C", Celsius::thresholds())
}

fn print_power(label: impl AsRef<str>, power: Watt) {
    print_value(label, power, "W", Watt::thresholds())
}

fn print_line(label: impl AsRef<str>, val: impl Display) {
    println!("{:>24}  {}", label.as_ref(), val);
}

fn print_value_unit(label: impl AsRef<str>, val: impl Display, unit: impl AsRef<str>) {
    println!("{:>24}  {:8.2} {:6}", label.as_ref(), val, unit.as_ref(),);
}

fn print_percentage(label: impl AsRef<str>, val: impl Into<f64> + PartialOrd) {
    print_value(label, val.into(), "%", [99.0, 75.0, 30.0, 10.0])
}

fn print_value<T>(label: impl AsRef<str>, val: T, unit: impl AsRef<str>, thresholds: [T; 4])
where
    T: Into<f64> + PartialOrd + Copy,
{
    println!(
        "{:>24}  {:8.2} {:6}{}",
        label.as_ref(),
        val.into(),
        unit.as_ref(),
        sparkles(val, thresholds)
    );
}

fn sparkles<T>(val: T, thresholds: [T; 4]) -> String
where
    T: Into<f64> + PartialOrd + Copy,
{
    let min = thresholds[0].into();
    let max = thresholds[3].into();

    if max > min {
        sparkline(val, thresholds, min, max, Ordering::Greater)
    } else {
        sparkline(val, thresholds, max, min, Ordering::Less)
    }
}

fn sparkline<T>(val: T, thresholds: [T; 4], min: f64, max: f64, target_ord: Ordering) -> String
where
    T: Into<f64> + PartialOrd + Copy,
{
    debug_assert!(max > min);

    static BLOCKS: [char; 8] = ['▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];

    let mut scale = (max - min) / 7.0;
    if scale < 1.0 {
        scale = 1.0;
    }

    let idx = ((val.into() - min) / scale).ceil();
    let idx = idx.max(0.0).min(8.0) as usize;

    let mut out = String::with_capacity(41);

    out.push_str("\x1B[38;5;");

    if T::partial_cmp(&val, &thresholds[3]) == Some(target_ord) {
        // red
        out.push('1');
    } else if T::partial_cmp(&val, &thresholds[2]) == Some(target_ord) {
        // light_red
        out.push('9');
    } else if T::partial_cmp(&val, &thresholds[1]) == Some(target_ord) {
        // yellow
        out.push('3');
    } else {
        // green
        out.push('2');
    }
    out.push('m');

    BLOCKS[..idx].iter().for_each(|&c| out.push(c));

    out.push_str("\x1B[2m"); // dim
    BLOCKS[idx..].iter().for_each(|&c| out.push(c));

    out.push_str("\x1B[0m"); // reset

    out
}
