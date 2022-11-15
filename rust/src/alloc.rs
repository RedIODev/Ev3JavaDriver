use std::sync::MutexGuard;

use jni::{JNIEnv, objects::JObject, errors::Result};

pub trait RustObjectCarrier<T> where T: Sized + Send + 'static {

    fn store(&self, env: &JNIEnv, rust_object: T) -> Result<()>;

    fn borrow<'a>(&'a self, env: &'a JNIEnv) -> Result<MutexGuard<'a,T>>;

    fn delete(&self, env: &JNIEnv) -> Result<()>;
}

const PTR_FIELD_NAME: &'static str = "ptr";

impl<T> RustObjectCarrier<T> for JObject<'_>  where T: Sized + Send + 'static {

    fn store(&self, env: &JNIEnv, rust_object: T) -> Result<()> {
        env.set_rust_field(*self, PTR_FIELD_NAME, rust_object)?;
        Ok(())
    }

    fn borrow<'a>(&'a self, env: &'a JNIEnv) -> Result<MutexGuard<'a,T>> {
        env.get_rust_field(*self, PTR_FIELD_NAME)
    }

    fn delete(&self, env: &JNIEnv) -> Result<()> {
        env.take_rust_field(*self, PTR_FIELD_NAME)?;
         Ok(())
    }
}
