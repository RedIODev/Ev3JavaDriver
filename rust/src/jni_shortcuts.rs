use std::borrow::Cow;

use jni::{
    descriptors::Desc,
    objects::{JClass, JObject, JString},
    sys::jobjectArray,
    JNIEnv,
};

use crate::{alloc::RustObjectCarrier, errors::Ev3JApiError};

pub fn try_supplier<R, T, E, F>(jre: JNIEnv, this: JObject, f: F) -> Result<R, Ev3JApiError>
where
    F: FnOnce(&T) -> Result<R, E>,
    E: Into<Ev3JApiError>,
    T: Send + 'static,
{
    let rust_struct = this.borrow::<T>(&jre)?;
    f(&rust_struct).map_err(E::into)
}

pub fn try_consumer<I, T, E, F>(jre: JNIEnv, this: JObject, input: I, f: F) -> Result<(), Ev3JApiError>
where
    F: FnOnce(&T, I) -> Result<(), E>,
    E: Into<Ev3JApiError>,
    T: Send + 'static,
{
    let rust_struct = this.borrow::<T>(&jre)?;
    f(&rust_struct, input).map_err(E::into)
}

pub fn function<I, R, T, F>(jre: JNIEnv, this: JObject, input: I, f: F) -> Result<R, Ev3JApiError> 
where 
    F: FnOnce(&T, I) -> R,
    T: Send + 'static
{
    let rust_struct = this.borrow::<T>(&jre)?;
    Ok(f(&rust_struct, input))
}

pub fn bi_function<I, I2, R, T, F>(jre: JNIEnv, this: JObject, input: I, input2: I2, f: F) -> Result<R, Ev3JApiError> 
where
    F: FnOnce(&T, I, I2) -> R,
    T: Send + 'static
{
    let rust_struct = this.borrow::<T>(&jre)?;
    Ok(f(&rust_struct, input, input2))
}

pub fn vec_to_jarray<'a, T, A, O, E, F>(
    jre: JNIEnv<'a>,
    vec: Vec<T>,
    array_type: A,
    f: F,
) -> Result<jobjectArray, Ev3JApiError>
where
    A: Desc<'a, JClass<'a>>,
    F: Fn(JNIEnv<'a>, T) -> Result<O, E>,
    O: Into<JObject<'a>>,
    Ev3JApiError: From<E>,
{
    let array = jre.new_object_array(vec.len() as i32, array_type, JObject::null())?;
    let len = vec.len();
    let mut iter = vec.into_iter();
    for i in 0..len {
        let val = f(jre, iter.next().unwrap())?;
        jre.set_object_array_element(array, i as i32, val)?;
    }
    Ok(array)
}

pub fn wrap_obj<'a, T>(jre: JNIEnv<'a>, class: JClass, val: T) -> Result<JObject<'a>, Ev3JApiError> 
where T: Send + 'static
{
    let jobj = jre.new_object(class, "()V", &[])?;
    jobj.store(jre, val)?;
    Ok(jobj)
} 

pub fn new_color(jre: JNIEnv, red: i32, green: i32, blue: i32) -> Result<JObject, Ev3JApiError> {
    let red = jni::objects::JValue::Int(red);
    let green = jni::objects::JValue::Int(green);
    let blue = jni::objects::JValue::Int(blue);
    let jobj = jre.new_object("dev/redio/ev3dev/Color", "(III)V", &[red, green, blue])?;
    Ok(jobj)
}

pub fn new_reflection(jre: JNIEnv, x: i32, y: i32) -> Result<JObject, Ev3JApiError> {
    let x = jni::objects::JValue::Int(x);
    let y = jni::objects::JValue::Int(y);
    let jobj = jre.new_object("dev/redio/ev3dev/Reflection", "(II)V", &[x,y])?;
    Ok(jobj)
}


pub fn condition_callback<'a>(jre: JNIEnv<'a>, f: JObject<'a>) -> impl Fn() -> bool + 'a {
    move || {
        let result = jre.call_method(f, "condition", "()Z", &[]);
        match result {
            Ok(jni::objects::JValue::Bool(b)) => b != 0,
            Ok(_) => panic!("Invalid type returned by java function."),
            Err(err) => {
                let err = Ev3JApiError::from(err);
                let _ = jre.throw(err);
                false
            }
        }
    }
}

pub fn getClassSpecifier(jre: JNIEnv, class: JClass) -> Result<String, Ev3JApiError> {
    let java_name:JString = jre.call_method(class, "getName", "()Ljava/lang/String;", &[])?.l()?.into();
    let jstr = &jre.get_string(java_name)?;
    let str_name = &*Into::<Cow<str>>::into(jstr);
    let mut class_name = str_name.replace('.', "/");
    class_name.insert(0, 'L');
    class_name.push(';');
    Ok(class_name)
}