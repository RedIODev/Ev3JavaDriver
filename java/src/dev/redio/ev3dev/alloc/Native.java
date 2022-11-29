package dev.redio.ev3dev.alloc;

import dev.redio.ev3dev.exceptions.Ev3Exception;

/**
 * Defines an abstract base class for native wrapper types.
 * This class provides an API to wrap a struct, class or other data structures from different languages other then Java.
 * The of heap allocated native structure is safely managed by this class.
 * The allocated memory will be released once the close method is called.
 * @apiNote Not calling the close method on an object of this type will result in an memory and potentially an OS resources leak.
 * @author RedIODev
 */
public abstract class Native implements AutoCloseable {
    
    static {
        System.loadLibrary("ev3");
    }

    @NativeField
    private long ptr;
    private boolean isClosed = false;

    protected Native(Object... args) throws Ev3Exception {
        new0(args);
    }

    @NativeMethod
    protected Native() {}

    /**
     * @apiNote calling any method relying on the native structure after close was called results in undefined behavior.
     * {@inheritDoc}
     */
    @Override
    public void close() throws Ev3Exception {
        if (isClosed)
            return;
        isClosed = true;
        delete0();
    }

    /**
     * The creation method for the native structure.
     * This method should allocate, initialize and store the structure in the hidden {@code long ptr} field of this class.
     * @param args the args passed to initialize the native structure
     * @throws Ev3Exception when the native structure could not be created successfully.
     * @apiNote calling this method manually from withing java will result in undefined behavior.
     * It is therefore advised to keep this method protected when implementing it.
     */
    protected abstract void new0(Object... args) throws Ev3Exception;

    /**
     * The deletion method for the native structure.
     * This method uninitializes and deletes the native structure.
     * @throws Ev3Exception when something went wrong freeing the memory or uninitializing the native structure.
     * @apiNote calling this method manually from withing java will result in undefined behavior.
     * It is therefore advised to keep this method protected when implementing it.
     */
    protected abstract void delete0() throws Ev3Exception;
}
