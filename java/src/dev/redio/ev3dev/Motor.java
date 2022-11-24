package dev.redio.ev3dev;

import dev.redio.ev3dev.alloc.Native;
import dev.redio.ev3dev.alloc.NativeMethod;
import dev.redio.ev3dev.exceptions.Ev3Exception;

public final class Motor extends Native {

    public static native Motor find() throws Ev3Exception;

    public static native Motor[] list() throws Ev3Exception;

    public Motor(MotorPort port) throws Ev3Exception {
        super(port);
    }

    @NativeMethod
    private Motor() {}

    @Override
    protected native void new0(Object... args) throws Ev3Exception;

    @Override
    protected native void delete0();

    public native int getCountPerMeter() throws Ev3Exception;

    public native int getCountPerRotation() throws Ev3Exception;

    public native int getDutyCycle() throws Ev3Exception;

    public native int getDutyCycleSetpoint() throws Ev3Exception;

    public native int getFullTravelCount() throws Ev3Exception;

    public native float getHoldPidKd() throws Ev3Exception;

    public native float getHoldPidKi() throws Ev3Exception;

    public native float getHoldPidKp() throws Ev3Exception;

    public native int getMaxSpeed() throws Ev3Exception;

    public native String getPolarity() throws Ev3Exception;

    public native int getPosition() throws Ev3Exception;

    public native int getRampDownSetpoint() throws Ev3Exception;
    
    public native int getRampUpSetpoint() throws Ev3Exception;

    public native int getSpeed() throws Ev3Exception;

    public native float getSpeedPidKd() throws Ev3Exception;

    public native float getSpeedPidKi() throws Ev3Exception;

    public native float getSpeedPidKp() throws Ev3Exception;

    public native int getSpeedPidSetpoint() throws Ev3Exception;

    public native String[] getState() throws Ev3Exception;

    public native String getStopAction() throws Ev3Exception;

    public native String[] getStopActions() throws Ev3Exception;

    public native int getTimeSetpoint() throws Ev3Exception;

    public native boolean isHolding() throws Ev3Exception;

    public native boolean isOverloaded() throws Ev3Exception;

    public native boolean isRamping() throws Ev3Exception;

    public native boolean isRunning() throws Ev3Exception;

    public native boolean isStalled() throws Ev3Exception;

    public native void reset() throws Ev3Exception;

    public native void runDirect() throws Ev3Exception;

    public native void runForever() throws Ev3Exception;

    public native void runTimed() throws Ev3Exception;

    public native void runTimed(long mills) throws Ev3Exception;

    public native void runToAbsolutePosition() throws Ev3Exception;

    public native void runToAbsolutePosition(int pos) throws Ev3Exception;

    public native void runToRelativePosition() throws Ev3Exception;

    public native void runToRelativePosition(int pos) throws Ev3Exception;

    public native void setDutyCycleSetpoint(int dutyCycle) throws Ev3Exception;

    public native void setHoldPidKd(float kd) throws Ev3Exception;

    public native void setHoldPidKi(float ki) throws Ev3Exception;

    public native void setHoldPidKp(float kp) throws Ev3Exception;

    public native void setPolarity(String polarity) throws Ev3Exception;

    public native void setPosition(int position) throws Ev3Exception;

    public native void setPositionSetpoint(int position) throws Ev3Exception;

    public native void setRampDownSetPoint(int rampDown) throws Ev3Exception;

    public native void setRampUpSetPoint(int rampUp) throws Ev3Exception;

    public native void setSpeedPidKd(float kd) throws Ev3Exception;

    public native void setSpeedPidKi(float ki) throws Ev3Exception;

    public native void setSpeedPidKp(float kp) throws Ev3Exception;

    public native void setSpeedSetPoint(int speed) throws Ev3Exception;

    public native void setStopAction(String action) throws Ev3Exception;

    public native void setTimeSetPoint(int mills) throws Ev3Exception;

    public native void stop() throws Ev3Exception;

    public native boolean isLarge() throws Ev3Exception;

    public native void waitUntilNotMoving() throws Ev3Exception;
}
