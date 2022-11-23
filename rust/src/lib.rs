// use std::ffi::c_void;

// use jni::{sys::JavaVM, sys::jint};
use jni_proc_macro::jni_package;
//use std::arch::global_asm;
mod alloc;
mod enum_conversions;
mod errors;
mod result_extensions;

//global_asm!(".symver realpath,realpath@GLIBC_2.24");


//mod test;
// #[no_mangle]
// pub extern "C" fn JNI_Onload_ev3(_vm:*mut JavaVM, _ptr:*mut c_void) -> jint {
//     jni::sys::JNI_VERSION_1_8
// }

#[jni_package("dev.redio.ev3dev")]
pub mod dev_redio_ev3dev {
    use jni_proc_macro::jni_class;

    #[jni_class("LargeMotor")]
    pub mod large_motor {
        use ev3dev_lang_rust::motors::LargeMotor;
        use jni::{objects::{JObject, JString}, sys::jobjectArray, JNIEnv};

        use crate::{
            alloc::RustObjectCarrier,
            enum_conversions::{IntEnum, JavaEnum},
            errors::Ev3JApiError,
            result_extensions::{FlattenInto, MapAuto},
        };

        fn new0(jre: JNIEnv, this: JObject, args: jobjectArray) -> Result<(), Ev3JApiError> {
            let port = jre
                .get_object_array_element(args, 0)
                .and_then(|o| o.ordinal(&jre))
                .map(IntEnum::try_into)
                .flatten_into::<Ev3JApiError>()?;
            let motor = LargeMotor::get(port)?;
            this.store(&jre, motor)
                    .map_auto()
        }

        fn delete0(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            this.take(&jre)?;
            Ok(())
        }

        fn getCountPerRotation(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .get_count_per_rot()
                    .map_auto()
        }

        fn getDutyCycle(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .get_duty_cycle()
                    .map_auto()
        }

        fn getDutyCycleSp(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .get_duty_cycle_sp()
                    .map_auto()
        }

        fn getFullTravelCount(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .get_full_travel_count()
                    .map_auto()
        }

        fn getHoldPidKd(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .get_hold_pid_kd()
                    .map_auto()
        }

        fn getHoldPidKi(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .get_hold_pid_ki()
                    .map_auto()
        }

        fn getHoldPidKp(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .get_hold_pid_kp()
                    .map_auto()
        }

        fn getMaxSpeed(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .get_max_speed()
                    .map_auto()
        }

        fn getPolarity<'a>(jre: JNIEnv<'a>, this: JObject<'a>) -> Result<JString<'a>, Ev3JApiError> {
            let str = this.borrow::<LargeMotor>(&jre)?
                    .get_polarity()?;
            jre.new_string(str)
                    .map_auto()
        }
        //...

        fn setDutyCycleSp(jre: JNIEnv, this: JObject, duty_cycle: i32) -> Result<(), Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .set_duty_cycle_sp(duty_cycle)
                    .map_auto()
        }

        //...

        fn runDirect(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            this.borrow::<LargeMotor>(&jre)?
                    .run_direct()
                    .map_auto()
        }
    }
}
