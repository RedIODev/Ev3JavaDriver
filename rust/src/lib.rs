#![feature(result_flattening)]


//use jni::{sys::jobjectArray, objects::JObject, JNIEnv};
use jni_proc_macro::jni_mod;

mod alloc;
mod enum_conversions;
mod errors;
mod result_extentions;

mod test;


#[jni_mod]
mod ffi {
    use ev3dev_lang_rust::motors::LargeMotor;
    use jni::{JNIEnv, sys::jobjectArray, objects::JObject};
    use jni_proc_macro::jni_func;


    use crate::{errors::Ev3JApiError, enum_conversions::{JavaEnum, IntEnum}, result_extentions::FlattenInto, alloc::RustObjectCarrier};

    #[jni_func]
    fn LargeMotor_new0(jre: &JNIEnv, this: &JObject, args: &jobjectArray) -> Result<(), Ev3JApiError> {
        let port = jre
            .get_object_array_element(*args, 0)
            .map(|o| o.ordinal(&jre))
            .flatten()
            .map(IntEnum::try_into)
            .flatten_into::<Ev3JApiError>()?;
        let motor = LargeMotor::get(port);
        this.store(&jre, motor)?;
        Ok(())
    }

    fn test(jre: &JNIEnv) -> Result<i32,Ev3JApiError> {
        Ok(5)
    }
}