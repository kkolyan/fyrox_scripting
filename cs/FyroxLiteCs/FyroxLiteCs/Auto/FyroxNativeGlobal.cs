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

using System.Runtime.InteropServices;
using FyroxLite;

internal partial class FyroxNativeGlobal {
    
    #region native internal method

    [LibraryImport("fyrox_lite_cs", StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    internal static partial void init_fyrox(NativeScriptAppFunctions app);
    #endregion

    internal delegate NativeString_optional GetScriptsAssemblyPath();

    internal delegate NativeScriptMetadata_slice GetScriptsMetadata();

    internal delegate void_result NodeOnUpdate(NativeInstanceId thiz, float dt);

    internal delegate void_result NodeOnInit(NativeInstanceId thiz);

    internal delegate void_result NodeOnDeinit(NativeInstanceId thiz);

    internal delegate void_result NodeOnStart(NativeInstanceId thiz);

    internal delegate void_result NodeOnMessage(NativeInstanceId thiz, UserScriptMessage message);

    internal delegate void_result GameOnInit(NativeInstanceId thiz, NativeString_optional initial_scene_override);

    internal delegate void_result GameOnUpdate(NativeInstanceId thiz);

    internal delegate NativeInstanceId_result CreateScriptInstance(NativeClassId thiz, NativePropertyValue_slice state, NativeHandle_optional node);

    internal delegate void DisposeMessage(UserScriptMessage message);

    internal delegate void DisposeScript(NativeInstanceId script);
}