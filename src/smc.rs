use crate::{Error as PubError, Result as PubResult};
use std::{
    array::TryFromSliceError,
    convert::{TryFrom, TryInto},
    ffi::CStr,
    num::TryFromIntError,
    ops::Deref,
    time::Duration,
};

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Celsius(pub f32);

impl Deref for Celsius {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<f64> for Celsius {
    fn into(self) -> f64 {
        f64::from(self.0)
    }
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
pub struct Fahrenheit(pub f32);

impl Deref for Fahrenheit {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(v: Celsius) -> Self {
        Self((v.0 * (9.0 / 5.0)) + 32.0)
    }
}

impl Celsius {
    const THRESHOLDS: [Self; 4] = [Self(50.0), Self(68.0), Self(80.0), Self(90.0)];

    pub fn thresholds() -> [Self; 4] {
        Self::THRESHOLDS
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct CpuTemperatures {
    pub proximity: Celsius,
    pub die: Celsius,
    pub graphics: Celsius,
    pub system_agent: Celsius,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct GpuTemperatures {
    pub proximity: Celsius,
    pub die: Celsius,
}

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct OtherTemperatures {
    pub memory_bank_proximity: Celsius,
    pub mainboard_proximity: Celsius,
    pub platform_controller_hub_die: Celsius,
    pub airport: Celsius,
    pub airflow_left: Celsius,
    pub airflow_right: Celsius,
    pub thunderbolt_left: Celsius,
    pub thunderbolt_right: Celsius,
    pub heatpipe_1: Celsius,
    pub heatpipe_2: Celsius,
    pub palm_rest_1: Celsius,
    pub palm_rest_2: Celsius,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Rpm(pub f32);

impl Deref for Rpm {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<f64> for Rpm {
    fn into(self) -> f64 {
        f64::from(self.0)
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct FanSpeed {
    pub actual: Rpm,
    pub min: Rpm,
    pub max: Rpm,
    pub target: Rpm,
    pub safe: Rpm,
    pub mode: FanMode,
}

impl FanSpeed {
    pub fn percentage(&self) -> f32 {
        let rpm = (*self.actual - *self.min).max(0.0);
        let pct = rpm / (*self.max - *self.min);
        100.0 * pct
    }

    pub fn thresholds(&self) -> [Rpm; 4] {
        let span = (*self.max - *self.min) / 3.0;
        [
            self.min,
            Rpm(*self.min + span),
            Rpm(*self.min + (2.0 * span)),
            self.max,
        ]
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum FanMode {
    Forced,
    Auto,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BatteryInfo {
    pub battery_powered: bool,
    pub charging: bool,
    pub ac_present: bool,
    pub health_ok: bool,
    pub temperature_max: Celsius,
    pub temperature_1: Celsius,
    pub temperature_2: Celsius,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BatteryDetail {
    pub cycles: u32,
    pub current_capacity: MilliAmpereHours,
    pub full_capacity: MilliAmpereHours,
    pub amperage: MilliAmpere,
    pub voltage: Volt,
    pub power: Watt,
}

impl BatteryDetail {
    pub fn percentage(&self) -> f32 {
        (100.0 * (f64::from(*self.current_capacity) / f64::from(*self.full_capacity))) as f32
    }

    pub fn time_remaining(&self) -> Option<Duration> {
        if *self.amperage >= 0 {
            None
        } else {
            let hours = f64::from(*self.current_capacity) / f64::from(-*self.amperage);
            Some(Duration::from_secs_f64(3600.0 * hours))
        }
    }

    pub fn time_until_full(&self) -> Option<Duration> {
        if *self.amperage <= 0 {
            None
        } else {
            let hours =
                f64::from(*self.full_capacity - *self.current_capacity) / f64::from(*self.amperage);
            Some(Duration::from_secs_f64(3600.0 * hours))
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CpuPower {
    pub core: Watt,
    pub dram: Watt,
    pub gfx: Watt,
    pub rail: Watt,
    pub total: Watt,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct MilliAmpereHours(pub u32);

impl Deref for MilliAmpereHours {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct MilliAmpere(pub i32);

impl Deref for MilliAmpere {
    type Target = i32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Volt(pub f32);

impl Deref for Volt {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Watt(pub f32);

impl Deref for Watt {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Into<f64> for Watt {
    fn into(self) -> f64 {
        f64::from(self.0)
    }
}

impl Watt {
    const THRESHOLDS: [Self; 4] = [Self(35.0), Self(50.0), Self(70.0), Self(85.0)];

    pub fn thresholds() -> [Self; 4] {
        Self::THRESHOLDS
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DataValue {
    Flag(bool),
    Float(f32),
    Int(i64),
    Uint(u64),
    Str(String),
    Unknown(Vec<u8>),
}

#[derive(Debug)]
pub struct Dbg(String, pub DataValue);

type Result<T> = std::result::Result<T, InternalError>;
enum InternalError {
    SmcNotFound,
    SmcFailedToOpen(i32),
    NotPrivlileged,
    UnknownSmc(i32, u8),
    _UnknownKey,
    _DataKeyError(u32),
    _DataValueError,
    // for pub error
    DataError { key: u32, tpe: u32 },
}

impl From<TryFromSliceError> for InternalError {
    fn from(_: TryFromSliceError) -> Self {
        Self::_DataValueError
    }
}

impl From<TryFromIntError> for InternalError {
    fn from(_: TryFromIntError) -> Self {
        Self::_DataValueError
    }
}

impl From<InternalError> for PubError {
    fn from(ie: InternalError) -> Self {
        match ie {
            InternalError::SmcNotFound => PubError::SmcNotAvailable,
            InternalError::SmcFailedToOpen(_) => PubError::SmcNotAvailable,
            InternalError::NotPrivlileged => PubError::InsufficientPrivileges,
            InternalError::UnknownSmc(code, _) => PubError::SmcError(code),
            InternalError::DataError { key, tpe } => PubError::DataError { key, tpe },
            InternalError::_UnknownKey => unreachable!(),
            InternalError::_DataValueError => unreachable!(),
            InternalError::_DataKeyError(_) => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Smc {
    inner: cffi::SMCConnection,
}

impl Smc {
    pub fn connect() -> PubResult<Self> {
        let inner = cffi::SMCConnection::new()?;
        Ok(Smc { inner })
    }

    pub fn fans(&mut self) -> PubResult<FanIter> {
        FanIter::new(self)
    }

    fn number_of_fans(&mut self) -> PubResult<u8> {
        Ok(self.inner.read_value(GetNumberOfFans)?)
    }

    fn fan_speed(&mut self, fan: u8) -> PubResult<FanSpeed> {
        let actual = self.inner.read_value(GetActualFanSpeed(fan))?;
        let min = self.inner.read_value(GetMinFanSpeed(fan))?;
        let max = self.inner.read_value(GetMaxFanSpeed(fan))?;
        let target = self.inner.read_value(GetTargetFanSpeed(fan))?;
        let safe = self.inner.read_value(GetSafeFanSpeed(fan))?;
        let mode = self.inner.read_value(GetFanMode(fan))?;
        Ok(FanSpeed {
            actual,
            min,
            max,
            target,
            safe,
            mode,
        })
    }

    pub fn battery_info(&mut self) -> PubResult<BatteryInfo> {
        let BatteryStatus {
            charging,
            ac_present,
            health_ok,
        } = self.inner.read_value(GetBatteryInfo)?;
        let battery_powered = self.inner.read_value(IsBatteryPowered)?;
        let temperature_max = self.inner.read_value(GetBatteryTemperatureMax)?;
        let temperature_1 = self.inner.read_value(GetBatteryTemperature1)?;
        let temperature_2 = self.inner.read_value(GetBatteryTemperature2)?;
        Ok(BatteryInfo {
            battery_powered,
            charging,
            ac_present,
            health_ok,
            temperature_max,
            temperature_1,
            temperature_2,
        })
    }

    fn number_of_batteries(&mut self) -> PubResult<u8> {
        Ok(self.inner.read_value(GetNumberOfBatteries)?)
    }

    pub fn battery_details(&mut self) -> PubResult<BatteryIter> {
        Ok(BatteryIter::new(self)?)
    }

    fn battery_detail(&mut self, battery: u8) -> PubResult<BatteryDetail> {
        let cycles = self.inner.read_value(GetBatteryCycleCount(battery))?;
        let current_capacity = self.inner.read_value(GetBatteryCurrentCapacity(battery))?;
        let full_capacity = self.inner.read_value(GetBatteryFullCapacity(battery))?;
        let amperage = self.inner.read_value(GetBatteryAmperage(battery))?;
        let voltage = self.inner.read_value(GetBatteryVoltage(battery))?;
        let power = self.inner.read_value(GetBatteryPower(battery))?;
        Ok(BatteryDetail {
            cycles,
            current_capacity,
            full_capacity,
            amperage,
            voltage,
            power,
        })
    }

    fn number_of_cpus(&mut self) -> PubResult<u8> {
        Ok(cffi::num_cpus().min(255) as u8)
    }

    pub fn cpu_temperature(&mut self) -> PubResult<CpuTemperatures> {
        let proximity = self.inner.read_value(CpuProximityTemperature)?;
        let die = self.inner.read_value(CpuDieTemperature)?;
        let graphics = self.inner.read_value(CpuGfxTemperature)?;
        let system_agent = self.inner.read_value(CpuSystemAgentTemperature)?;
        Ok(CpuTemperatures {
            proximity,
            die,
            graphics,
            system_agent,
        })
    }

    pub fn cpu_core_temps(&mut self) -> PubResult<CpuIter> {
        Ok(CpuIter::new(self)?)
    }

    fn cpu_core_temperature(&mut self, core: u8) -> PubResult<Celsius> {
        Ok(self.inner.read_value(CpuCoreTemperature(core + 1))?)
    }

    pub fn gpu_temperature(&mut self) -> PubResult<GpuTemperatures> {
        let proximity = self.inner.read_value(GpuProximityTemperature)?;
        let die = self.inner.read_value(GpuDieTemperature)?;
        Ok(GpuTemperatures { proximity, die })
    }

    pub fn other_temperatures(&mut self) -> PubResult<OtherTemperatures> {
        let memory_bank_proximity = self.inner.read_value(GetMemoryBankProximityTemperature)?;
        let mainboard_proximity = self.inner.read_value(GetMainboardProximityTemperature)?;
        let platform_controller_hub_die = self.inner.read_value(GetPCHDieTemperature)?;
        let airport = self.inner.read_value(GetAirportTemperature)?;
        let airflow_left = self.inner.read_value(GetAirflowLeftTemperature)?;
        let airflow_right = self.inner.read_value(GetAirflowRightTemperature)?;
        let thunderbolt_left = self.inner.read_value(GetThunderboltLeftTemperature)?;
        let thunderbolt_right = self.inner.read_value(GetThunderboltRightTemperature)?;
        let heatpipe_1 = self.inner.read_value(GetHeatpipe1Temperature)?;
        let heatpipe_2 = self.inner.read_value(GetHeatpipe2Temperature)?;
        let palm_rest_1 = self.inner.read_value(GetPalmRest1Temperature)?;
        let palm_rest_2 = self.inner.read_value(GetPalmRest2Temperature)?;
        Ok(OtherTemperatures {
            memory_bank_proximity,
            mainboard_proximity,
            platform_controller_hub_die,
            airport,
            airflow_left,
            airflow_right,
            thunderbolt_left,
            thunderbolt_right,
            heatpipe_1,
            heatpipe_2,
            palm_rest_1,
            palm_rest_2,
        })
    }

    pub fn cpu_power(&mut self) -> PubResult<CpuPower> {
        let core = self.inner.read_value(CpuCorePower)?;
        let dram = self.inner.read_value(CpuDramPower)?;
        let gfx = self.inner.read_value(CpuGfxPower)?;
        let rail = self.inner.read_value(CpuRailPower)?;
        let total = self.inner.read_value(CpuTotalPower)?;
        Ok(CpuPower {
            core,
            dram,
            gfx,
            rail,
            total,
        })
    }

    pub fn gpu_power(&mut self) -> PubResult<Watt> {
        Ok(self.inner.read_value(GpuRailPower)?)
    }

    pub fn power_dc_in(&mut self) -> PubResult<Watt> {
        Ok(self.inner.read_value(DcInPower)?)
    }

    pub fn power_system_total(&mut self) -> PubResult<Watt> {
        Ok(self.inner.read_value(SystemTotalPower)?)
    }

    fn check(&mut self, name: &str) -> PubResult<Option<Dbg>> {
        let value = self.inner.opt_read_value(Check(name))?;
        Ok(value.map(|v| Dbg(name.to_string(), v)))
    }

    pub fn number_of_keys(&mut self) -> PubResult<u32> {
        Ok(self.inner.read_value(NumberOfKeys)?)
    }

    fn key_info(&mut self, name: &str) -> PubResult<String> {
        let info = self.inner.key_info(Check(name))?;
        let key = info.key.to_be_bytes();
        let tpe = info.data_type.to_be_bytes();

        Ok(format!(
            "key = {}, type = {:?}, size = {}",
            String::from_utf8_lossy(&key),
            String::from_utf8_lossy(&tpe),
            info.data_size
        ))
    }

    pub fn key_info_by_index(&mut self, index: u32) -> PubResult<String> {
        let info = self.inner.key_info_by_index(index)?;
        let key = info.key.to_be_bytes();
        let key = std::str::from_utf8(&key).map_err(|_| InternalError::DataError {
            key: info.key,
            tpe: info.data_type,
        })?;
        self.key_info(key)
    }

    pub fn key_data_by_index(&mut self, index: u32) -> PubResult<Dbg> {
        let info = self.inner.key_info_by_index(index)?;
        let key = info.key.to_be_bytes();
        let key = std::str::from_utf8(&key).map_err(|_| InternalError::DataError {
            key: info.key,
            tpe: info.data_type,
        })?;
        // key_info already fails of key does not exist
        self.check(key).map(|d| d.unwrap())
    }
}

macro_rules! iter_impl {
    ($struct:ident = $max:ident : $get:ident -> $out:tt) => {
        pub struct $struct<'a> {
            smc: &'a mut $crate::smc::Smc,
            next: u8,
            max: u8,
        }

        impl<'a> $struct<'a> {
            fn new(smc: &'a mut $crate::smc::Smc) -> $crate::Result<Self> {
                let max = smc.$max()?;
                Ok(Self { smc, next: 0, max })
            }
        }

        impl<'a> Iterator for $struct<'a> {
            type Item = $crate::Result<$out>;

            fn next(&mut self) -> Option<Self::Item> {
                if self.next >= self.max {
                    return None;
                }
                let value = match self.smc.$get(self.next) {
                    Ok(value) => value,
                    Err(e) => return Some(Err(e)),
                };
                self.next += 1;
                Some(Ok(value))
            }
        }
    };
}

iter_impl!(FanIter = number_of_fans: fan_speed -> FanSpeed);
iter_impl!(BatteryIter = number_of_batteries: battery_detail -> BatteryDetail);
iter_impl!(CpuIter = number_of_cpus: cpu_core_temperature -> Celsius);

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct CommandKey(u32);

impl Deref for CommandKey {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CommandKey {
    fn set1(self, value: u8) -> Self {
        let value = b'0' + value;
        let mut bytes = self.0.to_be_bytes();
        bytes[1] = value;
        CommandKey(u32::from_be_bytes(bytes))
    }

    fn set2(self, value: u8) -> Self {
        let value = b'0' + value;
        let mut bytes = self.0.to_be_bytes();
        bytes[2] = value;
        CommandKey(u32::from_be_bytes(bytes))
    }
}

trait ReadAction {
    type Out: ValueParser;

    fn key(&self) -> CommandKey;

    fn parse(self, val: DataValue) -> Result<Self::Out>
    where
        Self: Sized,
    {
        <Self::Out as ValueParser>::parse(val)
    }
}

trait ValueParser: Sized {
    fn parse(val: DataValue) -> Result<Self>;
}

impl ValueParser for Celsius {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Float(value) => Ok(Self(value)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for Rpm {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Float(value) => Ok(Self(value)),
            DataValue::Uint(v) => Ok(Self(f32::from(u16::try_from(v)?))),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl From<bool> for FanMode {
    fn from(v: bool) -> Self {
        if v {
            FanMode::Forced
        } else {
            FanMode::Auto
        }
    }
}

impl Default for FanMode {
    fn default() -> Self {
        FanMode::Auto
    }
}

impl ValueParser for FanMode {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Flag(bool) => Ok(bool.into()),
            DataValue::Int(value) => Ok((value != 0).into()),
            DataValue::Uint(value) => Ok((value != 0).into()),
            DataValue::Float(value) => Ok((value != 0.0).into()),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

#[derive(Debug, Default)]
struct BatteryStatus {
    charging: bool,
    ac_present: bool,
    health_ok: bool,
}

impl ValueParser for BatteryStatus {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Uint(val) => {
                let charging = val & 0x01 == 0x01;
                let ac_present = val & 0x02 == 0x02;
                let health_ok = val & 0x40 == 0x40;
                Ok(BatteryStatus {
                    charging,
                    ac_present,
                    health_ok,
                })
            }
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for MilliAmpereHours {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Uint(v) => Ok(Self(v.try_into()?)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for MilliAmpere {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Int(v) => Ok(Self(v.try_into()?)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for Watt {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Float(v) => Ok(Self(v)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for Volt {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Float(v) => Ok(Self(v)),
            DataValue::Uint(v) => Ok(Self(f32::from(u16::try_from(v)?) / 1000.0)),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for bool {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Flag(v) => Ok(v),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for u8 {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Uint(v) => Ok(u8::try_from(v)?),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for u32 {
    fn parse(val: DataValue) -> Result<Self> {
        match val {
            DataValue::Uint(v) => Ok(u32::try_from(v)?),
            _ => Err(InternalError::_DataValueError),
        }
    }
}

impl ValueParser for DataValue {
    fn parse(val: DataValue) -> Result<Self> {
        Ok(val)
    }
}

macro_rules! read_impl {
    ($struct:ident = $key:ident -> $out:tt) => {
        #[derive(Debug)]
        struct $struct;

        impl $crate::smc::ReadAction for $struct {
            type Out = $out;

            fn key(&self) -> CommandKey {
                $key
            }
        }
    };

    ($struct:ident($arg:tt) = $key:ident -> $out:tt) => {
        #[derive(Debug)]
        struct $struct($arg);

        impl $crate::smc::ReadAction for $struct {
            type Out = $out;

            fn key(&self) -> CommandKey {
                $key.set1(self.0)
            }
        }
    };

    ($struct:ident($arg:tt) == $key:ident -> $out:tt) => {
        #[derive(Debug)]
        struct $struct($arg);

        impl $crate::smc::ReadAction for $struct {
            type Out = $out;

            fn key(&self) -> CommandKey {
                $key.set2(self.0)
            }
        }
    };
}

static NUMBER_OF_KEYS: CommandKey = smc_key(b"#KEY");

static NUM_FANS: CommandKey = smc_key(b"FNum");
static FAN_MODE: CommandKey = smc_key(b"F0Md");
static FAN_SPEED_ACTUAL: CommandKey = smc_key(b"F0Ac");
static FAN_SPEED_MAX: CommandKey = smc_key(b"F0Mx");
static FAN_SPEED_MIN: CommandKey = smc_key(b"F0Mn");
static FAN_SPEED_SAFE: CommandKey = smc_key(b"F0Sf");
static FAN_SPEED_TARGET: CommandKey = smc_key(b"F0Tg");

static NUM_BATTERIES: CommandKey = smc_key(b"BNum");
static BATTERY_POWERED: CommandKey = smc_key(b"BATP");
static BATTERY_INFO: CommandKey = smc_key(b"BSIn");
static BATTERY_CYCLES: CommandKey = smc_key(b"B0CT");
static BATTERY_CURRENT_CAPACITY: CommandKey = smc_key(b"B0RM");
static BATTERY_FULL_CAPACITY: CommandKey = smc_key(b"B0FC");
static BATTERY_POWER: CommandKey = smc_key(b"B0AP");
static BATTERY_AMPERAGE: CommandKey = smc_key(b"B0AC");
static BATTERY_VOLTAGE: CommandKey = smc_key(b"B0AV");

static TEMP_BATTERY_MAX: CommandKey = smc_key(b"TB0T");
static TEMP_BATTERY_1: CommandKey = smc_key(b"TB1T");
static TEMP_BATTERY_2: CommandKey = smc_key(b"TB2T");

static TEMP_CPU_CORE: CommandKey = smc_key(b"TC0C");
static TEMP_CPU_DIE: CommandKey = smc_key(b"TC0F");
static TEMP_CPU_SYSTEM_AGENT: CommandKey = smc_key(b"TCSA");
static TEMP_CPU_GFX: CommandKey = smc_key(b"TCGC");
static TEMP_CPU_PROXIMITY: CommandKey = smc_key(b"TC0P");

static TEMP_GPU_PROXIMITY: CommandKey = smc_key(b"TG0P");
static TEMP_GPU_DIE: CommandKey = smc_key(b"TGDD");

static TEMP_MEM_PROXIMITY: CommandKey = smc_key(b"TM0P");
static TEMP_PLATFORM_CONTROLLER_HUB_DIE: CommandKey = smc_key(b"TPCD");
static TEMP_HEATPIPE_1: CommandKey = smc_key(b"Th1H");
static TEMP_HEATPIPE_2: CommandKey = smc_key(b"Th2H");
static TEMP_MAINBOARD_PROXIMITY: CommandKey = smc_key(b"Tm0P");

static TEMP_PALM_REST_1: CommandKey = smc_key(b"Ts0P");
static TEMP_PALM_REST_2: CommandKey = smc_key(b"Ts1P");
static TEMP_AIRPORT: CommandKey = smc_key(b"TW0P");
static TEMP_AIRFLOW_LEFT: CommandKey = smc_key(b"TaLC");
static TEMP_AIRFLOW_RIGHT: CommandKey = smc_key(b"TaRC");
static TEMP_THUNDERBOLT_LEFT: CommandKey = smc_key(b"TTLD");
static TEMP_THUNDERBOLT_RIGHT: CommandKey = smc_key(b"TTRD");

static POWER_CPU_CORE: CommandKey = smc_key(b"PCPC");
static POWER_CPU_DRAM: CommandKey = smc_key(b"PCPD");
static POWER_CPU_GFX: CommandKey = smc_key(b"PCPG");
static POWER_CPU_RAIL: CommandKey = smc_key(b"PC0R");
static POWER_CPU_TOTAL: CommandKey = smc_key(b"PCPT");
static POWER_DC_IN: CommandKey = smc_key(b"PDTR");
static POWER_GPU_RAIL: CommandKey = smc_key(b"PG0R");
static POWER_SYSTEM_TOTAL: CommandKey = smc_key(b"PSTR");

const fn smc_key(key: &'static [u8]) -> CommandKey {
    let key = [key[0], key[1], key[2], key[3]];
    let key = u32::from_be_bytes(key);
    CommandKey(key)
}

read_impl!(NumberOfKeys = NUMBER_OF_KEYS -> u32);

read_impl!(GetNumberOfFans = NUM_FANS -> u8);
read_impl!(GetActualFanSpeed(u8) = FAN_SPEED_ACTUAL -> Rpm);
read_impl!(GetMinFanSpeed(u8) = FAN_SPEED_MIN -> Rpm);
read_impl!(GetMaxFanSpeed(u8) = FAN_SPEED_MAX -> Rpm);
read_impl!(GetTargetFanSpeed(u8) = FAN_SPEED_TARGET -> Rpm);
read_impl!(GetSafeFanSpeed(u8) = FAN_SPEED_SAFE -> Rpm);
read_impl!(GetFanMode(u8) = FAN_MODE -> FanMode);

read_impl!(GetNumberOfBatteries = NUM_BATTERIES -> u8);
read_impl!(IsBatteryPowered = BATTERY_POWERED -> bool);
read_impl!(GetBatteryInfo = BATTERY_INFO -> BatteryStatus);
read_impl!(GetBatteryCycleCount(u8) = BATTERY_CYCLES -> u32);
read_impl!(GetBatteryCurrentCapacity(u8) = BATTERY_CURRENT_CAPACITY -> MilliAmpereHours);
read_impl!(GetBatteryFullCapacity(u8) = BATTERY_FULL_CAPACITY -> MilliAmpereHours);
read_impl!(GetBatteryAmperage(u8) = BATTERY_AMPERAGE -> MilliAmpere);
read_impl!(GetBatteryVoltage(u8) = BATTERY_VOLTAGE -> Volt);
read_impl!(GetBatteryPower(u8) = BATTERY_POWER -> Watt);
read_impl!(GetBatteryTemperatureMax = TEMP_BATTERY_MAX -> Celsius);
read_impl!(GetBatteryTemperature1 = TEMP_BATTERY_1 -> Celsius);
read_impl!(GetBatteryTemperature2 = TEMP_BATTERY_2 -> Celsius);

read_impl!(CpuProximityTemperature = TEMP_CPU_PROXIMITY -> Celsius);
read_impl!(CpuDieTemperature = TEMP_CPU_DIE -> Celsius);
read_impl!(CpuGfxTemperature = TEMP_CPU_GFX -> Celsius);
read_impl!(CpuSystemAgentTemperature = TEMP_CPU_SYSTEM_AGENT -> Celsius);
read_impl!(CpuCoreTemperature(u8) == TEMP_CPU_CORE -> Celsius);

read_impl!(GpuProximityTemperature = TEMP_GPU_PROXIMITY -> Celsius);
read_impl!(GpuDieTemperature = TEMP_GPU_DIE -> Celsius);

read_impl!(GetMemoryBankProximityTemperature = TEMP_MEM_PROXIMITY -> Celsius);
read_impl!(GetMainboardProximityTemperature = TEMP_MAINBOARD_PROXIMITY -> Celsius);
read_impl!(GetPCHDieTemperature = TEMP_PLATFORM_CONTROLLER_HUB_DIE -> Celsius);
read_impl!(GetAirportTemperature = TEMP_AIRPORT -> Celsius);
read_impl!(GetAirflowLeftTemperature = TEMP_AIRFLOW_LEFT -> Celsius);
read_impl!(GetAirflowRightTemperature = TEMP_AIRFLOW_RIGHT -> Celsius);
read_impl!(GetThunderboltLeftTemperature = TEMP_THUNDERBOLT_LEFT -> Celsius);
read_impl!(GetThunderboltRightTemperature = TEMP_THUNDERBOLT_RIGHT -> Celsius);
read_impl!(GetHeatpipe1Temperature = TEMP_HEATPIPE_1 -> Celsius);
read_impl!(GetHeatpipe2Temperature = TEMP_HEATPIPE_2 -> Celsius);
read_impl!(GetPalmRest1Temperature = TEMP_PALM_REST_1 -> Celsius);
read_impl!(GetPalmRest2Temperature = TEMP_PALM_REST_2 -> Celsius);

read_impl!(CpuCorePower = POWER_CPU_CORE -> Watt);
read_impl!(CpuDramPower = POWER_CPU_DRAM -> Watt);
read_impl!(CpuGfxPower = POWER_CPU_GFX -> Watt);
read_impl!(CpuRailPower = POWER_CPU_RAIL -> Watt);
read_impl!(CpuTotalPower = POWER_CPU_TOTAL -> Watt);
read_impl!(GpuRailPower = POWER_GPU_RAIL -> Watt);
read_impl!(DcInPower = POWER_DC_IN -> Watt);
read_impl!(SystemTotalPower = POWER_SYSTEM_TOTAL -> Watt);

struct Check<'a>(&'a str);

impl<'a> ReadAction for Check<'a> {
    type Out = DataValue;

    fn key(&self) -> CommandKey {
        let bytes = self.0.as_bytes();
        let key = u32::from_be_bytes(bytes.try_into().unwrap());
        CommandKey(key)
    }
}

#[derive(Clone, Debug, PartialEq)]
struct DataType(DataValue, u32);

macro_rules! int_tpe {
    ($data:ident as $narrow:ty as $wide:ty as $out:ident) => {{
        Ok($crate::smc::DataValue::$out(<$wide>::from(
            <$narrow>::from_be_bytes($data.try_into()?),
        )))
    }};
    ($data:ident as $wide:ty as $out:ident) => {{
        Ok($crate::smc::DataValue::$out(<$wide>::from_be_bytes(
            $data.try_into()?,
        )))
    }};
}

impl DataValue {
    fn convert(data: &[u8], tpe: u32) -> Result<Self> {
        let tpe_str = tpe.to_be_bytes();

        match &tpe_str {
            b"flag" => return Ok(DataValue::Flag(!data.is_empty() && data[0] != 0)),
            b"flt " => return Ok(DataValue::Float(f32::from_ne_bytes(data.try_into()?))),
            b"hex_" => match data.len() {
                1 => return int_tpe!(data as u8 as u64 as Uint),
                2 => return int_tpe!(data as u16 as u64 as Uint),
                4 => return int_tpe!(data as u32 as u64 as Uint),
                8 => return int_tpe!(data as u64 as u64 as Uint),
                _ => {}
            },
            b"ch8*" => {
                let has_nul_termiantor = data.contains(&0);
                let s = if has_nul_termiantor {
                    unsafe { CStr::from_ptr(data.as_ptr() as *const _) }
                        .to_string_lossy()
                        .into_owned()
                } else {
                    let mut data = data.to_vec();
                    data.push(0);
                    unsafe { CStr::from_ptr(data.as_ptr() as *const _) }
                        .to_string_lossy()
                        .into_owned()
                };
                return Ok(DataValue::Str(s));
            }
            _ => {}
        }

        match &tpe_str[..2] {
            b"fp" => {
                // fpXY, unsigned fixed point floats, X = integer width, Y = floating width
                let i = char_to_int(tpe_str[2]);
                let f = char_to_int(tpe_str[3]);
                if i + f == 16 {
                    let unsigned = u16::from_be_bytes(data.try_into()?);
                    return decode_fp_float(f32::from(unsigned), f);
                }
            }
            b"sp" => {
                // spXY, signed fixed point floats, X = integer width, Y = floating width
                let i = char_to_int(tpe_str[2]);
                let f = char_to_int(tpe_str[3]);
                if i + f == 15 {
                    let signed = i16::from_be_bytes(data.try_into()?);
                    return decode_fp_float(f32::from(signed), f);
                }
            }
            b"ui" => match &tpe_str[2..] {
                b"8 " => return int_tpe!(data as u8 as u64 as Uint),
                b"16" => return int_tpe!(data as u16 as u64 as Uint),
                b"32" => return int_tpe!(data as u32 as u64 as Uint),
                b"64" => return int_tpe!(data as u64 as Uint),
                _ => {}
            },
            b"si" => match &tpe_str[2..] {
                b"8 " => return int_tpe!(data as i8 as i64 as Int),
                b"16" => return int_tpe!(data as i16 as i64 as Int),
                b"32" => return int_tpe!(data as i32 as i64 as Int),
                b"64" => return int_tpe!(data as i64 as Int),
                _ => {}
            },
            _ => {}
        }

        Ok(DataValue::Unknown(data.to_vec()))
    }
}

fn char_to_int(c: u8) -> u8 {
    static A: u8 = b'a';
    static F: u8 = b'f';
    static N0: u8 = b'0';
    static N9: u8 = b'9';

    if c >= A && c <= F {
        c - A + 10
    } else if c >= N0 && c <= N9 {
        c - N0
    } else {
        0
    }
}

#[inline]
fn decode_fp_float(float: f32, f: u8) -> Result<DataValue> {
    Ok(DataValue::Float(float / f32::from(1_u16 << f)))
}

impl Into<u32> for DataType {
    fn into(self) -> u32 {
        self.1
    }
}

#[derive(Debug)]
struct KeyInfo {
    key: u32,
    data_type: u32,
    data_size: u32,
}

mod cffi {
    use super::*;
    use std::{mem::size_of, os::raw::c_void, ptr};

    #[allow(non_camel_case_types)]
    type kern_return_t = i32;
    #[allow(non_camel_case_types)]
    type ipc_port_t = *mut c_void;
    #[allow(non_camel_case_types)]
    type mach_port_t = ipc_port_t;
    #[allow(non_camel_case_types)]
    type io_object_t = mach_port_t;
    #[allow(non_camel_case_types)]
    type io_connect_t = io_object_t;
    #[allow(non_camel_case_types)]
    type task_t = *mut c_void;
    #[allow(non_camel_case_types)]
    type task_port_t = task_t;
    #[allow(non_camel_case_types)]
    type io_service_t = io_object_t;

    const MACH_PORT_NULL: mach_port_t = 0 as mach_port_t;
    const MASTER_PORT_DEFAULT: mach_port_t = MACH_PORT_NULL;

    const KERN_SUCCESS: kern_return_t = 0;
    const RETURN_SUCCESS: kern_return_t = KERN_SUCCESS;

    const SYS_IOKIT: kern_return_t = (0x38 & 0x3f) << 26;
    const SUB_IOKIT_COMMON: kern_return_t = 0;
    const RETURN_NOT_PRIVILEGED: kern_return_t = SYS_IOKIT | SUB_IOKIT_COMMON | 0x2c1;

    const KERNEL_INDEX_SMC: u32 = 2;

    pub(super) fn num_cpus() -> i32 {
        let mut cpus: i32 = 0;
        let mut cpus_size = std::mem::size_of_val(&cpus);

        let sysctl_name =
            CStr::from_bytes_with_nul(b"hw.physicalcpu\0").expect("byte literal is missing NUL");

        unsafe {
            if 0 != libc::sysctlbyname(
                sysctl_name.as_ptr(),
                &mut cpus as *mut _ as *mut _,
                &mut cpus_size as *mut _ as *mut _,
                ptr::null_mut(),
                0,
            ) {
                // On ARM targets, processors could be turned off to save power.
                // Use `_SC_NPROCESSORS_CONF` to get the real number.
                #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
                const CONF_NAME: libc::c_int = libc::_SC_NPROCESSORS_CONF;
                #[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
                const CONF_NAME: libc::c_int = libc::_SC_NPROCESSORS_ONLN;

                cpus = libc::sysconf(CONF_NAME) as i32;
            }
        }

        cpus.max(1)
    }

    #[derive(Debug)]
    pub(super) struct SMCConnection {
        conn: io_connect_t,
    }

    impl Drop for SMCConnection {
        fn drop(&mut self) {
            unsafe { _smc_close(self.conn) }
        }
    }

    impl SMCConnection {
        pub(super) fn new() -> Result<Self> {
            let conn = unsafe { _smc_open() }?;
            Ok(Self { conn })
        }

        pub(super) fn read_value<R>(&mut self, op: R) -> Result<R::Out>
        where
            R: ReadAction,
            R::Out: Default,
        {
            Ok(self.opt_read_value(op)?.unwrap_or_default())
        }

        pub(super) fn opt_read_value<R: ReadAction>(&mut self, op: R) -> Result<Option<R::Out>> {
            let result = self.try_read_value(op);
            match result {
                Ok(result) => Ok(Some(result)),
                Err(InternalError::_UnknownKey) => Ok(None),
                Err(e) => Err(e),
            }
        }

        fn try_read_value<R: ReadAction>(&mut self, op: R) -> Result<R::Out> {
            let key = *op.key();
            let result = unsafe { _smc_read_key(self.conn, key) };
            let result = result.map_err(|e| match e {
                InternalError::_DataKeyError(tpe) => InternalError::DataError { key, tpe },
                otherwise => otherwise,
            })?;
            let tpe = result.data_type;
            let data = &result.bytes.0[..result.data_size as usize];
            let data = DataValue::convert(data, tpe)?;
            op.parse(data).map_err(|e| match e {
                InternalError::_DataValueError => InternalError::DataError { key, tpe },
                otherwise => otherwise,
            })
        }

        pub(super) fn key_info<O: ReadAction>(&mut self, op: O) -> Result<KeyInfo> {
            let key = *op.key();
            let result = unsafe { _smc_key_info(self.conn, key) };
            result.map_err(|e| match e {
                InternalError::_UnknownKey => InternalError::DataError {
                    key,
                    tpe: *smc_key(b"????"),
                },
                otherwise => otherwise,
            })
        }

        pub(super) fn key_info_by_index(&mut self, index: u32) -> Result<KeyInfo> {
            let result = unsafe { _smc_key_index_info(self.conn, index) };
            result.map_err(|e| match e {
                InternalError::_UnknownKey => InternalError::DataError {
                    key: *smc_key(b"????"),
                    tpe: *smc_key(b"????"),
                },
                otherwise => otherwise,
            })
        }
    }

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    #[repr(u8)]
    enum SMCReadCommand {
        Data = 5,
        ByIndex = 8,
        KeyInfo = 9,
    }

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    #[repr(C)]
    struct SMCKeyData {
        key: u32,
        version: SMCKeyDataVersion,
        p_limit_data: SMCKeyDataLimitData,
        key_info: SMCKeyDataKeyInfo,
        result: u8,
        status: u8,
        data8: u8,
        data32: u32,
        bytes: SMCBytes,
    }

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    #[repr(C)]
    struct SMCKeyDataVersion {
        major: u8,
        minor: u8,
        build: u8,
        reserved: u8,
        release: u16,
    }

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    #[repr(C)]
    struct SMCKeyDataLimitData {
        version: u16,
        length: u16,
        cpu_p_limit: u32,
        gpu_p_limit: u32,
        mem_p_limit: u32,
    }

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    #[repr(C)]
    struct SMCKeyDataKeyInfo {
        data_size: u32,
        data_type: u32,
        data_attributes: u8,
    }

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    #[repr(transparent)]
    struct SMCBytes([u8; 32]);

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
    #[repr(C)]
    struct SMCVal {
        key: u32,
        data_size: u32,
        data_type: u32,
        bytes: SMCBytes,
    }

    #[repr(C)]
    struct __CFDictionary(c_void);

    type CFDictionaryRef = *const __CFDictionary;
    type CFMutableDictionaryRef = *mut __CFDictionary;

    #[link(name = "IOKit", kind = "framework")]
    extern "C" {
        fn IOServiceMatching(name: *const u8) -> CFMutableDictionaryRef;

        fn IOServiceGetMatchingService(
            master_port: mach_port_t,
            matching: CFDictionaryRef,
        ) -> io_service_t;

        fn IOServiceOpen(
            service: io_service_t,
            owning_task: task_port_t,
            r#type: u32,
            connect: *const io_connect_t,
        ) -> kern_return_t;

        fn IOServiceClose(connect: io_connect_t) -> kern_return_t;

        fn IOConnectCallStructMethod(
            connection: mach_port_t,
            selector: u32,
            input: *const c_void,
            input_size: usize,
            output: *mut c_void,
            output_size: *mut usize,
        ) -> kern_return_t;

        fn IOObjectRelease(object: io_object_t) -> kern_return_t;

        fn mach_task_self() -> mach_port_t;
    }

    unsafe fn _smc_open() -> Result<io_connect_t> {
        let matching_dictionary = IOServiceMatching(b"AppleSMC\0".as_ptr());
        let device = IOServiceGetMatchingService(MASTER_PORT_DEFAULT, matching_dictionary);

        if device.is_null() {
            return Err(InternalError::SmcNotFound);
        }

        let result: kern_return_t;
        let conn: io_connect_t = MASTER_PORT_DEFAULT;
        result = IOServiceOpen(device, mach_task_self(), 0, &conn);
        IOObjectRelease(device);

        if result != RETURN_SUCCESS {
            return Err(InternalError::SmcFailedToOpen(result));
        }

        Ok(conn)
    }

    unsafe fn _smc_close(conn: io_connect_t) {
        let _ = IOServiceClose(conn);
    }

    unsafe fn _smc_read_key(conn: mach_port_t, key: u32) -> Result<SMCVal> {
        let mut input = SMCKeyData::default();
        input.key = key;
        input.data8 = SMCReadCommand::KeyInfo as u8;

        let mut output = SMCKeyData::default();
        _smc_call(conn, &input, &mut output)?;

        let data_type = output.key_info.data_type;
        let data_size = output.key_info.data_size;

        if data_size > 32 {
            return Err(InternalError::_DataKeyError(data_type));
        }

        input.key_info.data_size = data_size;
        input.data8 = SMCReadCommand::Data as u8;

        _smc_call(conn, &input, &mut output)?;

        let val = SMCVal {
            key,
            data_size,
            data_type,
            bytes: output.bytes,
        };

        Ok(val)
    }

    unsafe fn _smc_key_info(conn: mach_port_t, key: u32) -> Result<KeyInfo> {
        let mut input = SMCKeyData::default();
        input.key = key;
        input.data8 = SMCReadCommand::KeyInfo as u8;

        let mut output = SMCKeyData::default();
        _smc_call(conn, &input, &mut output)?;

        let data_type = output.key_info.data_type;
        let data_size = output.key_info.data_size;

        Ok(KeyInfo {
            key,
            data_type,
            data_size,
        })
    }

    unsafe fn _smc_key_index_info(conn: mach_port_t, index: u32) -> Result<KeyInfo> {
        let mut input = SMCKeyData::default();
        input.data8 = SMCReadCommand::ByIndex as u8;
        input.data32 = index;

        let mut output = SMCKeyData::default();
        _smc_call(conn, &input, &mut output)?;

        let key = output.key;
        let data_type = output.key_info.data_type;
        let data_size = output.key_info.data_size;

        Ok(KeyInfo {
            key,
            data_type,
            data_size,
        })
    }

    unsafe fn _smc_call(
        conn: mach_port_t,
        input: &SMCKeyData,
        output: &mut SMCKeyData,
    ) -> Result<()> {
        let mut output_size = size_of::<SMCKeyData>();

        let result = IOConnectCallStructMethod(
            conn,
            KERNEL_INDEX_SMC,
            input as *const _ as *const c_void,
            size_of::<SMCKeyData>(),
            output as *mut _ as *mut c_void,
            &mut output_size,
        );

        if result == RETURN_NOT_PRIVILEGED {
            return Err(InternalError::NotPrivlileged);
        }
        if result != RETURN_SUCCESS {
            return Err(InternalError::UnknownSmc(result, output.result));
        }
        if output.result == 132 {
            return Err(InternalError::_UnknownKey);
        }

        Ok(())
    }
}
