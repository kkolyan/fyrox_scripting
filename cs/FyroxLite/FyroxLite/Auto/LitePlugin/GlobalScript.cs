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

// fyrox_lite::lite_plugin::LiteGlobalScript
public abstract partial class GlobalScript
{

    public static T Get<T>() where T : class
    {
        unsafe {
            
            var __ret = fyrox_lite_lite_plugin_LiteGlobalScript_get(NativeClassId.By<T>.Resolve());
            return NativeInstanceId_result.ToFacade(__ret) as T;
        }
    }

    [LibraryImport("fyrox_c", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeInstanceId_result fyrox_lite_lite_plugin_LiteGlobalScript_get(NativeClassId class_id);
}