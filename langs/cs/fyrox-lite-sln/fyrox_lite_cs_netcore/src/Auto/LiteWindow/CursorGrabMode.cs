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

// fyrox_lite::lite_window::LiteCursorGrabMode

public enum CursorGrabMode
{
    
    /// <para>No grabbing of the cursor is performed.</para>
    None,
    
    /// <para>The cursor is confined to the window area.</para><para>There’s no guarantee that the cursor will be hidden. You should hide it by yourself if you
    /// want to do so.</para><para><b>Platform-specific</b></para><ul>
    /// <li><b>macOS:</b> Not implemented. Always returns [<c>ExternalError::NotSupported</c>] for now.</li>
    /// <li><b>iOS / Android / Web:</b> Always returns an [<c>ExternalError::NotSupported</c>].</li>
    /// </ul>
    Confined,
    
    /// <para>The cursor is locked inside the window area to the certain position.</para><para>There’s no guarantee that the cursor will be hidden. You should hide it by yourself if you
    /// want to do so.</para><para><b>Platform-specific</b></para><ul>
    /// <li><b>X11 / Windows:</b> Not implemented. Always returns [<c>ExternalError::NotSupported</c>] for now.</li>
    /// <li><b>iOS / Android:</b> Always returns an [<c>ExternalError::NotSupported</c>].</li>
    /// </ul>
    Locked,
}