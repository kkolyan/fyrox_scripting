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

// fyrox_lite::lite_script::LiteGlobalScript

public abstract partial class GlobalScript
{

    
    /// <para>find a global script by type</para>
    public static T Get<T>() where T : class
    {
        #region native call
        unsafe {
            
            var __ret = fyrox_lite_lite_script_LiteGlobalScript_get(NativeClassId.By<T>.Resolve());
            return NativeInstanceId_result.ToFacade(__ret) as T;
        }
        #endregion
    }

    #region native internal methods

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial NativeInstanceId_result fyrox_lite_lite_script_LiteGlobalScript_get(NativeClassId class_id);
    #endregion

}
#region internal type wrappers

#endregion