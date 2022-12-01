package dev.redio.ev3dev;

/**
     * Defines the possible states the motor can be in.
     * @author RedIODev
     */
    public enum MotorState {
        /**
         * The motor is not turning, but rather attempting to hold a fixed position.
         */
        HOLDING,
        /**
         * The motor is turning as fast as possible, but cannot reach the value set using {@link Motor#setTargetSpeed} method.
         */
        OVERLOADED,
        /**
         * The motor is ramping up or down and has not yet reached a stable speed.
         */
        RAMPING,
        /**
         * Power is being sent to the motor.
         */
        RUNNING,
        /**
         * The motor is trying to run but is not turning at all.
         */
        STALLED
    }