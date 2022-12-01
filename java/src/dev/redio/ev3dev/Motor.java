package dev.redio.ev3dev;

import dev.redio.ev3dev.alloc.Native;
import dev.redio.ev3dev.alloc.NativeMethod;
import dev.redio.ev3dev.exceptions.Ev3Exception;
/**
 * The class representing a LEGO Mindstorm Ev3 Motor or LargeMotor.<p>
 * There are {NUMBER} rotate* methods that rotate the motor with different conditions:
 * <h2>RotateLoad</h2>
 * //TODO: Finish Motor Doc
 * //TODO: Add impicit mode change for ColorSensor when certain methods are called.
 * @apiNote Units are called Tach counts in other documentations.
 *          Load is called Duty cycles in other documentations.
 * @author RedIODev
 */
public final class Motor extends Native {

    /**
     * Finds <b>the single</b> Motor connected to the Ev3 brick.
     * @return the single connected motor
     * @throws Ev3Exception when no or more than 1 motor was found
     */
    public static native Motor find() throws Ev3Exception;

    /**
     * Finds all Motors connected to the Ev3 brick.
     * @return a list of all motors connected
     * @throws Ev3Exception when something went wrong retrieving the list of sensors
     */
    public static native Motor[] list() throws Ev3Exception;

     /**
     * Connects to the Motor on the specified {@link MotorPort}.
     * @param port the port the motor is connected to
     * @throws Ev3Exception when no motor could be found on the specified port
     */
    public Motor(MotorPort port) throws Ev3Exception {
        super(port);
    }

    @NativeMethod
    private Motor() {}

    @Override
    protected native void new0(Object... args) throws Ev3Exception;

    @Override
    protected native void delete0();


    /**
     * Returns the number of Units per rotation of the motor.<p>
     * These units are used for speed and position of the motor.
     * @return the number of Units in 1 full rotation of the motor
     * @throws Ev3Exception when {NOT DOCUMENTED}
     * @apiNote this method usually returns 360 Units for Ev3 motors.
     * This means that you can treat Units as Degrees.
     */
    public native int getUnitsPerRotation() throws Ev3Exception;

    /**
     * Returns the current load on the motor in % (-100 to +100).<p>
     * This load refers to actual used strength of the motor.<p>
     * If you for example assist the motor by turning it manually the load will reduce.<p>
     * If you turn the motor fast enough or the speed is set to 0 this value will be negative (Motor is braking)<p>
     * This value will be constant using {@link Motor#rotateLoad}.
     * @return the strength used by the motor.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getLoad() throws Ev3Exception;

    /**
     * Returns the targeted load for the motor.<p>
     * This value is only used by {@link Motor#rotateLoad} it will be ignored otherwise.
     * @return the target strength for the motor.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getTargetLoad() throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * The derivative constant for the position PID.
     * @return the derivative constant for the position PID
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native float getHoldPidKd() throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * The integral constant for the position PID.
     * @return the integral constant for the position PID
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native float getHoldPidKi() throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * The proportional constant for the position PID.
     * @return the proportional constant for the position PID
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native float getHoldPidKp() throws Ev3Exception;

    /**
     * Returns the maximum speed the motor can theoretically achieve in Units per second.
     * @return the max speed in Units per second
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getMaxSpeed() throws Ev3Exception;

    /**
     * Returns the polarity of the motor.
     * This value determines whether the motor runs in reverse.
     * @return the motor's polarity.
     * @throws Ev3Exception when an unexpected value is returned by the native library.
     */
    public native Polarity getPolarity() throws Ev3Exception;

    /**
     * Returns the absolute position in Units.<p>
     * This position ranges from {@link Integer#MIN_VALUE} to {@link Integer#MAX_VALUE} and then wraps back around.
     * @return the motors absolute position.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getAbsolutePosition() throws Ev3Exception;

    /**
     * Returns the currently set Target Position for the motor.<p>
     * This is both the absolute value for {@link Motor#rotateAbsolute} and the relative value for {@link Motor#rotateRelative}.
     * @return the current position
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getTargetPosition() throws Ev3Exception;

    /**
     * Returns the time the motor will take to slow down to standstill in milliseconds.<p>
     * This speed can be set using the {@link Motor#setSlowdownTime} method.
     * The actual slowdown can vary based on speed and outside side effects.
     * @return the time to stop in milliseconds
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getSlowdownTime() throws Ev3Exception;
    
    /**
     * Returns the time the motor will take to speed up from standstill to set speed in milliseconds.<p>
     * This speed can be set using the {@link Motor#setSpeedupTime} method.
     * The actual speedup can vary based on target speed and outside side effects.
     * @return the time to speed up in milliseconds
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getSpeedupTime() throws Ev3Exception;

    /**
     * Returns the current speed of the motor in Units per second.
     * @return the speed of the motor
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getSpeed() throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Returns the derivative pub constant for the speed regulation PID.
     * @return the derivative pub constant for the speed regulation PID
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native float getSpeedPidKd() throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Returns the integral pub constant for the speed regulation PID.
     * @return the integral pub constant for the speed regulation PID
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native float getSpeedPidKi() throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Returns the proportional pub constant for the speed regulation PID.
     * @return the proportional pub constant for the speed regulation PID
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native float getSpeedPidKp() throws Ev3Exception;

    /**
     * Returns the target speed set for the motor.<p>
     * The maximum possible speed is returned by the {@link Motor#getMaxSpeed} method.
     * @return the targeted speed
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getTargetSpeed() throws Ev3Exception;

    /**
     * Returns the current state of the motor represented by a set of state enum constants.
     * @return the state of the motor
     * @throws Ev3Exception when an unexpected value is returned by the native library.
     */
    public native MotorState[] getState() throws Ev3Exception;

    /**
     * Returns the current StopAction active for the motor.<p>
     * This value determents the action taken when the {@link Motor#stop} method is called.
     * @return the current StopAction
     * @throws Ev3Exception when an unexpected value is returned by the native library.
     */
    public native StopAction getStopAction() throws Ev3Exception;

    /**
     * Returns a list of valid StopActions for the motor.<p>
     * Setting a StopAction not on this list using the {@link Motor#setStopAction} method will result in an exception or crash.
     * @return a list of supported StopActions
     * @throws Ev3Exception when an unexpected value is returned by the native library.
     */
    public native StopAction[] getSupportedStopActions() throws Ev3Exception;

    /**
     * Returns the time the motor will run in milliseconds when run using the {@link Motor#rotateUntil} method.
     * This value will be ignored by other rotate methods.
     * @return the targeted duration the motor will run
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native int getTargetDuration() throws Ev3Exception;

    /**
     * Return true when the motor is holding a fixed position by force.
     * @return whether the motor is trying to hold it's position
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native boolean isHolding() throws Ev3Exception;

    /**
     * Returns true when the motor is at 100% load but can't reach the set target speed.
     * @return whether the motor is overloaded
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native boolean isOverloaded() throws Ev3Exception;

    /**
     * Returns true when the motors speed is changing either up or down and has not reached it's target speed.
     * @return whether the motor is changing speed
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native boolean isRamping() throws Ev3Exception;

    /**
     * Returns true when the motor is receiving power to do something.
     * @return whether the motor is doing anything
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native boolean isRunning() throws Ev3Exception;

    /**
     * Returns true when the motor is trying to move but can't.
     * @return whether the motor is stuck
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native boolean isStalled() throws Ev3Exception;

    /**
     * Resets the motor to the default settings.<p>
     * This will stop the motor as a side effect.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void reset() throws Ev3Exception;

    /**
     * Rotates the motor based on the set target load.<p>
     * This rotate variant ignores any settings like speed, time, etc. only the target load setting has an effect on this variant.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void rotateLoad() throws Ev3Exception;
    
    /**
     * Starts rotating the motor until another command is send based on the set speed.<p>
     * Changing any setting after the call to rotate will have no effect.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void rotate() throws Ev3Exception;
    
    /**
     * Rotates the motor for as long as set using the {@link Motor#setTargetDuration} in milliseconds.<p>
     * The speed of the motor will be controlled by the {@link Motor#setTargetSpeed} method.<p>
     * Changing any setting after the call to rotate will have no effect.<p>
     * The stop action is used to end the rotation once the target is reached.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void rotateUntil() throws Ev3Exception;

    /**
     * Rotates the motor for as long as specified in the millis parameter in milliseconds.<p>
     * This overload ignores the set target duration.
     * The speed of the motor will be controlled by the {@link Motor#setTargetSpeed} method.<p>
     * Changing any setting after the call to rotate will have no effect.<p>
     * The stop action is used to end the rotation once the target is reached.
     * @param mills the time the motor will be rotating for
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void rotateUntil(long mills) throws Ev3Exception;

    /**
     * Rotates the motor to the absolute position set by the {@link Motor#setTargetPosition} method.<p>
     * Whether the motor rotates clockwise or counter-clockwise is determent by whether the target position is larger or smaller then the current position.<p>
     * The current position can be acquired using the {@link Motor#getAbsolutePosition} method.<p>
     * Changing any setting after the call to rotate will have no effect.<p>
     * The stop action is used to end the rotation once the target is reached.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void rotateAbsolute() throws Ev3Exception;

    /**
     * Rotates the motor to the absolute position specified in the pos parameter.<p>
     * This overload ignores the set target position.
     * Whether the motor rotates clockwise or counter-clockwise is determent by whether the target position is larger or smaller then the current position.<p>
     * The current position can be acquired using the {@link Motor#getAbsolutePosition} method.<p>
     * Changing any setting after the call to rotate will have no effect.<p>
     * The stop action is used to end the rotation once the target is reached.
     * @param pos the absolute target position of the motor in Units
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void rotateAbsolute(int pos) throws Ev3Exception;

    /**
     * Rotates the motor relative to the current position set by the {@link Motor#setTargetPosition} method.<p>
     * Whether the motor rotates clockwise or counter-clockwise is determent by whether the target position is positive or negative.<p>
     * Changing any setting after the call to rotate will have no effect.<p>
     * The stop action is used to end the rotation once the target is reached.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void rotateRelative() throws Ev3Exception;

    /**
     * Rotates the motor relative to the current position specified in the pos parameter.<p>
     * This overload ignores the set target position.
     * Whether the motor rotates clockwise or counter-clockwise is determent by whether the target position is positive or negative.<p>
     * Changing any setting after the call to rotate will have no effect.<p>
     * The stop action is used to end the rotation once the target is reached.
     * @param pos the relative target position of the motor in Units
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void rotateRelative(int pos) throws Ev3Exception;

    /**
     * Sets the targeted load for the motor in %.<p>
     * This setting only applies to the {@link Motor#rotateLoad} method, the others will ignore it.<p>
     * This setting is the only setting that takes effect even after calling the rotate method. 
     * (It takes effect immediately)
     * @param load the new target load for the motor
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setTargetLoad(int load) throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Sets the derivative pub constant for the position PID.
     * @param kd the new derivative constant
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setHoldPidKd(float kd) throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Sets the integral pub constant for the position PID.
     * @param ki the new integral constant
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setHoldPidKi(float ki) throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Sets the proportional pub constant for the position PID.
     * @param kp the new proportional constant
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setHoldPidKp(float kp) throws Ev3Exception;

    /**
     * Sets the polarity of the motor to the value passed as a parameter.<p>
     * The polarity will decide whether the motor runs clockwise or counter_clockwise.
     * @param polarity the new polarity for the motor
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setPolarity(Polarity polarity) throws Ev3Exception;

    /**
     * Sets the current position of the motor.<p>
     * This will NOT rotate the motor it will simply reset the internal position value to the value passed as a parameter.
     * @param position the new position of the motor. 
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setPosition(int position) throws Ev3Exception;

    /**
     * Sets the target position of the motor<p>
     * This is both the absolute value for {@link Motor#rotateAbsolute} and the relative value for {@link Motor#rotateRelative}.
     * @param position the new target position
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setTargetPosition(int position) throws Ev3Exception;

    /**
     * Sets the time the motor will take to slow down to standstill in milliseconds.<p>
     * The actual slowdown can vary based on speed and outside side effects.
     * @param slowdown the time the motor will need to stop in milliseconds
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setSlowdownTime(int slowdown) throws Ev3Exception;


    /**
     * Sets the time the motor will take to speed up from standstill to set speed in milliseconds.<p>
     * The actual speedup can vary based on speed and outside side effects.
     * @param speedup the time the motor will need to reach full speed in milliseconds
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setSpeedupTime(int speedup) throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Sets the derivative pub constant for the speed regulation PID.
     * @param kd the new derivative constant
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setSpeedPidKd(float kd) throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Sets the integral pub constant for the speed regulation PID.
     * @param ki the new integral constant
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setSpeedPidKi(float ki) throws Ev3Exception;

    /**
     * <h2>Experimental</h2>
     * Sets the proportional pub constant for the speed regulation PID.
     * @param kp the new proportional constant
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setSpeedPidKp(float kp) throws Ev3Exception;

    /**
     * Sets the target speed set for the motor.<p>
     * The maximum possible speed is returned by the {@link Motor#getMaxSpeed} method.
     * @param speed the new targeted speed
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setTargetSpeed(int speed) throws Ev3Exception;

    /**
     * Sets the StopAction active for the motor.<p>
     * This value determents the action taken when the {@link Motor#stop} method is called.
     * @param action the new StopAction
     * @throws Ev3Exception when an unexpected value is received by the native library.
     */
    public native void setStopAction(StopAction action) throws Ev3Exception;

    /**
     * Sets the time the motor will run in milliseconds when run using the {@link Motor#rotateUntil} method.
     * This value will be ignored by other rotate methods.
     * @param mills the new targeted duration the motor will run
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void setTargetDuration(int mills) throws Ev3Exception;

    /**
     * Stops the motor using the StopAction set using the {@link Motor#setStopAction} method.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void stop() throws Ev3Exception;

    /**
     * Returns true when the motor is a Ev3 LargeMotor.
     * @return rather this is a large motor or not
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native boolean isLarge() throws Ev3Exception;

    /**
     * Blocks the current thread until the motor stopped moving.<p>
     * This method blocks until {@link Motor#isRunning} returns false.
     * @throws Ev3Exception when {NOT DOCUMENTED}
     * @apiNote this method is equivalent to calling:
     * <pre>
     * motor.sleepWhile(State.RUNNING);
     * </pre>
     */
    public native void sleepUntilNotMoving() throws Ev3Exception;
    
    /**
     * Blocks the current thread until the motor stopped moving or a set timeout is reached.<p>
     * This method blocks until {@link Motor#isRunning} returns false.
     * Returns true when the motor stopped before the timeout.
     * @param timeout the timeout in milliseconds
     * @return rather the timeout was reached or not
     * @throws Ev3Exception when {NOT DOCUMENTED}
     * @apiNote this method is equivalent to calling:
     * <pre>
     * motor.sleepWhile(State.RUNNING, timeout);
     * </pre>
     */
    public native boolean sleepUntilNotMoving(long timeout) throws Ev3Exception;

    /**
     * Blocks the current thread until the passed state is reached.<p>
     * @param state the state required to unblock the thread
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void sleepUntil(MotorState state) throws Ev3Exception;

    /**
     * Blocks the current thread until the passed state is reached or a set timeout is reached.<p>
     * Returns true when the condition was reached before the timeout.
     * @param state the state required to unblock the thread
     * @param timeout the timeout in milliseconds
     * @return rather the timeout was reached or not
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native boolean sleepUntil(MotorState state, long timeout) throws Ev3Exception;

    /**
     * Blocks the current thread while the passed condition is met.<p>
     * @param state the state while which the thread will stay blocked
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native void sleepWhile(MotorState state) throws Ev3Exception;

        /**
     * Blocks the current thread while the passed condition is met or a set timeout is reached.<p>
     * Returns true when the condition ended the sleep before the timeout was reached.
     * @param state the state while which the thread will stay blocked
     * @param timeout the timeout in milliseconds
     * @return rather the timeout was reached or not
     * @throws Ev3Exception when {NOT DOCUMENTED}
     */
    public native boolean sleepWhile(MotorState state, long timeout) throws Ev3Exception;

    /**
     * Blocks the current thread until the condition returns true.<p>
     * The condition is checked every time the State of the motor changes.
     * @param <X> the exception thrown by the condition
     * @param condition the condition that needs to be met to unblock the thread
     * @throws Ev3Exception when {NOT DOCUMENTED}
     * @throws X when the condition callback function throws X
     */
    public native <X extends Exception> void sleep(Condition<X> condition) throws Ev3Exception, X;

    /**
     * Blocks the current thread until the condition returns true or a set timeout is reached.<p>
     * The condition is checked every time the State of the motor changes.<p>
     * Returns true when the condition was reached before the timeout.
     * @param condition the condition that needs to be met to unblock the thread
     * @param timeout the timeout in milliseconds
     * @param <X> the exception thrown by the condition
     * @return rather the timeout was reached or not
     * @throws Ev3Exception when {NOT DOCUMENTED}
     * @throws X when the condition callback function throws X
     */
    public native <X extends Exception> boolean sleep(Condition<X> condition, long timeout) throws Ev3Exception, X;

    /**
     * Defines the action the motor will use to slow down to standstill.
     * @author RedIODev
     */
    public enum StopAction {
        /**
         * Removes power from the motor and creates a passive electrical load.
         * This is usually done by shorting the motor terminals together.
         * This load will absorb the energy from the rotation of the motors and cause the motor to stop more quickly than coasting.
         */
        BRAKE,
        /**
         * Removes power from the motor. The motor will freely coast to a stop.
         */
        COAST,
        /**
         * Causes the motor to actively try to hold the current position.
         * If an external force tries to turn the motor, the motor will "push back" to maintain its position.
         */
        HOLD
    }

    /**
     * Defines the polarity of the motor. 
     * @author RedIODev
     */
    public enum Polarity {
        /**
         * A positive load will cause the motor to rotate clockwise.
         */
        NORMAL,

        /**
         * A positive load will cause the motor to rotate counter-clockwise.
         */
        INVERSED
    }

    
}
