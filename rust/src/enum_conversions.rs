use ev3dev_lang_rust::{motors::MotorPort, sensors::SensorPort};
use jni::{JNIEnv, objects::JObject, errors::Error};
use crate::errors::EnumConversionError;

pub struct IntEnum(i32);

impl From<i32> for IntEnum {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

impl TryInto<MotorPort> for IntEnum {
    type Error = EnumConversionError;

    fn try_into(self) -> Result<MotorPort, Self::Error> {
        match self.0 {
            0 => Ok(MotorPort::OutA),
            1 => Ok(MotorPort::OutB),
            2 => Ok(MotorPort::OutC),
            3 => Ok(MotorPort::OutD),
            _ => Err(EnumConversionError),
        }
    }
}

impl TryInto<SensorPort> for IntEnum {
    type Error = EnumConversionError;

    fn try_into(self) -> Result<SensorPort, Self::Error> {
        match self.0 {
            0 => Ok(SensorPort::In1),
            1 => Ok(SensorPort::In2),
            2 => Ok(SensorPort::In3),
            3 => Ok(SensorPort::In4),
            _ => Err(EnumConversionError),
        }
    }
}

pub trait JavaEnum {
    fn ordinal(&self, env: &JNIEnv) -> Result<IntEnum,Error>;
}

impl JavaEnum for JObject<'_> {
    fn ordinal(&self, env: &JNIEnv) -> Result<IntEnum,Error> {
        let ordinal = env.call_method(*self, "ordinal", "()I", &[])?
                .i()?;
        Ok(ordinal.into())   
    }
}