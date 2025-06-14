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

// fyrox_lite::lite_color::LiteColor

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeColor
{
    private byte _r;
    private byte _g;
    private byte _b;
    private byte _a;
}
#region internal wrappers


[StructLayout(LayoutKind.Sequential)]
internal struct NativeColor_optional
{
    internal NativeColor value;
    internal int has_value;

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static Color? ToFacade(in NativeColor_optional value)
    {
        if (value.has_value != 0)
        {
            var __item = value.value;
            var __item_to_facade = NativeColor.ToFacade(__item);
            return __item_to_facade;
        }
        return null;
    }

    [MethodImpl(MethodImplOptions.AggressiveInlining)]
    public static NativeColor_optional FromFacade(in Color? value)
    {
        if (value == null)
        {
            return new NativeColor_optional { value = default, has_value = 0 };
        }
        var __item = value.Value;
        var __item_from_facade = NativeColor.FromFacade(__item);
        return new NativeColor_optional { value = __item_from_facade, has_value = 1 };
    }
}

[StructLayout(LayoutKind.Sequential)]
internal partial struct NativeColor_slice
{
    internal unsafe NativeColor* begin;
    internal int length;

    internal unsafe NativeColor_slice(NativeColor* begin, int length)
    {
        this.begin = begin;
        this.length = length;
    }

    internal static unsafe List<Color> ToFacade(in NativeColor_slice self)
    {
        var fetched = new List<Color>();

        for (var i = 0; i < self.length; i++)
        {
            var __item = *(self.begin + i);
            var __item_to_facade = NativeColor.ToFacade(__item);
            fetched.Add(__item_to_facade);
        }
        return fetched;
    }

    [ThreadStatic]
    private static NativeColor[]? _uploadBuffer;

    internal static NativeColor_slice FromFacade(in List<Color> self)
    {
        _uploadBuffer ??= new NativeColor[1024];
        while (_uploadBuffer.Length < self.Count)
        {
            _uploadBuffer = new NativeColor[_uploadBuffer.Length * 2];
        }

        for (var i = 0; i < self.Count; i++)
        {
            var __item = self[i];
            var __item_from_facade = NativeColor.FromFacade(__item);
            _uploadBuffer[i] = __item_from_facade;
        }

        unsafe
        {
            fixed (NativeColor* buffer_ptr = _uploadBuffer)
            {
                var native_slice = fyrox_lite_upload_fyrox_lite_lite_color_LiteColor_slice(new NativeColor_slice(buffer_ptr, self.Count));
                return native_slice;
            }
        }
    }

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static unsafe partial NativeColor_slice fyrox_lite_upload_fyrox_lite_lite_color_LiteColor_slice(NativeColor_slice managed);
}

[StructLayout(LayoutKind.Sequential)]
internal struct NativeColor_result
{
    internal int ok;
    internal NativeColor_result_value value;

    internal static unsafe Color ToFacade(in NativeColor_result self)
    {
        if (self.ok != 0)
        {
            var __item = self.value.ok;
            var __item_to_facade = NativeColor.ToFacade(__item);
            return __item_to_facade;
        }
        throw new Exception(NativeString.ToFacade(self.value.err));
    }

    internal static NativeColor_result FromFacade(in Color self)
    {
        var __item = self;
        var __item_from_facade = NativeColor.FromFacade(__item);
        return new NativeColor_result {ok = 1, value = new NativeColor_result_value { ok = __item_from_facade } };
    }

    internal static NativeColor_result FromFacadeError(in string err)
    {
        return new NativeColor_result {ok = 0, value = new NativeColor_result_value { err = NativeString.FromFacade(err) } };
    }
}

[StructLayout(LayoutKind.Explicit)]
internal struct NativeColor_result_value
{
    [FieldOffset(0)]
    internal NativeColor ok;

    [FieldOffset(0)]
    internal NativeString err;
}
#endregion