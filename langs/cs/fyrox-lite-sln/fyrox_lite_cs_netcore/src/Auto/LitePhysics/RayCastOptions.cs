// ReSharper disable InconsistentNaming
// ReSharper disable RedundantUnsafeContext
// ReSharper disable UnusedMember.Global
// ReSharper disable RedundantUsingDirective
using FyroxLite;
using System.Drawing;
using System.Runtime.CompilerServices;
using System.Runtime.InteropServices;
using System.Collections;
namespace FyroxLite;

// fyrox_lite::lite_physics::LiteRayCastOptions

[StructLayout(LayoutKind.Sequential)]
public partial struct RayCastOptions
{
    
    /// <para>A ray origin.</para>
    public Vector3 RayOrigin {
        #region get/set with wrapping/unwrapping
        get => NativeVector3.ToFacade(_ray_origin);
        set => _ray_origin = NativeVector3.FromFacade(value);
        #endregion
    }
    
    /// <para>A ray direction. Can be non-normalized.</para>
    public Vector3 RayDirection {
        #region get/set with wrapping/unwrapping
        get => NativeVector3.ToFacade(_ray_direction);
        set => _ray_direction = NativeVector3.FromFacade(value);
        #endregion
    }
    
    /// <para>Maximum distance of cast.</para>
    public float MaxLen {
        #region trivial get/set
        get => _max_len;
        set => _max_len = value;
        #endregion
    }
    
    /// <para>Groups to check.</para>
    public InteractionGroups? Groups {
        #region get/set with wrapping/unwrapping
        get => InteractionGroups_optional.ToFacade(_groups);
        set => _groups = InteractionGroups_optional.FromFacade(value);
        #endregion
    }
    
    /// <para>Whether to sort intersections from closest to farthest.</para>
    public bool SortResults {
        #region get/set with wrapping/unwrapping
        get => NativeBool.ToFacade(_sort_results);
        set => _sort_results = NativeBool.FromFacade(value);
        #endregion
    }
#region Native Fields
//===============================================================
// private fields for all properties (not only mapped),
// because it makes ABI much more readable.
// I hope, NativeAOT will optimize out this.
//===============================================================
    private NativeVector3 _ray_origin;
    private NativeVector3 _ray_direction;
    private float _max_len;
    private InteractionGroups_optional _groups;
    private NativeBool _sort_results;
#endregion
}
#region internal wrappers


[StructLayout(LayoutKind.Sequential)]
internal struct RayCastOptions_optional
{
    internal RayCastOptions value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RayCastOptions? ToFacade(in RayCastOptions_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static RayCastOptions_optional FromFacade(in RayCastOptions? value)
    {
        if (value == null)
        {
            return new RayCastOptions_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = __item;
        return new RayCastOptions_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct RayCastOptions_slice
{
    internal unsafe RayCastOptions* begin;
    internal int length;

    internal unsafe RayCastOptions_slice(RayCastOptions* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<RayCastOptions> ToFacade(in RayCastOptions_slice self)
    {
        var fetched = new List<RayCastOptions>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = __item;
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static RayCastOptions[]? _uploadBuffer;

    internal static RayCastOptions_slice FromFacade(in List<RayCastOptions> self)
    {
        _uploadBuffer ??= new RayCastOptions[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new RayCastOptions[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = __item;
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (RayCastOptions* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_physics_LiteRayCastOptions_slice(new RayCastOptions_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial RayCastOptions_slice fyrox_lite_upload_fyrox_lite_lite_physics_LiteRayCastOptions_slice(RayCastOptions_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct RayCastOptions_result
{
    internal int ok;
    internal RayCastOptions_result_value value;

    internal static unsafe RayCastOptions ToFacade(in RayCastOptions_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = __item;
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static RayCastOptions_result FromFacade(in RayCastOptions self)
    {
        var __item = self;
        var __item_from_facade = __item;
        return new RayCastOptions_result {ok = 1, value = new RayCastOptions_result_value { ok = __item_from_facade } };
    }

    internal static RayCastOptions_result FromFacadeError(in string err)
    {
        return new RayCastOptions_result {ok = 0, value = new RayCastOptions_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct RayCastOptions_result_value
{
    [FieldOffset(0)]
    internal RayCastOptions ok;

    [FieldOffset(0)]
    internal NativeString err;
}
#endregion