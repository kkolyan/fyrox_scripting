namespace FyroxLite;

internal partial struct UserScriptMessage
{
    private UserScriptMessage(long id, NativeClassId classId)
    {
        this.id = id;
        class_id = classId;
    }

    internal static UserScriptMessage FromFacade(object value)
    {
        var nativeClassId = NativeClassId.Resolve(value.GetType());
        return new UserScriptMessage(ObjectRegistry.Put(value), nativeClassId);
    }

    internal static object? ToFacade(UserScriptMessage id) => ObjectRegistry.Get(id.id);

    internal static void DropMessage(UserScriptMessage id) => ObjectRegistry.Drop(id.id);

    public bool Equals(UserScriptMessage other) => id == other.id && class_id.Equals(other.class_id);

    public override bool Equals(object? obj) => obj is UserScriptMessage other && Equals(other);

    public override int GetHashCode() => HashCode.Combine(id, class_id);
}