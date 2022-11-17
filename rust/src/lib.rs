#![feature(result_flattening)]

use std::ffi::c_void;

use jni::{JavaVM, sys::jint};
use jni_proc_macro::jni_package;

mod alloc;
mod enum_conversions;
mod errors;
mod result_extensions;
//mod test;
#[no_mangle]
pub extern "system" fn JNI_Onload_ev3(_vm:JavaVM, _ptr:*mut c_void) -> jint {
    jni::sys::JNI_VERSION_1_8
}

#[jni_package("dev.redio.ev3dev")]
pub mod dev_redio_ev3dev {
    use jni_proc_macro::jni_class;

    #[jni_class("LargeMotor")]
    pub mod large_motor {
        use std::sync::MutexGuard;

        use ev3dev_lang_rust::motors::LargeMotor;
        use jni::{objects::JObject, sys::jobjectArray, JNIEnv};

        use crate::{
            alloc::RustObjectCarrier,
            enum_conversions::{IntEnum, JavaEnum},
            errors::Ev3JApiError,
            result_extensions::FlattenInto,
        };

        type MutMotor<'a> = MutexGuard<'a, LargeMotor>;

        fn new0(jre: JNIEnv, this: JObject, args: jobjectArray) -> Result<(), Ev3JApiError> {
            let port = jre
                .get_object_array_element(args, 0)
                .and_then(|o| o.ordinal(&jre))
                .map(IntEnum::try_into)
                .flatten_into::<Ev3JApiError>()?;
            let motor = LargeMotor::get(port)?;
            this.store(&jre, motor)?;
            Ok(())
        }

        fn delete0(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            this.take(&jre)?;
            Ok(())
        }

        fn getCountPerRotation(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            let motor:MutMotor = this.borrow(&jre)?;
            let result = motor.get_count_per_rot()?;
            Ok(result)
        }

        //...

        fn setDutyCycleSp(jre: JNIEnv, this: JObject, duty_cycle: i32) -> Result<(), Ev3JApiError> {
            let motor: MutMotor = this.borrow(&jre)?;
            motor.set_duty_cycle_sp(duty_cycle)?;
            Ok(())
        }

        //...

        fn runDirect(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            let motor:MutMotor = this.borrow(&jre)?;
            motor.run_direct()?;
            Ok(())
        }
    }
}
