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

// fyrox_lite::lite_physics::LitePhysics

public static partial class Physics
{
    
    // <para>Exclude from the query any collider attached to a fixed rigid-body and colliders with no rigid-body attached.</para>
    //public const int EXCLUDE_FIXED = 1 << 1;
    
    // <para>Exclude from the query any collider attached to a kinematic rigid-body.</para>
    //public const int EXCLUDE_KINEMATIC = 1 << 2;
    
    // <para>Exclude from the query any collider attached to a dynamic rigid-body.</para>
    //public const int EXCLUDE_DYNAMIC = 1 << 3;
    
    // <para>Exclude from the query any collider that is a sensor.</para>
    //public const int EXCLUDE_SENSORS = 1 << 4;
    
    // <para>Exclude from the query any collider that is not a sensor.</para>
    //public const int EXCLUDE_SOLIDS = 1 << 5;
    
    // <para>Excludes all colliders not attached to a dynamic rigid-body.</para>
    //public const int ONLY_DYNAMIC = LitePhysics :: EXCLUDE_FIXED | LitePhysics :: EXCLUDE_KINEMATIC;
    
    // <para>Excludes all colliders not attached to a kinematic rigid-body.</para>
    //public const int ONLY_KINEMATIC = LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_FIXED;
    
    // <para>Exclude all colliders attached to a non-fixed rigid-body
    // (this will not exclude colliders not attached to any rigid-body).</para>
    //public const int ONLY_FIXED = LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_KINEMATIC;

    
    public static List<Intersection> CastRay(RayCastOptions opts)
    {
        #region native call
        unsafe {
            var _opts = opts;
            var __ret = fyrox_lite_lite_physics_LitePhysics_cast_ray(&_opts);
            return Intersection_slice.ToFacade(__ret);
        }
        #endregion
    }

    #region native internal methods

    [LibraryImport(FyroxDll.Name, StringMarshalling = StringMarshalling.Utf8, SetLastError = true)]
    private static unsafe partial Intersection_slice fyrox_lite_lite_physics_LitePhysics_cast_ray(RayCastOptions* opts);
    #endregion

}
#region internal type wrappers

#endregion