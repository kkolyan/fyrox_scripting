namespace FyroxLite;

internal class StacklessException(string? message) : Exception(message)
{
    public override string ToString()
    {
        return Message;
    }
}