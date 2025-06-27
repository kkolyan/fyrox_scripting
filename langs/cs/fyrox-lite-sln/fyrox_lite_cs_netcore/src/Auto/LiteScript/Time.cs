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

// fyrox_lite::lite_script::LiteTime

public static partial class Time
{
    
    public static float Fps
    {
        get
        {
            #region native call
            unsafe {
                var __ret = fyrox_lite_lite_script_LiteTime_get_fps();
                return __ret;
            }
            #endregion
        }
    }

    #region native internal methods

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial float fyrox_lite_lite_script_LiteTime_get_fps();
    #endregion

}
#region internal type wrappers

#endregion