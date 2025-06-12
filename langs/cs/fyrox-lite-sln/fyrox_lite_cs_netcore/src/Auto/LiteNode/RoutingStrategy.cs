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

// fyrox_lite::lite_node::LiteRoutingStrategy

public enum RoutingStrategy
{
    
    /// <para>An message will be passed to the specified root node and then to every node up in the hierarchy.</para>
    Up,
    
    /// <para>An message will be passed to every node down the tree in the hierarchy.</para>
    Down,
}