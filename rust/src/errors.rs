use std::str::Utf8Error;

use ev3dev_lang_rust::Ev3Error;
use jni::{objects::JThrowable, JNIEnv, descriptors::Desc};

pub type JNIError = jni::errors::Error;
//pub type Ev3JApiResult<T> = core::result::Result<T, Ev3JApiError>;

#[derive(Debug)]
pub struct EnumConversionError;

pub trait IntoJThrowable<'a, 'e> {
    fn throwable(&self, env: &JNIEnv<'a>) -> jni::errors::Result<JThrowable<'e>>;
}

impl<'a:'e,'e> IntoJThrowable<'a, 'e> for Ev3Error {
    fn throwable(&self, env: &JNIEnv<'a>) -> jni::errors::Result<JThrowable<'e>> {
        let msg = env.new_string(self.to_string())?;
        let ex = env.new_object(
            "dev/redio/ev3dev/exceptions/Ev3Exception",
            "(Ljava/lang/String;)V",
            &[msg.into()],
        )?;
        Ok(JThrowable::from(ex))
    }
}

impl<'a:'e,'e> IntoJThrowable<'a,'e> for EnumConversionError {
    fn throwable(&self, env: & JNIEnv<'a>) -> jni::errors::Result<JThrowable<'e>> {
        let msg = env.new_string(format!("{:?}", self))?;
        let ex = env.new_object(
            "dev/redio/ev3dev/exceptions/EnumConversionException",
            "(Ljava/lang/String;)V",
            &[msg.into()],
        )?;
        Ok(JThrowable::from(ex))
    }
}

impl<'a:'e,'e> IntoJThrowable<'a,'e> for Utf8Error {
    fn throwable(&self, env: & JNIEnv<'a>) -> jni::errors::Result<JThrowable<'e>> {
        let msg = env.new_string(format!("{:?}", self))?;
        let ex = env.new_object(
            "dev/redio/ev3dev/exceptions/StringFormatException",
            "(Ljava/lang/String;)V",
            &[msg.into()],
        )?;
        Ok(JThrowable::from(ex))
    }
}



#[derive(Debug)]
pub enum Ev3JApiError {
    Ev3(Ev3Error),
    Jni(JNIError),
    EnumConversion(EnumConversionError),
    StringFormat(Utf8Error)
}

impl<'a : 'e, 'e> Desc<'a, JThrowable<'e>> for Ev3JApiError {
    fn lookup(self, env: &JNIEnv<'a>) -> jni::errors::Result<JThrowable<'e>> {
        use crate::errors::Ev3JApiError::*;
        match self {
            Ev3(e) => e.throwable(env),
            Jni(e) => panic!("JNI:{}", e),
            EnumConversion(e) => e.throwable(env),
            StringFormat(e) => e.throwable(env),
        }
    }
}

impl From<Ev3Error> for Ev3JApiError {
    fn from(e: Ev3Error) -> Self {
        Ev3JApiError::Ev3(e)
    }
}

impl From<JNIError> for Ev3JApiError {
    fn from(e: JNIError) -> Self {
        Ev3JApiError::Jni(e)
    }
}

impl From<EnumConversionError> for Ev3JApiError {
    fn from(e: EnumConversionError) -> Self {
        Ev3JApiError::EnumConversion(e)
    }
}

impl From<Utf8Error> for Ev3JApiError {
    fn from(e: Utf8Error) -> Self {
        Ev3JApiError::StringFormat(e)
    }
}

