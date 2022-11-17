package dev.redio.ev3dev.alloc;

public abstract class Native<NX extends Exception, DX extends Exception> implements AutoCloseable {
    
    static {
        System.load("/lib/libev3.a");
    }

    @NativeField
    private long ptr;

    protected Native(Object... args) throws NX {
        new0(args);
    }

    @Override
    public void close() throws DX {
        delete0();
    }

    protected abstract void new0(Object... args) throws NX;

    protected abstract void delete0() throws DX;
}
