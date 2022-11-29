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

        //use jni_proc_macro::optional_overload;
        use std::{time::Duration, borrow::Cow};

        use ev3dev_lang_rust::motors::{TachoMotor, LargeMotor, MediumMotor};
        use jni::{
            objects::{JClass, JObject, JString},
            sys::{jobjectArray, jlong},
            JNIEnv,
        };

        use crate::{
            alloc::RustObjectCarrier,
            enum_conversions::{Enum, JavaEnum, MotorState, StopAction, Polarity},
            errors::{Ev3JApiError, EnumConversionError},
            jni_shortcuts::{try_supplier, vec_to_jarray, wrap_obj, try_consumer, function, bi_function, condition_callback},
            result_extensions::{FlattenInto, MapAuto},
        };

        fn find<'a>(jre: JNIEnv<'a>, class: JClass<'a>) -> Result<JObject<'a>, Ev3JApiError> {
            let motor = TachoMotor::find()?;
            wrap_obj(jre, class, motor)
        }

        fn list(jre: JNIEnv, class: JClass) -> Result<jobjectArray, Ev3JApiError> {
            let motors = TachoMotor::list()?;
            vec_to_jarray(jre, motors, class, |jre, motor| {
                wrap_obj(jre, class, motor)
            })
        }

        fn new0(jre: JNIEnv, this: JObject, args: jobjectArray) -> Result<(), Ev3JApiError> {
            let port = jre
                .get_object_array_element(args, 0)
                .and_then(|o| o.ordinal(jre))
                .map(Enum::try_into)
                .flatten_into::<Ev3JApiError>()?;
            let motor = TachoMotor::get(port)?;
            this.store(jre, motor).map_auto()
        }

        fn delete0(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            this.take(jre)?;
            Ok(())
        }

        //NOT SUPPORTED
        // fn getDegreePerMeter(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
        //     try_supplier(jre, this, TachoMotor::get_count_per_m)
        // }

        fn getUnitsPerRotation(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_count_per_rot)
        }

        fn getLoad(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_duty_cycle)
        }

        fn getTargetLoad(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_duty_cycle_sp)
        }

        //NOT SUPPORTED
        // fn getFullTravelCount(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
        //     try_supplier(jre, this, TachoMotor::get_full_travel_count)
        // }

        fn getHoldPidKd(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_hold_pid_kd)
        }

        fn getHoldPidKi(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_hold_pid_ki)
        }

        fn getHoldPidKp(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_hold_pid_kp)
        }

        fn getMaxSpeed(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_max_speed)
        }

        fn getPolarity<'a>(
            jre: JNIEnv<'a>,
            this: JObject<'a>,
        ) -> Result<JObject<'a>, Ev3JApiError> {
            const ENUM_TYPE:&str = "dev/redio/ev3dev/Motor$Polarity";
            let enum_constants = JObject::values(jre, ENUM_TYPE)?;
            let string = try_supplier(jre, this, TachoMotor::get_polarity)?;
            let ordinal = match string.as_str() {
                TachoMotor::POLARITY_NORMAL => 0,
                TachoMotor::POLARITY_INVERSED => 1,
                _ => return Err(EnumConversionError.into())
            };
            jre.get_object_array_element(enum_constants, ordinal).map_err(Ev3JApiError::from)
        }

        fn getAbsolutePosition(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_position)
        }

        fn getTargetPosition(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_position_sp)
        }


        fn getSlowdownTime(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_ramp_down_sp)
        }

        fn getSpeedupTime(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_ramp_up_sp)
        }

        fn getSpeed(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_speed)
        }

        fn getSpeedPidKd(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_speed_pid_kd)
        }

        fn getSpeedPidKi(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_speed_pid_ki)
        }

        fn getSpeedPidKp(jre: JNIEnv, this: JObject) -> Result<f32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_speed_pid_kp)
        }

        fn getTargetSpeed(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_speed_sp)
        }

        fn getState(jre: JNIEnv, this: JObject) -> Result<jobjectArray, Ev3JApiError> {
            const ENUM_TYPE:&str = "dev/redio/ev3dev/Motor$State";
            let enum_constants = JObject::values(jre, ENUM_TYPE)?;
            let state = try_supplier(jre, this, TachoMotor::get_state)?;
            let match_func = |_, string: String| {
                let ordinal = match string.as_str() {
                    TachoMotor::STATE_HOLDING => 0,
                    TachoMotor::STATE_OVERLOADED => 1,
                    TachoMotor::STATE_RAMPING => 2,
                    TachoMotor::STATE_RUNNING => 3,
                    TachoMotor::STATE_STALLED => 4,
                    _ => return Err(EnumConversionError.into())
                };
                jre.get_object_array_element(enum_constants, ordinal).map_err(Ev3JApiError::from)
            };
            vec_to_jarray(jre, state, ENUM_TYPE, match_func)
        }

        fn getStopAction<'a>(
            jre: JNIEnv<'a>,
            this: JObject<'a>,
        ) -> Result<JObject<'a>, Ev3JApiError> {
            const ENUM_TYPE:&str = "dev/redio/ev3dev/Motor$StopAction";
            let enum_constants = JObject::values(jre, ENUM_TYPE)?;
            let string = try_supplier(jre, this, TachoMotor::get_stop_action)?;
            let ordinal = match string.as_str() {
                TachoMotor::STOP_ACTION_BRAKE => 0,
                TachoMotor::STOP_ACTION_COAST => 1,
                TachoMotor::STOP_ACTION_HOLD => 2,
                _ => return Err(EnumConversionError.into())
            };
            jre.get_object_array_element(enum_constants, ordinal).map_err(Ev3JApiError::from)
        }

        fn getSupportedStopActions(jre: JNIEnv, this: JObject) -> Result<jobjectArray, Ev3JApiError> {
            const ENUM_TYPE:&str = "dev/redio/ev3dev/Motor$StopAction";
            let actions = try_supplier(jre, this, TachoMotor::get_stop_actions)?;
            let enum_constants = JObject::values(jre, ENUM_TYPE)?;
            let match_func = |_, string: String| {
                let ordinal = match string.as_str() {
                    TachoMotor::STOP_ACTION_BRAKE => 0,
                    TachoMotor::STOP_ACTION_COAST => 1,
                    TachoMotor::STOP_ACTION_HOLD => 2,
                    _ => return Err(EnumConversionError.into())
                };
                jre.get_object_array_element(enum_constants, ordinal).map_err(Ev3JApiError::from)
            };
            vec_to_jarray(jre, actions, ENUM_TYPE, match_func)
        }

        fn getTargetDuration(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::get_time_sp)
        }

        fn isHolding(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::is_holding)
        }

        fn isOverloaded(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::is_overloaded)
        }

        fn isRamping(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::is_ramping)
        }

        fn isRunning(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::is_running)
        }

        fn isStalled(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::is_stalled)
        }

        fn reset(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::reset)
        }

        fn rotateLoad(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::run_direct)
        }

        fn rotate(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::run_forever)
        }

        fn rotateUntil__(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, None, TachoMotor::run_timed)
        }

        fn rotateUntil__J(jre: JNIEnv, this: JObject, mills: i64) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, Some(Duration::from_millis(mills as u64)), TachoMotor::run_timed)
        }

        fn rotateAbsolute__(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, None, TachoMotor::run_to_abs_pos)
        }

        fn rotateAbsolute__I(jre: JNIEnv, this: JObject, pos: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, Some(pos), TachoMotor::run_to_abs_pos)
        }

        fn rotateRelative__(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, None, TachoMotor::run_to_rel_pos)
        }

        fn rotateRelative__I(jre: JNIEnv, this: JObject, pos: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, Some(pos), TachoMotor::run_to_rel_pos)
        }

        fn setTargetLoad(jre: JNIEnv, this: JObject, duty_cycle: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, duty_cycle, TachoMotor::set_duty_cycle_sp)
        }

        fn setHoldPidKd(jre: JNIEnv, this: JObject, kd: f32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, kd, TachoMotor::set_hold_pid_kd)
        }

        fn setHoldPidKi(jre: JNIEnv, this: JObject, ki: f32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, ki, TachoMotor::set_hold_pid_ki)
        }

        fn setHoldPidKp(jre: JNIEnv, this: JObject, kp: f32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, kp, TachoMotor::set_hold_pid_kp)
        }

        fn setPolarity(jre: JNIEnv, this: JObject, polarity: i32) -> Result<(), Ev3JApiError> {
            let polarity:Enum<Polarity> = polarity.try_into()?;
            let polarity = polarity.0.value();
            try_consumer(jre, this, polarity, TachoMotor::set_polarity)
        }

        fn setPosition(jre: JNIEnv, this: JObject, pos: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, pos, TachoMotor::set_position)
        }

        fn setTargetPosition(jre: JNIEnv, this: JObject, pos: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, pos, TachoMotor::set_position_sp)
        }

        fn setSlowdownTime(jre: JNIEnv, this: JObject, sp: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, sp, TachoMotor::set_ramp_down_sp)
        }

        fn setRampUpSetPoint(jre: JNIEnv, this: JObject, sp: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, sp, TachoMotor::set_ramp_up_sp)
        }

        fn setSpeedPidKd(jre: JNIEnv, this: JObject, kd: f32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, kd, TachoMotor::set_speed_pid_kd)
        }

        fn setSpeedPidKi(jre: JNIEnv, this: JObject, ki: f32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, ki, TachoMotor::set_speed_pid_ki)
        }

        fn setSpeedPidKp(jre: JNIEnv, this: JObject, kp: f32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, kp, TachoMotor::set_speed_pid_kp)
        }

        fn setTargetSpeed(jre: JNIEnv, this: JObject, sp: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, sp, TachoMotor::set_speed_sp)
        }

        fn setStopAction(jre: JNIEnv, this: JObject, stopAction: i32) -> Result<(), Ev3JApiError> {
            let action:Enum<StopAction> = stopAction.try_into()?;
            let action = action.0.value();
            try_consumer(jre, this, action, TachoMotor::set_stop_action)
        }

        fn setTargetDuration(jre: JNIEnv, this: JObject, mills: i32) -> Result<(), Ev3JApiError> {
            try_consumer(jre, this, mills, TachoMotor::set_time_sp)
        }

        fn stop(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            try_supplier(jre, this, TachoMotor::stop)
        }

        fn isLarge(jre: JNIEnv, this: JObject) -> Result<bool, Ev3JApiError> {
            let motor = this.borrow::<TachoMotor>(&jre)?;
            Ok(motor.clone().into_large_motor().is_ok())
        }

        fn sleepUntilNotMoving__(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            function(jre, this, None, TachoMotor::wait_until_not_moving)?;
            Ok(())
        }

        fn sleepUntilNotMoving__J(jre: JNIEnv, this: JObject, mills: i64) -> Result<bool, Ev3JApiError> {
            function(jre, this, Some(Duration::from_millis(mills as u64)), TachoMotor::wait_until_not_moving)
        }

        fn sleepUntil__Ldev_redio_ev3dev_Motor_State_20(jre: JNIEnv, this: JObject, state: i32) -> Result<(), Ev3JApiError> {
            let state:Enum<MotorState> = state.try_into()?;
            let state = state.0.value();
            bi_function(jre, this, state, None, TachoMotor::wait_until)?;
            Ok(())
        }

        fn sleepUntil__Ldev_redio_ev3dev_Motor_State_2J0(jre: JNIEnv, this: JObject, state: i32, mills: i64) -> Result<bool, Ev3JApiError> {
            let state:Enum<MotorState> = state.try_into()?;
            let state = state.0.value();
            bi_function(jre, this, state, Some(Duration::from_millis(mills as u64)), TachoMotor::wait_until)
        }

        //TODO: Create enums for motor states
        fn sleep_while__Ldev_redio_ev3dev_Motor_State_20(jre: JNIEnv, this: JObject, state: i32) -> Result<(), Ev3JApiError> {
            let state:Enum<MotorState> = state.try_into()?;
            let state = state.0.value();
            bi_function(jre, this, state, None, TachoMotor::wait_while)?;
            Ok(())
        }

        fn sleepWhile__Ldev_redio_ev3dev_Motor_State_2J0(jre: JNIEnv, this: JObject, state: i32, mills: i64) -> Result<bool, Ev3JApiError> {
            let state:Enum<MotorState> = state.try_into()?;
            let state = state.0.value();
            bi_function(jre, this, state, Some(Duration::from_millis(mills as u64)), TachoMotor::wait_while)
        }

        fn sleep__Ldev_redio_ev3dev_Condition_2(jre: JNIEnv, this: JObject, f: JObject) -> Result<(), Ev3JApiError> {
            let f = condition_callback(jre, f);
            bi_function(jre, this, f, None, TachoMotor::wait)?;
            Ok(())
        }

        fn sleep__Ldev_redio_ev3dev_Condition_2J(jre: JNIEnv, this: JObject, f: JObject, mills: i64) -> Result<bool, Ev3JApiError> {
            let f = condition_callback(jre, f);
            bi_function(jre, this, f, Some(Duration::from_millis(mills as u64)), TachoMotor::wait)
        }

        // #[optional_overload("java.util.function.BooleanSupplier", "long")]
        // fn sleep(jre: JNIEnv, this: JObject, f: JObject, mills: Option<i64>) -> Result<bool, Ev3JApiError> {
        //     let f = boolean_supplier_callback(&jre, &f);
        //     let mills = mills.map(|int| int as u64);
        //     bi_function(&jre, &this, f, mills.map(Duration::from_millis), TachoMotor::wait)
        // }

    }

    #[jni_class("ColorSensor")]
    pub mod color_sensor {

        use std::{time::Duration, borrow::Cow, cell::Ref};

        use ev3dev_lang_rust::{motors::TachoMotor, sensors::ColorSensor, wait};
        use jni::{
            objects::{JClass, JObject, JString},
            sys::{jobjectArray, jlong},
            JNIEnv,
        };

        use crate::{
            alloc::RustObjectCarrier,
            enum_conversions::{Enum, ColorMode},
            errors::{Ev3JApiError, EnumConversionError},
            jni_shortcuts::{try_supplier, vec_to_jarray, wrap_obj, try_consumer, get_int},
            result_extensions::{FlattenInto, MapAuto},
        };

        fn find<'a>(jre: JNIEnv<'a>, class: JClass<'a>) -> Result<JObject<'a>, Ev3JApiError> {
            let sensor = ColorSensor::find()?;
            wrap_obj(jre, class, sensor)
        }

        fn list(jre: JNIEnv, class: JClass) -> Result<jobjectArray, Ev3JApiError> {
            let sensors = ColorSensor::list()?;
            vec_to_jarray(jre, sensors, class, |jre, sensor| {
                wrap_obj(jre, class, sensor)
            })
        }

        fn new0(jre: JNIEnv, this: JObject, args: jobjectArray) -> Result<(), Ev3JApiError> {
            let port = jre
                .get_object_array_element(args, 0)
                .and_then(|o| get_int(jre, o))
                .map(Enum::try_from)
                .flatten_into::<Ev3JApiError>()?;
            let sensor = ColorSensor::get(port.0)?;
            this.store(jre, sensor).map_auto()
        }

        fn delete0(jre: JNIEnv, this: JObject) -> Result<(), Ev3JApiError> {
            this.take(jre)?;
            Ok(())
        }

        fn value2(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {    
            try_supplier(jre, this, ColorSensor::get_blue)
        }

        // fn getIntensity(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {    
        //     try_supplier(jre, this, ColorSensor::get_color)
        // }

        fn value1(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {    
            try_supplier(jre, this, ColorSensor::get_green)
        }

        fn value0(jre: JNIEnv, this: JObject) -> Result<i32, Ev3JApiError> {    
            try_supplier(jre, this, ColorSensor::get_red)
        }

        // fn getRGB<'a>(jre: JNIEnv<'a>, this: JObject<'a>) -> Result<JObject<'a>, Ev3JApiError> {    
        //     let (r,g,b) = try_supplier(jre, this, ColorSensor::get_rgb)?;
        //     new_color(jre, r, g, b)
        // }

        fn setMode0(jre: JNIEnv, this: JObject, mode: i32) -> Result<(), Ev3JApiError> {
            use ColorMode::*;
            let mode: Enum<ColorMode> = mode.try_into()?;
            match mode.0 {
                SimpleColor => try_supplier(jre, this, ColorSensor::set_mode_col_color),
                Color => try_supplier(jre, this, ColorSensor::set_mode_rgb_raw),
                Reflect => try_supplier(jre, this, ColorSensor::set_mode_col_reflect),
                RawReflect => try_supplier(jre, this, ColorSensor::set_mode_ref_raw),
                Ambient => try_supplier(jre, this, ColorSensor::set_mode_col_ambient)
            }
        }

        fn getMode0<'a>(jre: JNIEnv<'a>, this: JObject<'a>) -> Result<i32, Ev3JApiError> {
            //let values = JObject::values(jre, "dev/redio/ev3dev/ColorSensor$Mode")?;
            let index = if try_supplier(jre, this, ColorSensor::is_mode_col_color)? {
                0
            } else if try_supplier(jre, this, ColorSensor::is_mode_rgb_raw)? {
                1
            } else if try_supplier(jre, this, ColorSensor::is_mode_col_reflect)? {
                2
            } else if try_supplier(jre, this, ColorSensor::is_mode_ref_raw)? {
                3
            } else if try_supplier(jre, this, ColorSensor::is_mode_col_ambient)? {
                4
            } else {
                return Err(EnumConversionError.into());
            };
            Ok(index)
        }


    }
}
