use std::sync::MutexGuard;

use jni::{JNIEnv, objects::JObject, errors::Result};

pub trait RustObjectCarrier {

    fn store<T>(&self, env: JNIEnv, rust_object: T) -> Result<()>
    where T: Send + 'static;

    fn borrow<'a, T>(&'a self, env: &'a JNIEnv<'a>) -> Result<MutexGuard<'a,T>>
    where T: Send + 'static;

    fn take<T>(&self, env: JNIEnv) -> Result<T>
    where T: Send + 'static;
}

const PTR_FIELD_NAME: &str = "ptr";

impl RustObjectCarrier for JObject<'_> {

    fn store<T>(&self, env: JNIEnv, rust_object: T) -> Result<()>
    where T: Send + 'static {
        env.set_rust_field(*self, PTR_FIELD_NAME, rust_object)?;
        Ok(())
    }

    fn borrow<'a, T>(&'a self, env: &'a JNIEnv<'a>) -> Result<MutexGuard<'a, T>>
    where T: Send + 'static {
        env.get_rust_field(*self, PTR_FIELD_NAME)
    }

    fn take<T>(&self, env: JNIEnv) -> Result<T> 
    where T: Send + 'static {
        let f = env.take_rust_field(*self, PTR_FIELD_NAME)?;
         Ok(f)
    }
}
