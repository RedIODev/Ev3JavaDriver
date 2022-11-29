use std::alloc::LayoutError;

use crate::{
    errors::{EnumConversionError, Ev3JApiError},
    jni_shortcuts::getClassSpecifier,
};
use ev3dev_lang_rust::{
    motors::{MotorPort, TachoMotor},
    sensors::{Sensor, SensorPort},
};
use jni::{
    descriptors::Desc,
    errors::Error,
    objects::{JClass, JObject},
    sys::jobjectArray,
    JNIEnv,
};
use num_traits::{AsPrimitive, Num, PrimInt, Unsigned};

use self::{motor::{Polarity, StopAction, State}, sensors::color::ColorMode};

//pub struct IntEnum(i32);

///# Safety
/// Be careful when setting size.
/// Update size if enum variant_count changes.
///
pub unsafe trait Enum<B = u32> {
    const SIZE: B;

    fn ordinal<T>(&self) -> Result<T, EnumConversionError>
    where
        Self: Sized,
        T: Unsigned + Copy,
    {
        let source = std::mem::size_of::<Self>();
        let target = std::mem::size_of::<T>();
        if source > target {
            return Err(EnumConversionError);
        }
        unsafe { Ok(*(self as *const _ as *const T)) }
    }

    fn from_ordinal<T>(ordinal: T) -> Result<Self, EnumConversionError>
    where
        Self: Sized + Copy,
        T: Unsigned + PartialOrd<B>,
    {
        let source = std::mem::size_of::<Self>();
        let target = std::mem::size_of::<T>();
        if ordinal >= Self::SIZE || source > target {
            return Err(EnumConversionError);
        }
        unsafe { Ok(*(&ordinal as *const _ as *const Self)) }
    }
}

unsafe impl Enum for ColorMode {
    const SIZE: u32 = 5;
}

unsafe impl Enum for MotorPort {
    const SIZE: u32 = 4;
}

unsafe impl Enum for SensorPort {
    const SIZE: u32 = 4;
}

unsafe impl Enum for Polarity {
    const SIZE: u32 = 2;
}

unsafe impl Enum for StopAction {
    const SIZE: u32 = 3;
}

unsafe impl Enum for State {
    const SIZE: u32 = 5;
}

pub mod sensors {

    pub mod color {
        #[derive(Clone, Copy)]
        pub enum ColorMode {
            SimpleColor,
            Color,
            SimpleReflect,
            Reflect,
            Ambient,
        }
    }

}

pub mod motor {
    use ev3dev_lang_rust::motors::TachoMotor;

    use crate::errors::EnumConversionError;

    #[derive(Clone, Copy)]
    pub enum Polarity {
        Normal,
        Inversed,
    }

    impl Polarity {
        pub fn value(&self) -> &'static str {
            use Polarity::*;
            match self {
                Normal => TachoMotor::POLARITY_NORMAL,
                Inversed => TachoMotor::POLARITY_INVERSED,
            }
        }

        pub fn from_value(value: &str) -> Result<Self, EnumConversionError> {
            use Polarity::*;
            let value = match value {
                TachoMotor::POLARITY_NORMAL => Normal,
                TachoMotor::POLARITY_INVERSED => Inversed,
                _ => return Err(EnumConversionError)
            };
            Ok(value)
        }
    }

    #[derive(Clone, Copy)]
    pub enum StopAction {
        Brake,
        Coast,
        Hold,
    }

    impl StopAction {
        pub fn value(&self) -> &'static str {
            use StopAction::*;
            match self {
                Brake => TachoMotor::STOP_ACTION_BRAKE,
                Coast => TachoMotor::STOP_ACTION_COAST,
                Hold => TachoMotor::STOP_ACTION_HOLD,
            }
        }

        pub fn from_value(value: &str) -> Result<Self, EnumConversionError> {
            use StopAction::*;
            let value = match value {
                TachoMotor::STOP_ACTION_BRAKE => Brake,
                TachoMotor::STOP_ACTION_COAST => Coast,
                TachoMotor::STOP_ACTION_HOLD => Hold,
                _ => return Err(EnumConversionError)
            };
            Ok(value)
        }
    }

    #[derive(Clone, Copy)]
    pub enum State {
        Holding,
        Overloaded,
        Ramping,
        Running,
        Stalled
    }

    impl State {
        pub fn value(&self) -> &'static str {
            use State::*;
            match self {
                Holding => TachoMotor::STATE_HOLDING,
                Overloaded => TachoMotor::STATE_OVERLOADED,
                Ramping => TachoMotor::STATE_RAMPING,
                Running => TachoMotor::STATE_RUNNING,
                Stalled => TachoMotor::STATE_STALLED
            }
        }

        pub fn from_value(value: &str) -> Result<Self, EnumConversionError> {
            use State::*;
            let value = match value {
                TachoMotor::STATE_HOLDING => Holding,
                TachoMotor::STATE_OVERLOADED => Overloaded,
                TachoMotor::STATE_RAMPING => Ramping,
                TachoMotor::STATE_RUNNING => Running,
                TachoMotor::STATE_STALLED => Stalled,
                _ => return Err(EnumConversionError)
            };
            Ok(value)
        }
    }


}
// impl AsRef<i32> for IntEnum {
//     fn as_ref(&self) -> &i32 {
//         &self.0
//     }
// }

// impl From<i32> for IntEnum {
//     fn from(value: i32) -> Self {
//         Self(value)
//     }
// }

// impl TryInto<MotorPort> for IntEnum {
//     type Error = EnumConversionError;

//     fn try_into(self) -> Result<MotorPort, Self::Error> {
//         match self.0 {
//             0 => Ok(MotorPort::OutA),
//             1 => Ok(MotorPort::OutB),
//             2 => Ok(MotorPort::OutC),
//             3 => Ok(MotorPort::OutD),
//             _ => Err(EnumConversionError),
//         }
//     }
// }

// impl TryInto<SensorPort> for IntEnum {
//     type Error = EnumConversionError;

//     fn try_into(self) -> Result<SensorPort, Self::Error> {
//         match self.0 {
//             0 => Ok(SensorPort::In1),
//             1 => Ok(SensorPort::In2),
//             2 => Ok(SensorPort::In3),
//             3 => Ok(SensorPort::In4),
//             _ => Err(EnumConversionError),
//         }
//     }
// }

// impl TryInto<ColorMode> for IntEnum {
//     type Error = EnumConversionError;

//     fn try_into(self) -> Result<ColorMode, Self::Error> {
//         match self.0 {
//             0 => Ok(ColorMode::Color),
//             1 => Ok(ColorMode::Reflect),
//             2 => Ok(ColorMode::Ambient),
//             _ => Err(EnumConversionError),
//         }
//     }
// }

pub trait JavaEnum {
    fn ordinal(self, jre: JNIEnv) -> Result<i32, Error>;

    fn values<'a, 'c, T>(jre: JNIEnv<'a>, class: T) -> Result<jobjectArray, Ev3JApiError>
    where
        T: Desc<'a, JClass<'c>>;
}

impl JavaEnum for JObject<'_> {
    fn ordinal(self, jre: JNIEnv) -> Result<i32, Error> {
        let ordinal = jre.call_method(*self, "ordinal", "()I", &[])?.i()?;
        Ok(ordinal)
    }

    fn values<'a, 'c, T>(jre: JNIEnv<'a>, class: T) -> Result<jobjectArray, Ev3JApiError>
    where
        T: Desc<'a, JClass<'c>>,
    {
        let class = class.lookup(&jre)?;
        let mut sig = getClassSpecifier(jre, class)?;

        sig.insert_str(0, "()["); //empty arg list and arrayPrefix
        let obj = jre.call_static_method(class, "values", sig, &[])?.l()?;
        Ok(obj.into_inner())
    }
}
