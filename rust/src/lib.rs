#![allow(non_snake_case)]
#![allow(unused)]
use jni_proc_macro::jni_package;
mod alloc;
mod enum_conversions;
mod errors;
mod jni_shortcuts;
mod result_extensions;
//mod test;

#[jni_package("dev.redio.ev3dev")]
pub mod dev_redio_ev3dev {
    use jni_proc_macro::jni_class;

    #[jni_class("Motor")]
    pub mod large_motor {
        use std::{time::Duration, borrow::Cow};

        use ev3dev_lang_rust::motors::{TachoMotor, LargeMotor, MediumMotor};
        use jni::{
            objects::{JClass, JObject, JString},
            sys::{jobjectArray, jlong},
            JNIEnv,
        };

        use crate::{
            alloc::RustObjectCarrier,
            enum_conversions::{IntEnum, JavaEnum},
            errors::Ev3JApiError,
            jni_shortcuts::{supplier, vec_to_jarray, wrap_obj, consumer},
            result_extensions::{FlattenInto, MapAuto},
        };

        fn find<'a>(jre: JNIEnv<'a>, class: JClass<'a>) -> Result<JObject<'a>, Ev3JApiError> {
            let motor = TachoMotor::find()?;
            wrap_obj(&jre, class, motor)
        }

        fn list(jre: JNIEnv, class: JClass) -> Result<jobjectArray, Ev3JApiError> {
            let motors = TachoMotor::list()?;
            vec_to_jarray(&jre, motors, class, |jre, motor| {
                wrap_obj(&jre, class.clone(), motor)
            })
        }

        fn new0(jre: JNIEnv, this: JObject, args: jobjectArray) -> Result<(), Ev3JApiError> {
            let port = jre
                .get_object_array_element(args, 0)
                .and_then(|o| o.ordinal(&jre))
                .map(IntEnum::try_into)
                .flatten_into::<Ev3JApiError>()?;
            let motor = TachoMotor::get(port)?;
            this.store(&jre, motor).map_auto()
        }

        fn delete0(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            let _ = this.take(&jre)?;
            Ok(())
        }

        fn getCountPerMeter(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_count_per_m)
        }

        fn getCountPerRotation(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_count_per_rot)
        }

        fn getDutyCycle(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_duty_cycle)
        }

        fn getDutyCycleSetpoint(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_duty_cycle_sp)
        }

        fn getFullTravelCount(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_full_travel_count)
        }

        fn getHoldPidKd(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_hold_pid_kd)
        }

        fn getHoldPidKi(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_hold_pid_ki)
        }

        fn getHoldPidKp(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_hold_pid_kp)
        }

        fn getMaxSpeed(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_max_speed)
        }

        fn getPolarity<'a>(
            jre: JNIEnv<'a>,
            this: JObject<'a>,
        ) -> Result<JString<'a>, Ev3JApiError> {
            let str = supplier(&jre, &this, TachoMotor::get_polarity)?;
            jre.new_string(str).map_err(Ev3JApiError::from)
        }

        fn getPosition(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_position)
        }

        fn getRampDownSetpoint(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_ramp_down_sp)
        }

        fn getRampUpSetpoint(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_ramp_up_sp)
        }

        fn getSpeed(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_speed)
        }

        fn getSpeedPidKd(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_speed_pid_kd)
        }

        fn getSpeedPidKi(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_speed_pid_ki)
        }

        fn getSpeedPidKp(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_speed_pid_kp)
        }

        fn getSpeedPidSetpoint(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_speed_sp)
        }

        fn getState(jre: JNIEnv, this: JObject) -> Result<jobjectArray, Ev3JApiError> {
            let state = supplier(&jre, &this, TachoMotor::get_state)?;
            vec_to_jarray(&jre, state, "java/lang/String", JNIEnv::new_string)
        }

        fn getStopAction<'a>(
            jre: JNIEnv<'a>,
            this: JObject<'a>,
        ) -> Result<JString<'a>, Ev3JApiError> {
            let str = supplier(&jre, &this, TachoMotor::get_stop_action)?;
            jre.new_string(str).map_err(Ev3JApiError::from)
        }

        fn getStopActions(jre: JNIEnv, this: JObject) -> Result<jobjectArray, Ev3JApiError> {
            let actions = supplier(&jre, &this, TachoMotor::get_stop_actions)?;
            vec_to_jarray(&jre, actions, "java/lang/String", JNIEnv::new_string)
        }

        fn getTimeSetpoint(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::get_time_sp)
        }

        fn isHolding(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::is_holding)
        }

        fn isOverloaded(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::is_overloaded)
        }

        fn isRamping(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::is_ramping)
        }

        fn isRunning(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::is_running)
        }

        fn isStalled(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::is_stalled)
        }

        fn reset(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::reset)
        }

        fn runDirect(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::run_direct)
        }

        fn runForever(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::run_forever)
        }

        fn runTimed__(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, None, TachoMotor::run_timed)
        }

        fn runTimed__J(jre: JNIEnv, this: JObject, mills: i64) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, Some(Duration::from_millis(mills as u64)), TachoMotor::run_timed)
        }

        fn runToAbsolutePosition__(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, None, TachoMotor::run_to_abs_pos)
        }

        fn runToAbsolutePosition__I(jre: JNIEnv, this: JObject, pos: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, Some(pos), TachoMotor::run_to_abs_pos)
        }

        fn runToRelativePosition__(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            // let motor = this.borrow::<TachoMotor>(&jre)?;
            // println!("Got motor");
            // motor.
            // motor.run_to_rel_pos(None)?;
            // Ok(())
            consumer(&jre, &this, None, TachoMotor::run_to_rel_pos)
        }

        fn runToRelativePosition__I(jre: JNIEnv, this: JObject, pos: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, Some(pos), TachoMotor::run_to_rel_pos)
        }

        fn setDutyCycleSetpoint(jre: JNIEnv, this: JObject, duty_cycle: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, duty_cycle, TachoMotor::set_duty_cycle_sp)
        }

        fn setHoldPidKd(jre: JNIEnv, this: JObject, kd: f32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, kd, TachoMotor::set_hold_pid_kd)
        }

        fn setHoldPidKi(jre: JNIEnv, this: JObject, ki: f32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, ki, TachoMotor::set_hold_pid_ki)
        }

        fn setHoldPidKp(jre: JNIEnv, this: JObject, kp: f32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, kp, TachoMotor::set_hold_pid_kp)
        }

        fn setPolarity(jre: JNIEnv, this: JObject, polarity: JString) -> Result<(), Ev3JApiError> {
            let jstr = &jre.get_string(polarity)?;
            let str = &*Into::<Cow<str>>::into(jstr);
            consumer(&jre, &this, str, TachoMotor::set_polarity)
        }

        fn setPosition(jre: JNIEnv, this: JObject, pos: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, pos, TachoMotor::set_position)
        }

        fn setPositionSetpoint(jre: JNIEnv, this: JObject, pos: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, pos, TachoMotor::set_position_sp)
        }

        fn setRampDownSetPoint(jre: JNIEnv, this: JObject, sp: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, sp, TachoMotor::set_ramp_down_sp)
        }

        fn setRampUpSetPoint(jre: JNIEnv, this: JObject, sp: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, sp, TachoMotor::set_ramp_up_sp)
        }

        fn setSpeedPidKd(jre: JNIEnv, this: JObject, kd: f32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, kd, TachoMotor::set_speed_pid_kd)
        }

        fn setSpeedPidKi(jre: JNIEnv, this: JObject, ki: f32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, ki, TachoMotor::set_speed_pid_ki)
        }

        fn setSpeedPidKp(jre: JNIEnv, this: JObject, kp: f32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, kp, TachoMotor::set_speed_pid_kp)
        }

        fn setSpeedSetPoint(jre: JNIEnv, this: JObject, sp: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, sp, TachoMotor::set_speed_sp)
        }

        fn setStopAction(jre: JNIEnv, this: JObject, stopAction: JString) -> Result<(), Ev3JApiError> {
            let jstr = &jre.get_string(stopAction)?;
            let str = &*Into::<Cow<str>>::into(jstr);
            consumer(&jre, &this, str, TachoMotor::set_stop_action)
        }

        fn setTimeSetPoint(jre: JNIEnv, this: JObject, millis: i32) -> Result<(), Ev3JApiError> {
            consumer(&jre, &this, millis, TachoMotor::set_time_sp)
        }

        fn stop(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            supplier(&jre, &this, TachoMotor::stop)
        }

        fn isLarge(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            let motor = this.borrow::<TachoMotor>(&jre)?;
            Ok(motor.clone().into_large_motor().is_ok())
        }

        fn waitUntilNotMoving(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            let motor = this.borrow::<TachoMotor>(&jre)?;
            match motor.clone().into_large_motor() {
                Ok(lm) => {lm.wait_until_not_moving(None);},
                Err(m) => match m.into_medium_motor() {
                    Ok(mm) => {mm.wait_until_not_moving(None);},
                    Err(_) => {
                        println!("Error motor not recognized");
                    }
                }
            }
            Ok(())
        }

    }

    #[jni_class("ColorSensor")]
    pub mod color_sensor {

        use std::{time::Duration, borrow::Cow};

        use ev3dev_lang_rust::{motors::TachoMotor, sensors::ColorSensor};
        use jni::{
            objects::{JClass, JObject, JString},
            sys::{jobjectArray, jlong},
            JNIEnv,
        };

        use crate::{
            alloc::RustObjectCarrier,
            enum_conversions::{IntEnum, JavaEnum},
            errors::Ev3JApiError,
            jni_shortcuts::{supplier, vec_to_jarray, wrap_obj, consumer,new_color},
            result_extensions::{FlattenInto, MapAuto},
        };

        fn find<'a>(jre: JNIEnv<'a>, class: JClass<'a>) -> Result<JObject<'a>, Ev3JApiError> {
            let sensor = ColorSensor::find()?;
            wrap_obj(&jre, class, sensor)
        }

        fn list(jre: JNIEnv, class: JClass) -> Result<jobjectArray, Ev3JApiError> {
            let sensors = ColorSensor::list()?;
            vec_to_jarray(&jre, sensors, class, |jre, sensor| {
                wrap_obj(&jre, class.clone(), sensor)
            })
        }

        fn new0(jre: JNIEnv, this: JObject, args: jobjectArray) -> Result<(), Ev3JApiError> {
            let port = jre
                .get_object_array_element(args, 0)
                .and_then(|o| o.ordinal(&jre))
                .map(IntEnum::try_into)
                .flatten_into::<Ev3JApiError>()?;
            let sensor = ColorSensor::get(port)?;
            this.store(&jre, sensor).map_auto()
        }

        fn delete0(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            let _ = this.take(&jre)?;
            Ok(())
        }

        fn getBlue(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {    
            supplier(&jre, &this, ColorSensor::get_blue)
        }

        fn getColor(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {    
            supplier(&jre, &this, ColorSensor::get_color)
        }

        fn getGreen(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {    
            supplier(&jre, &this, ColorSensor::get_green)
        }

        fn getRed(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {    
            supplier(&jre, &this, ColorSensor::get_red)
        }

        fn getRGB<'a>(jre: JNIEnv<'a>, this: JObject<'a>) -> Result<JObject<'a>, Ev3JApiError> {    
            let (r,g,b) = supplier(&jre, &this, ColorSensor::get_rgb)?;
            new_color(&jre, r, g, b)
        }

        fn setWhite(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {    //combine with others
            supplier(&jre, &this, ColorSensor::set_mode_col_color)
        }

        fn setRGBRaw(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {    //combine with others
            supplier(&jre, &this, ColorSensor::set_mode_rgb_raw)
        }

        
    }
}
