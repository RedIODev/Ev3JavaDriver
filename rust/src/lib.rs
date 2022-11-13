use alloc::JNIStruct;
use ev3dev_lang_rust::{motors::{LargeMotor, MotorPort},Ev3Error };
use jni::{JNIEnv, objects::JObject, signature::JavaType};
//use robusta_jni::bridge;

mod alloc;
mod jni_bridge;

pub type JNIError = jni::errors::Error;
pub type Result<T> = core::result::Result<T, Ev3JApiError>;

pub fn test(mut env: JNIEnv, mut obj: JObject) -> Result<()>{
    let motor = LargeMotor::get(MotorPort::OutA)?;
    motor.store(&mut env, &mut obj)?;
    let motor = LargeMotor::borrow(&mut env, &mut obj)?;
    motor.set_duty_cycle_sp(10)?;
    drop(motor);
    LargeMotor::delete(&mut env, &mut obj)?;
    Ok(())
}

pub extern "system" fn Java_dev_redio_ev3dev_LargeMotor_new0(env: JNIEnv, this: JObject, args: JObject) {
    let port = env.get_field(args, "port", "dev/redio/ev3dev/MotorPort").unwrap().l().unwrap();
    let ordinal = get_enum_ordinal(&env, &port);
    let port = ordinal as MotorPort;
    
}


pub fn get_enum_ordinal(env: &JNIEnv, variant: &JObject) -> i32 {
    env.call_method(*variant, "ordinal", "()I", &[]).unwrap().i().unwrap()
}
// #[bridge]
// mod ev3 {
//     use std::marker::PhantomData;

//     use robusta_jni::{convert::{Signature, Field}, jni::objects::JObject};
//     use crate::JNIEnv;
//     use robusta_jni::convert::TryFromJavaValue;
    
//     #[derive(Signature,TryFromJavaValue)]
//     #[package(dev.redio.ev3dev)]
//     struct LargeMotor<'env: 'n,'borrow> {
//         #[instance]
//         phantom: PhantomData<&'env str>,
//         phantom2: PhantomData<&'borrow str>
//     }

//     // impl<'env,'borrow> LargeMotor<'env,'borrow> {
//     //     pub extern "jni" fn new0(self, env: &JNIEnv) {
            
//     //     }
//     // }

//     impl<'env: 'borrow,'borrow> LargeMotor<'env, 'borrow> {
//         pub extern "jni" fn op(self, _env: &JNIEnv, flag: bool) -> i32 {
//             //                       ^^^^^ optional
//             if flag {
//                 1
//             } else {
//                 0
//             }
//         }
//     }

//     // impl<'env : 'borrow,'borrow> TryFromJavaValue<'env, 'borrow> for LargeMotor {
        
//     // }
    
// }




// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub enum Ev3JApiError {
    Ev3(Ev3Error),
    JNI(JNIError)
}

impl From<Ev3Error> for Ev3JApiError {
    fn from(e: Ev3Error) -> Self {
        Ev3JApiError::Ev3(e)
    }
}

impl From<JNIError> for Ev3JApiError {
    fn from(e: JNIError) -> Self {
        Ev3JApiError::JNI(e)
    }
}
