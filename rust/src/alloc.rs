use std::sync::MutexGuard;

use jni::{JNIEnv, objects::JObject, errors::Result};

const PTR_FIELD_NAME: &'static str = "ptr";

impl<T> JNIStruct for T where T: Sized + Send + 'static {}

pub trait JNIStruct: Sized + Send + 'static {
    fn store(self, env: &mut JNIEnv, obj: &mut JObject) -> Result<()> {
        env.set_rust_field(*obj, PTR_FIELD_NAME, self)?;
        Ok(())
    }

    fn borrow<'a>(env: &'a mut JNIEnv, obj: &'a mut JObject) -> Result<MutexGuard<'a,Self>> {
        env.get_rust_field(*obj, PTR_FIELD_NAME)
    }

    fn delete(env: &mut JNIEnv, obj: &mut JObject) -> Result<()> {
        env.take_rust_field(*obj, PTR_FIELD_NAME)?;
        Ok(())
    }
}
