package dev.redio.ev3dev;

import dev.redio.ev3dev.alloc.Native;
import dev.redio.ev3dev.alloc.NativeMethod;
import dev.redio.ev3dev.exceptions.Ev3Exception;

public final class Motor extends Native<Ev3Exception, RuntimeException> {

    @Override
    protected native void new0(Object... args) throws Ev3Exception;

    @Override
    protected native void delete0();

    public native int getCountPerRotation() throws Ev3Exception;

    // public native int getCountPerM() throws Ev3Exception;

    // public native int getFullTravelCount() throws Ev3Exception;

    // public native int getDutyCycle() throws Ev3Exception;

    //public native int getDutyCycleSp() throws Ev3Exception;

     public native void setDutyCycleSetpoint(int dutyCycle) throws Ev3Exception;

    // public native int getPolarity() throws Ev3Exception;

    // public native void setPolarity(int polarity) throws Ev3Exception;

    // public native int getPosition() throws Ev3Exception;

    // public native void setPosition(int position) throws Ev3Exception;

    // public native float getHoldPidKp() throws Ev3Exception;

    // public native void setHoldPidKp(float holdPid) throws Ev3Exception;

    // public native float getHoldPidKi() throws Ev3Exception;

    // public native void setHoldPidKi(float holdPid) throws Ev3Exception;

    // public native float getHoldPidKd() throws Ev3Exception;

    // public native void setHoldPidKd(float holdPid) throws Ev3Exception;

    // public native int getMaxSpeed() throws Ev3Exception;

    // public native int getPositionSp() throws Ev3Exception;

    // public native void setPositionSp(int position) throws Ev3Exception;

    //...

    public native void setTimeSetPoint(int mills) throws Ev3Exception;

    public native void setSpeedSetPoint(int speed) throws Ev3Exception;

    public native void runDirect() throws Ev3Exception;

    public native void runTimed() throws Ev3Exception;

    public native void runTimed(long mills) throws Ev3Exception;

    public native void runForever() throws Ev3Exception;

    public native String getStopAction() throws Ev3Exception;

    public native void stop() throws Ev3Exception;

    public Motor(MotorPort port) throws Ev3Exception {
        super(port);
    }

    @NativeMethod
    private Motor() {}
}
