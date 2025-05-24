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

// fyrox_lite::lite_log::LiteLog
public static partial class Log
{

    public static void Info(string msg)
    {
        #region native call
        unsafe {
            var _msg = NativeString.FromFacade(msg);
            fyrox_lite_lite_log_LiteLog_info(_msg);
        }
        #endregion
    }

    public static void Warn(string msg)
    {
        #region native call
        unsafe {
            var _msg = NativeString.FromFacade(msg);
            fyrox_lite_lite_log_LiteLog_warn(_msg);
        }
        #endregion
    }

    public static void Err(string msg)
    {
        #region native call
        unsafe {
            var _msg = NativeString.FromFacade(msg);
            fyrox_lite_lite_log_LiteLog_err(_msg);
        }
        #endregion
    }

    #region native internal methods

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_info(NativeString msg);

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_warn(NativeString msg);

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_log_LiteLog_err(NativeString msg);
    #endregion

}
#region internal type wrappers

#endregion