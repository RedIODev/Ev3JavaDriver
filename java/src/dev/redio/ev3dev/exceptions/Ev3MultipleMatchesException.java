package dev.redio.ev3dev.exceptions;

import java.util.Arrays;

public class Ev3MultipleMatchesException extends Exception {
    private final String device;
    private final String[] ports;

    public Ev3MultipleMatchesException(String device, String[] ports) {
        super("Multiple '" + device + "' connected at ports " + Arrays.toString(ports));
        this.device = device;
        this.ports = ports;
    }

    public String device() {
        return device;
    }

    public String[] ports() {
        return ports.clone();
    }
}
