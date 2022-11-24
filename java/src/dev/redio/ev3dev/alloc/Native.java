package dev.redio.ev3dev.alloc;

import dev.redio.ev3dev.exceptions.Ev3Exception;

public abstract class Native implements AutoCloseable {
    
    static {
        System.loadLibrary("ev3");
    }

    @NativeField
    private long ptr;

    protected Native(Object... args) throws Ev3Exception {
        new0(args);
    }

    @NativeMethod
    protected Native() {}

    @Override
    public void close() throws Ev3Exception {
        delete0();
    }

    protected abstract void new0(Object... args) throws Ev3Exception;

    protected abstract void delete0() throws Ev3Exception;
}
