package dev.redio.ev3dev;

import dev.redio.ev3dev.alloc.Native;
import dev.redio.ev3dev.alloc.NativeMethod;
import dev.redio.ev3dev.exceptions.Ev3Exception;

/**
 * The class representing a LEGO Mindstorm EV3 ColorSensor<p>
 * The ColorSensor supports 5 modes: Simple Color, Color, Simple Reflection, Reflection and Ambient
 * <h2>Simple Color</h2>
 * In simple color mode the sensor will measure how close the color is to a preset of colors.
 * The sensor will return the color from the preset closest to the actual color.
 * <h2>Color</h2>
 * In color mode the sensor will measure the color of the object in front of it.<p>
 * The color can be read by using either the methods for the 3 color values 
 * {@link ColorSensor#getRed}, {@link ColorSensor#getGreen} and {@link ColorSensor#getBlue} 
 * or by using the {@link ColorSensor#getRGB} method which returns a {@link Color} object.
 * For the ranges of RGB values returned see the {@link Color} class.
 * <h2>Simple reflection</h2>
 * In simple reflection mode the sensor will measure the light reflected by the object in front of it.<p>
 * The intensity of the measured light can be read using the {@link ColorSensor#getIntensity} method.
 * For the range of intensity values returned see the {@link ColorSensor#getIntensity()} method.
 * <h2>Reflection</h2>
 * In reflection mode the sensor will measure the light reflected by the object in front of it.<p>
 * The difference between simple reflection mode and reflection mode is that reflection mode returns the raw sensor data while simple reflection mode returns a percent value.
 * For more information and ranges of the values returned see the {@link Reflection} class.
 * <h2>Ambient</h2>
 * In ambient mode the sensor will measure the light intensity of the environment<p>
 * The intensity of the measured light can be read using the {@link ColorSensor#getIntensity} method.
 * For the range of intensity values returned see the {@link ColorSensor#getIntensity} method.
 * @see Color
 * @see SensorPort
 * @see Reflection
 * @see SimpleColor
 * @author RedIODev
 */
public final class ColorSensor extends Native {

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
    
    /**
     * Returns the red value measured by the sensor in Color mode.
     * The value ranges from 0 to 1020.
     * @return the red value measured
     * @throws Ev3Exception when the sensor is not in Color mode.
     * @see Color
     */
    public native int getRed() throws Ev3Exception;

    /**
     * Returns the green value measured by the sensor in Color mode.
     * The value ranges from 0 to 1020.
     * @return the green value measured
     * @throws Ev3Exception when the sensor is not in Color mode.
     * @see Color
     */
    public native int getGreen() throws Ev3Exception;

    /**
     * Returns the blue value measured by the sensor in Color mode.
     * The value ranges from 0 to 1020.
     * @return the blue value measured
     * @throws Ev3Exception when the sensor is not in Color mode.
     * @see Color
     */
    public native int getBlue() throws Ev3Exception;

    /**
     * Returns the RGB value measured by the sensor in Color mode as a Color object.
     * @return the Color object representing the RGB value measured
     * @throws Ev3Exception when the sensor is not in Color mode.
     * @see Color
     */
    public native Color getRGB() throws Ev3Exception;
    
    /**
     * Returns the light intensity measured by the sensor.
     * <h3>Simple reflection</h3>
     * In reflection mode the values vary between 0 (100% reflection) to 100 (100% absorption).
     * <h3>Ambient</h3>
     * In ambient mode the values vary between 0 (No light) to 100 (bright).
     * @return intensity of light in simple reflection and ambient mode
     * @throws Ev3Exception when the sensor is not in simple reflection or ambient mode.
     * @implNote the intensity values can vary based on lighting conditions.
     * @see Color
     */
    public native int getIntensity() throws Ev3Exception;

    /**
     * Returns the Reflection measured by the sensor.
     * @return the reflection value in reflection mode
     * @throws Ev3Exception when the sensor is not in reflection mode.
     * @implNote the values can vary based on lighting conditions.
     * @see Reflection
     */
    public native Reflection getReflection() throws Ev3Exception;

    /**
     * Returns the Color measured by the sensor.
     * @return the enum variant representing the measured color
     * @throws Ev3Exception when the sensor is not in simple color mode.
     * @see SimpleColor
     */
    public native SimpleColor getSimpleColor() throws Ev3Exception;

    /**
     * Sets the mode of the sensor.
     * @param mode the new mode of the sensor
     * @throws Ev3Exception when something went wrong changing the mode.
     * @see ColorSensor.Mode
     */
    public native void setMode(Mode mode) throws Ev3Exception;

    /**
     * Gets the current mode of the sensor.
     * @return the mode of the sensor
     * @throws Ev3Exception when something went wrong reading the current mode.
     */
    public native Mode getMode() throws Ev3Exception;

    /**
     * Represents the mode of the ColorSensor
     * @see ColorSensor
     * @author RedIODev
     */
    public enum Mode {
        /**
         * Measures color based on a preset of colors.
         */
        SIMPLE_COLOR,
        /**
         * Measures color in full RGB mode with values from 0-1020 each.
         */
        COLOR,
        /**
         * Measures the light reflected in percent.
         */
        SIMPLE_REFLECTION,
        /**
         * Measures the light reflected with detailed value x and y ranging from 0-1020 each.
         */
        REFLECTION,
        /**
         * Measures the light level around the sensor in percent.
         */
        AMBIENT;
    }

    /**
     * Represents the colors measured by the sensor in simple color mode.
     * @see ColorSensor#getSimpleColor
     * @author RedIODev
     */
    public enum SimpleColor {
        /**
         * No color.
         */
        NONE,
        /**
         * Black.
         */
        BLACK,
        /**
         * Blue.
         */
        BLUE,
        /**
         * Green.
         */
        GREEN,
        /**
         * Yellow.
         */
        YELLOW,
        /**
         * Red.
         */
        RED,
        /**
         * White.
         */
        WHITE,
        /**
         * Brown.
         */
        BROWN,
    }
}
