package dev.redio.ev3dev;

import dev.redio.ev3dev.alloc.Native;
import dev.redio.ev3dev.alloc.NativeMethod;
import dev.redio.ev3dev.exceptions.EnumConversionException;
import dev.redio.ev3dev.exceptions.Ev3Exception;
//TODO: Store current mode if avalible and prevent mismatch
/**
 * The class representing a LEGO Mindstorm EV3 ColorSensor<p>
 * The ColorSensor supports 3 modes: Color, Reflection and Ambient<p>
 * <h2>Color</h2>
 * In color mode the sensor will measure the color of the object in front of it.<p>
 * The color can be read by using either the methods for the 3 color values 
 * {@link ColorSensor#getRed}, {@link ColorSensor#getGreen} and {@link ColorSensor#getBlue} 
 * or by using the {@link ColorSensor#getRGB} method which returns a {@link Color} object.
 * For the ranges of RGB values returned see the {@link Color} class.
 * <h2>Reflection</h2>
 * In reflection mode the sensor will measure the light reflected by the object in front of it.<p>
 * The intensity of the measured light can be read using the {@link ColorSensor#getIntensity} method.
 * For the range of intensity values returned see the {@link ColorSensor#getIntensity()} method.
 * <h2>Ambient</h2>
 * In ambient mode the sensor will measure the light intensity of the environment<p>
 * The intensity of the measured light can be read using the {@link ColorSensor#getIntensity} method.
 * For the range of intensity values returned see the {@link ColorSensor#getIntensity} method.
 * @see Color
 * @see SensorPort
 * @author RedIODev
 */
public final class ColorSensor extends Native { //https://docs.ev3dev.org/projects/lego-linux-drivers/en/ev3dev-stretch/sensor_data.html#lego-ev3-color
    private Mode mode;
    /**
     * Finds the first ColorSensor connected to the Ev3 brick.
     * @return the first ColorSensor found
     * @throws Ev3Exception when no ColorSensor could be found
     */
    public static native ColorSensor find() throws Ev3Exception;

    /**
     * Finds all ColorSensors connected to the Ev3 brick.
     * @return a list of all sensors connected
     * @throws Ev3Exception when something went wrong retrieving the list of sensors
     */
    public static native ColorSensor[] list() throws Ev3Exception;

    /**
     * Connects to the ColorSensor on the specified {@link SensorPort}.
     * @param port the port the sensor is connected to
     * @throws Ev3Exception when no sensor could be found on the specified port
     */
    public ColorSensor(SensorPort port) throws Ev3Exception {
        super(port);
    }

    @NativeMethod
    private ColorSensor() {}

    @Override
    protected native void new0(Object... args) throws Ev3Exception;

    @Override
    protected native void delete0() throws Ev3Exception;
    
    private native int value0() throws Ev3Exception;

    private native int value1() throws Ev3Exception;

    private native int value2() throws Ev3Exception;

    private native void setMode0(Mode mode) throws Ev3Exception;

    private native Mode getMode0() throws Ev3Exception;
    /**
     * Returns the red value measured by the sensor in Color mode.
     * @return the red value measured
     * @throws Ev3Exception when the sensor is not in Color mode.
     * @see Color
     */
    public int getRed() throws Ev3Exception {
        return value0();
    }

    /**
     * Returns the green value measured by the sensor in Color mode.
     * @return the green value measured
     * @throws Ev3Exception when the sensor is not in Color mode.
     * @see Color
     */
    public int getGreen() throws Ev3Exception {
        return value1();
    }

    /**
     * Returns the blue value measured by the sensor in Color mode.
     * @return the blue value measured
     * @throws Ev3Exception when the sensor is not in Color mode.
     * @see Color
     */
    public int getBlue() throws Ev3Exception {
        return value2();
    }

    /**
     * Returns the RGB value measured by the sensor in Color mode as a Color object.
     * @return the Color object representing the RGB value measured
     * @throws Ev3Exception when the sensor is not in Color mode.
     * @see Color
     */
    public Color getRGB() throws Ev3Exception {
        return new Color(getRed(), getGreen(), getBlue());
    }
    
    public int getBrightness() throws Ev3Exception {
        return value0();
    }
    /**
     * Returns the light intensity measured by the sensor. TODO:WRONG REFLECTION
     * <h3>Reflection</h3>
     * In reflection mode the values vary between 0 (100% reflection) to 1000 (100% absorption)
     * <h3>Ambient</h3>
     * In ambient mode the values vary between 0 to 100 (percent)
     * @return intensity of light in reflection and ambient mode
     * @throws Ev3Exception when the sensor is not in reflection or ambient mode
     * @implNote the intensity values can vary based on lighting conditions.
     * @see Color
     */
    @Deprecated
    public native int getIntensity() throws Ev3Exception;

    public SimpleColor getColor() throws Ev3Exception {
        var index = getIntensity();
        var colors = SimpleColor.values();
        if (index < 0 || index >= colors.length)
            throw new EnumConversionException(index);
        return colors[index];
    }

    public RawReflection getRawReflection() throws Ev3Exception {
        return new RawReflection(value0(), value1());
    }

    /**
     * Sets the mode of the sensor.
     * @param mode the new mode of the sensor
     * @throws Ev3Exception when something went wrong changing the mode
     * @see ColorSensor.Mode
     */
    public Mode getMode() {

    }

    /**
     * Gets the current mode of the sensor.
     * @return the mode of the sensor
     * @throws Ev3Exception when something went wrong reading the current mode
     */
    public void setMode(Mode mode) {
        
    }

    /**
     * Represents the mode of the ColorSensor
     * @see ColorSensor
     */
    public enum Mode {
        SIMPLE_COLOR,
        COLOR,
        REFLECTION,
        RAW_REFLECTION,
        AMBIENT
    }

    public enum SimpleColor {
        NONE,
        BLACK,
        BLUE,
        GREEN,
        YELLOW,
        RED,
        WHITE,
        BROWN
    }
}
