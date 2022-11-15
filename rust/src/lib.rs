#![feature(result_flattening)]

use alloc::RustObjectCarrier;
use enum_conversions::{IntEnum, JavaEnum};
use errors::{Ev3JApiError, Ev3JApiResult};
use ev3dev_lang_rust::motors::LargeMotor;
use jni::{objects::JObject, sys::jobjectArray, JNIEnv};
use jni_proc_macro::jni_func;
use result_extentions::FlattenInto;

mod alloc;
mod enum_conversions;
mod errors;
mod result_extentions;




#[no_mangle]
pub extern "system" fn Java_dev_redio_ev3dev_LargeMotor_new0(
    env: JNIEnv,
    this: JObject,
    args: jobjectArray,
) {
    if let Err(err) = LargeMotor_new0(&env, &this, &args) {
        env.throw(err).unwrap();
    }
}

//#[allow(non_snake_case)]
#[jni_func]
fn LargeMotor_new0(env: &JNIEnv, this: &JObject, args: &jobjectArray) -> Result<(), Ev3JApiError> {
    let port: Ev3JApiResult<_> = env
        .get_object_array_element(*args, 0)
        .map(|o| o.ordinal(&env))
        .flatten()
        .map(IntEnum::try_into)
        .flatten_into();

    let motor = LargeMotor::get(port?);
    this.store(&env, motor)?;
    Ok(())
}
