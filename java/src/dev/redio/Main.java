package dev.redio;

import dev.redio.ev3dev.ColorSensor;
import dev.redio.ev3dev.Motor;
import dev.redio.ev3dev.MotorPort;
import dev.redio.ev3dev.SensorPort;
import dev.redio.ev3dev.ColorSensor.Mode;
import dev.redio.ev3dev.Motor.Polarity;
import dev.redio.ev3dev.Motor.StopAction;
import dev.redio.ev3dev.exceptions.Ev3Exception;

public class Main {
    public static void main(String[] args) throws Ev3Exception {
        try (var right = new Motor(MotorPort.OUT_A);
                var left = new Motor(MotorPort.OUT_D);
                var s = new ColorSensor(SensorPort.IN_1)) {
            s.setMode(Mode.REFLECTION);
            right.setTargetSpeed(200);
            left.setTargetSpeed(200);
            right.setStopAction(StopAction.BRAKE);
            left.setStopAction(StopAction.BRAKE);
            //right.setPolarity(Polarity.INVERSED);
            //left.setPolarity(Polarity.INVERSED);
            while (true) {
                var intensity = s.getIntensity();   //0 - 1000 (0 == bright 1000 == dark)
                System.out.println(intensity);
                if (intensity > 600) { //<450
                    left.rotateRelative(15);
                    left.sleepUntilNotMoving();
                } else {
                    right.rotateRelative(15);
                    right.sleepUntilNotMoving();
                }
            }
        }
    }

    
}
