package dev.redio;

import dev.redio.ev3dev.LargeMotor;
import dev.redio.ev3dev.MotorPort;
import dev.redio.ev3dev.exceptions.Ev3Exception;

public class Main {
    public static void main(String[] args) throws Ev3Exception {
        try (var lm = new LargeMotor(MotorPort.OUT_A)) {
            lm.runDirect();
            lm.setDutyCycleSp(50);
        }
    }
}
