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

// fyrox_lite::lite_physics::LiteFeatureKind

public enum FeatureKind
{
    
    /// <para>Shape-dependent identifier of a vertex.</para>
    Vertex,
    
    /// <para>Shape-dependent identifier of an edge.</para>
    Edge,
    
    /// <para>Shape-dependent identifier of a face.</para>
    Face,
    
    /// <para>Unknown identifier.</para>
    Unknown,
}