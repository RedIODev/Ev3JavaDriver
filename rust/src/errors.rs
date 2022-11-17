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



#[derive(Debug)]
pub enum Ev3JApiError {
    Ev3(Ev3Error),
    Jni(JNIError),
    EnumConversion(EnumConversionError),
}

impl<'a : 'e, 'e> Desc<'a, JThrowable<'e>> for Ev3JApiError {
    fn lookup(self, env: &JNIEnv<'a>) -> jni::errors::Result<JThrowable<'e>> {
        match self {
            Ev3JApiError::Ev3(e) => e.throwable(env),
            Ev3JApiError::Jni(e) => panic!("{}", e.to_string()),
            Ev3JApiError::EnumConversion(e) => e.throwable(env),
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