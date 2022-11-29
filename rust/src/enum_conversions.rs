use ev3dev_lang_rust::{motors::{MotorPort, TachoMotor}, sensors::SensorPort};
use jni::{JNIEnv, objects::{JObject, JClass}, errors::Error, sys::jobjectArray, descriptors::Desc};
use crate::{errors::{EnumConversionError, Ev3JApiError}, jni_shortcuts::getClassSpecifier};

#[derive(Clone, Copy)]
pub struct Enum<T>(pub T);

#[derive(Clone, Copy)]
pub enum ColorMode {
    SimpleColor,
    Color,
    Reflect,
    RawReflect,
    Ambient
}

impl Into<i32> for Enum<MotorPort> {
    fn into(self) -> i32 {
        use MotorPort::*;
        match self.0 {
            OutA => 0,
            OutB => 1,
            OutC => 2,
            OutD => 3,
        }
    }
}

impl TryFrom<i32> for Enum<MotorPort> {
    type Error = EnumConversionError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use MotorPort::*;
        match value {
            0 => Ok(Enum(OutA)),
            1 => Ok(Enum(OutB)),
            2 => Ok(Enum(OutC)),
            3 => Ok(Enum(OutD)),
            _ => Err(EnumConversionError),
        }
    }
}

impl Into<i32> for Enum<SensorPort> {
    fn into(self) -> i32 {
        use SensorPort::*;
        match self.0 {
            In1 => 0,
            In2 => 1,
            In3 => 2,
            In4 => 3,
        }
    }
}

impl TryFrom<i32> for Enum<SensorPort> {

    type Error = EnumConversionError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use SensorPort::*;
        match value {
            0 => Ok(Enum(In1)),
            1 => Ok(Enum(In2)),
            2 => Ok(Enum(In3)),
            3 => Ok(Enum(In4)),
            _ => Err(EnumConversionError),
        }
    }
}

impl Into<i32> for Enum<ColorMode> {
    fn into(self) -> i32 {
        use ColorMode::*;
        match self.0 {
            SimpleColor => 0,
            Color => 1,
            Reflect => 2,
            RawReflect => 3,
            Ambient => 4,
        }
    }
}

impl TryFrom<i32> for Enum<ColorMode> {

    type Error = EnumConversionError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use ColorMode::*;
        match value {
            0 => Ok(Enum(SimpleColor)),
            1 => Ok(Enum(Color)),
            2 => Ok(Enum(Reflect)),
            3 => Ok(Enum(RawReflect)),
            4 => Ok(Enum(Ambient)),
            _ => Err(EnumConversionError),
        }
    }
}

#[derive(Clone, Copy)]
pub enum MotorState {
    Holding,
    Overloaded,
    Ramping,
    Running,
    Stalled
}

impl MotorState {
    pub const fn value(self) -> &'static str {
        match self {
            MotorState::Holding => TachoMotor::STATE_HOLDING,
            MotorState::Overloaded => TachoMotor::STATE_OVERLOADED,
            MotorState::Ramping => TachoMotor::STATE_RAMPING,
            MotorState::Running => TachoMotor::STATE_RUNNING,
            MotorState::Stalled => TachoMotor::STATE_STALLED,
        }
    }
}

impl Into<i32> for Enum<MotorState> {
    fn into(self) -> i32 {
        use MotorState::*;
        match self.0 {
            Holding => 0,
            Overloaded => 1,
            Ramping => 2,
            Running => 3,
            Stalled => 4,
        }
    }
}

impl TryFrom<i32> for Enum<MotorState> {

    type Error = EnumConversionError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use MotorState::*;
        match value {
            0 => Ok(Enum(Holding)),
            1 => Ok(Enum(Overloaded)),
            2 => Ok(Enum(Ramping)),
            3 => Ok(Enum(Running)),
            4 => Ok(Enum(Stalled)),
            _ => Err(EnumConversionError),
        }
    }
}

#[derive(Clone, Copy)]
pub enum StopAction {
    Brake,
    Coast,
    Hold,
}

impl StopAction {
    pub const fn value(self) -> &'static str {
        match self {
            StopAction::Brake => TachoMotor::STOP_ACTION_BRAKE,
            StopAction::Coast => TachoMotor::STOP_ACTION_COAST,
            StopAction::Hold => TachoMotor::STOP_ACTION_HOLD,
        }
    }
}

impl Into<i32> for Enum<StopAction> {
    fn into(self) -> i32 {
        use StopAction::*;
        match self.0 {
            Brake => 0,
            Coast => 1,
            Hold => 2,
        }
    }
}

impl TryFrom<i32> for Enum<StopAction> {

    type Error = EnumConversionError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use StopAction::*;
        match value {
            0 => Ok(Enum(Brake)),
            1 => Ok(Enum(Coast)),
            2 => Ok(Enum(Hold)),
            _ => Err(EnumConversionError),
        }
    }
}

#[derive(Clone, Copy)]
pub enum Polarity {
    Normal,
    Inversed,
}

impl Polarity {
    pub const fn value(self) -> &'static str {
        match self {
            Polarity::Normal => TachoMotor::POLARITY_NORMAL,
            Polarity::Inversed => TachoMotor::POLARITY_INVERSED,
        }
    }
}

impl Into<i32> for Enum<Polarity> {
    fn into(self) -> i32 {
        use Polarity::*;
        match self.0 {
            Normal => 0,
            Inversed => 1,
        }
    }
}

impl TryFrom<i32> for Enum<Polarity> {

    type Error = EnumConversionError;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        use Polarity::*;
        match value {
            0 => Ok(Enum(Normal)),
            1 => Ok(Enum(Inversed)),
            _ => Err(EnumConversionError),
        }
    }
}

// pub trait JavaEnum<T> {
//     fn ordinal(self, jre: JNIEnv) -> Result<Enum<T>,Ev3JApiError>;

//     fn values<'a, 'c, U>(jre: JNIEnv<'a>, class: U) -> Result<jobjectArray, Ev3JApiError>
//     where U: Desc<'a, JClass<'c>>,;
// }

// impl<T> JavaEnum<T> for JObject<'_>
// where
//     Enum<T>: TryFrom<i32>,
//     Ev3JApiError: From<<Enum<T> as TryFrom<i32>>::Error> {
//     fn ordinal(self, jre: JNIEnv) -> Result<Enum<T>,Ev3JApiError> {
//         jre.call_method(*self, "ordinal", "()I", &[])?
//                 .i()?
//                 .try_into()
//                 .map_err(Ev3JApiError::from)
           
//     }

//     fn values<'a, 'c, U>(jre: JNIEnv<'a>, class: U) -> Result<jobjectArray, Ev3JApiError>
//     where U: Desc<'a, JClass<'c>>, {
//         let class = class.lookup(&jre)?;
//         let mut sig = getClassSpecifier(jre, class)?;
        
//         sig.insert_str(0, "()[");   //empty arg list and arrayPrefix
//         let obj = jre.call_static_method(class, "values", sig, &[])?.l()?;
//         Ok(obj.into_inner())
        
//     }
// }