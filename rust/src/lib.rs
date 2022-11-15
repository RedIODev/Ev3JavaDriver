#![feature(result_flattening)]
#![feature(try_blocks)]


//use jni::{sys::jobjectArray, objects::JObject, JNIEnv};
use jni_proc_macro::jni_func;

mod alloc;
mod enum_conversions;
mod errors;
mod result_extentions;

mod test;


#[jni_func]
mod jni_funcs {
    use ev3dev_lang_rust::motors::LargeMotor;
    use jni::{JNIEnv, sys::jobjectArray, objects::JObject};


    use crate::{errors::{Ev3JApiResult, Ev3JApiError}, enum_conversions::{JavaEnum, IntEnum}, result_extentions::FlattenInto, alloc::RustObjectCarrier};


    fn LargeMotor_new0(jre: &JNIEnv, this: &JObject, args: &jobjectArray) -> Result<(), Ev3JApiError> {
        let port: Ev3JApiResult<_> = jre
            .get_object_array_element(*args, 0)
            .map(|o| o.ordinal(&jre))
            .flatten()
            .map(IntEnum::try_into)
            .flatten_into();
        let motor = LargeMotor::get(port?);
        this.store(&jre, motor)?;
        Ok(())
    }

    fn test(jre: &JNIEnv) -> Result<(),Ev3JApiError> {
        Ok(())
    }
}

