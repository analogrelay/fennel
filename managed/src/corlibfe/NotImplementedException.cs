namespace System;

public class NotImplementedException : SystemException
{
    const string DefaultMessage = "The method or operation is not implemented.";

    public NotImplementedException()
        : base(DefaultMessage)
    {
    }

    public NotImplementedException(string message)
        : base(message ?? DefaultMessage)
    {
    }

    public NotImplementedException(string message, Exception inner)
        : base(message ?? DefaultMessage, inner)
    {
    }
}