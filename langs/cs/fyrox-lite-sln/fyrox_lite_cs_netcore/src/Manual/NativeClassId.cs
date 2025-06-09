namespace FyroxLite;

internal partial struct NativeClassId : IEquatable<NativeClassId>
{
    // that range is reserved for script classes. next are used lazily by messaging system
    internal const int MaxScriptClassCount = 100000;
    
    public NativeClassId(int value)
    {
        this.value = value;
    }

    public static NativeClassId Resolve(Type type)
    {
        return _state.GetInRightThread().ByType[type];
    }

    internal static class By<T>
    {
        // ReSharper disable once StaticMemberInGenericType
        [ThreadStatic] private static NativeClassId? _value;

        internal static NativeClassId Resolve()
        {
            if (_value == null)
            {
                var nativeClassIds = _state.GetInRightThread().ByType;
                if (!nativeClassIds.TryGetValue(typeof(T), out var classId))
                {
                    classId = new NativeClassId(_state.GetInRightThread().NextTypeId++);
                    
                    Register(typeof(T), classId);
                    nativeClassIds[typeof(T)] = classId;
                }
                _value = classId;
            }

            return _value.Value;
        }
    }

    [ThreadStatic] private static State? _state;
    
    private class State
    {
        internal int NextTypeId = MaxScriptClassCount + 1;
        internal readonly Dictionary<Type, NativeClassId> ByType = new();
        internal readonly Dictionary<NativeClassId, Type> ById = new();
    }

    internal Type GetCsClass()
    {
        if (_state.GetInRightThread().ById.TryGetValue(this, out var type))
        {
            return type;
        }

        throw new Exception($"No types associated with {this}. associations: [{string.Join(", ", _state.GetInRightThread().ById.Select(it => $"{it.Key}: {it.Value}"))}]");
    }

    internal static void InitThread()
    {
        _state ??= new State();
    }

    public static void Clear()
    {
        _state.GetInRightThread().ById.Clear();
        _state.GetInRightThread().ByType.Clear();
    }

    internal static void Register(Type type, NativeClassId id)
    {
        _state.GetInRightThread().ByType[type] = id;
        _state.GetInRightThread().ById[id] = type;
    }

    public bool Equals(NativeClassId other)
    {
        return value == other.value;
    }

    public override bool Equals(object? obj)
    {
        return obj is NativeClassId other && Equals(other);
    }

    public override int GetHashCode()
    {
        return value.GetHashCode();
    }

    public override string ToString()
    {
        return $"{nameof(NativeClassId)}({value})";
    }
}