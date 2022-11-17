use std::sync::MutexGuard;

use jni::{JNIEnv, objects::JObject, errors::Result};

pub trait RustObjectCarrier<T> where T: Sized + Send + 'static {

    fn store(&self, env: &JNIEnv, rust_object: T) -> Result<()>;

    fn borrow<'a>(&'a self, env: &'a JNIEnv) -> Result<MutexGuard<'a,T>>;

    fn take(&self, env: &JNIEnv) -> Result<T>;
}

const PTR_FIELD_NAME: &str = "ptr";

impl<T> RustObjectCarrier<T> for JObject<'_>  where T: Sized + Send + 'static {

    fn store(&self, env: &JNIEnv, rust_object: T) -> Result<()> {
        env.set_rust_field(*self, PTR_FIELD_NAME, rust_object)?;
        Ok(())
    }

    fn borrow<'a>(&'a self, env: &'a JNIEnv) -> Result<MutexGuard<'a,T>> {
        env.get_rust_field(*self, PTR_FIELD_NAME)
    }

    fn take(&self, env: &JNIEnv) -> Result<T> {
        let f = env.take_rust_field(*self, PTR_FIELD_NAME)?;
         Ok(f)
    }
}
