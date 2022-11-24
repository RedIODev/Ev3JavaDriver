package dev.redio.ev3dev;

import dev.redio.ev3dev.alloc.Native;
import dev.redio.ev3dev.alloc.NativeMethod;
import dev.redio.ev3dev.exceptions.Ev3Exception;

public final class ColorSensor extends Native {

    public static native ColorSensor find() throws Ev3Exception;

    public static native ColorSensor[] list() throws Ev3Exception;


    public ColorSensor(SensorPort port) throws Ev3Exception {
        super(port);
    }

    @NativeMethod
    private ColorSensor() {}

    @Override
    protected native void new0(Object... args) throws Ev3Exception;

    @Override
    protected native void delete0() throws Ev3Exception;
    
    public native int getBlue() throws Ev3Exception;

    public native int getColor() throws Ev3Exception;

    public native int getGreen() throws Ev3Exception;

    public native int getRed() throws Ev3Exception;

    public native Color getRGB() throws Ev3Exception;

    public native void setWhite() throws Ev3Exception;

    public native void setRGBRaw() throws Ev3Exception;

}
