package dev.redio.ev3dev;

import dev.redio.ev3dev.exceptions.Ev3Exception;

public final class LargeMotor implements AutoCloseable {

    private final long rustStructPtr;

    private static native long new0(MotorPort port) throws Ev3Exception, Error;

    private static native void free0(long ptr) throws Ev3Exception, Error;

    private static native int getCountPerRotation0(long ptr) throws Ev3Exception;

    private static native int getCountPerM0(long ptr) throws Ev3Exception;

    private static native int getFullTravelCount0(long ptr) throws Ev3Exception;

    private static native int getDutyCycle0(long ptr) throws Ev3Exception;

    private static native int getDutyCycleSp0(long ptr) throws Ev3Exception;

    private static native void setDutyCycleSp0(long ptr, int dutyCycle) throws Ev3Exception;

    private static native int getPolarity0(long ptr) throws Ev3Exception;

    private static native void setPolarity0(long ptr, int polarity) throws Ev3Exception;

    private static native int getPosition0(long ptr) throws Ev3Exception;

    private static native void setPosition0(long ptr, int position) throws Ev3Exception;

    private static native float getHoldPidKp0(long ptr) throws Ev3Exception;

    private static native void setHoldPidKp0(long ptr, float holdPid) throws Ev3Exception;

    private static native float getHoldPidKi0(long ptr) throws Ev3Exception;

    private static native void setHoldPidKi0(long ptr, float holdPid) throws Ev3Exception;

    private static native float getHoldPidKd0(long ptr) throws Ev3Exception;

    private static native void setHoldPidKd0(long ptr, float holdPid) throws Ev3Exception;

    private static native int getMaxSpeed(long ptr) throws Ev3Exception;

    private static native int getPositionSp0(long ptr) throws Ev3Exception;

    private static native void setPositionSp0(long ptr, int position) throws Ev3Exception;


    public LargeMotor(MotorPort port) throws Ev3Exception {
        this.rustStructPtr = new0(port);

    }
    @Override
    public void close() throws Ev3Exception {
        free0(rustStructPtr);
    }
    
}
