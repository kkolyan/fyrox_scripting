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

// fyrox_lite::lite_window::LiteWindow

public static partial class Window
{
    
    public static CursorGrabMode CursorGrab
    {
        set
        {
            #region native call
            unsafe {
                var _value = value;
                fyrox_lite_lite_window_LiteWindow_set_cursor_grab(_value);
            }
            #endregion
        }
    }

    #region native internal methods

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial void fyrox_lite_lite_window_LiteWindow_set_cursor_grab(CursorGrabMode mode);
    #endregion

}
#region internal type wrappers

#endregion