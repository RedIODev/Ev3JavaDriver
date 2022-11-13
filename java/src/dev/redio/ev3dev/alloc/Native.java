package dev.redio.ev3dev.alloc;

public abstract class Native<NX extends Exception, DX extends Exception> implements AutoCloseable {
    
    @NativeField
    private long ptr;

    protected Native(Args args) throws NX {
        new0(args);
    }

    @Override
    public void close() throws DX {
        delete0();
    }

    protected abstract void new0(Args args) throws NX;

    protected abstract void delete0() throws DX;
}
