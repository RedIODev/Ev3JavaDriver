use ev3dev_lang_rust::{motors::MotorPort, sensors::SensorPort};
use jni::{JNIEnv, objects::{JObject, JClass}, errors::Error, sys::jobjectArray, descriptors::Desc};
use crate::{errors::{EnumConversionError, Ev3JApiError}, jni_shortcuts::getClassSpecifier};

pub struct IntEnum(i32);

pub enum ColorMode {
    Color,
    Reflect,
    Ambient
}

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

impl TryInto<ColorMode> for IntEnum {
    type Error = EnumConversionError;

    fn try_into(self) -> Result<ColorMode, Self::Error> {
        match self.0 {
            0 => Ok(ColorMode::Color),
            1 => Ok(ColorMode::Reflect),
            2 => Ok(ColorMode::Ambient),
            _ => Err(EnumConversionError),
        }
    }
}

pub trait JavaEnum {
    fn ordinal(self, jre: JNIEnv) -> Result<IntEnum,Error>;

    fn values<'a, 'c, T>(jre: JNIEnv<'a>, class: T) -> Result<jobjectArray, Ev3JApiError>
    where T: Desc<'a, JClass<'c>>,;
}

impl JavaEnum for JObject<'_> {
    fn ordinal(self, jre: JNIEnv) -> Result<IntEnum,Error> {
        let ordinal = jre.call_method(*self, "ordinal", "()I", &[])?
                .i()?;
        Ok(ordinal.into())   
    }

    fn values<'a, 'c, T>(jre: JNIEnv<'a>, class: T) -> Result<jobjectArray, Ev3JApiError>
    where T: Desc<'a, JClass<'c>>, {
        let class = class.lookup(&jre)?;
        let mut sig = getClassSpecifier(jre, class)?;
        sig.insert_str(0, "()");
        let obj = jre.call_static_method(class, "values", sig, &[])?.l()?;
        Ok(obj.into_inner())
        
    }
}