namespace System;

public class Exception
{
    public virtual string Message { get; }
    public Exception? InnerException { get; }

    public Exception()
    {
        Message = "An exception occurred.";
    }

    public Exception(string message)
    {
        Message = message;
    }

    public Exception(string message, Exception inner)
    {
        Message = message;
        InnerException = inner;
    }
}