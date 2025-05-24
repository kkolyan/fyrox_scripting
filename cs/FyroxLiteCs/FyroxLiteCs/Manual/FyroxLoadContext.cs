using System.Reflection;
using System.Runtime.Loader;

namespace FyroxLite;

internal class FyroxLoadContext: AssemblyLoadContext
{
    internal FyroxLoadContext() : base(isCollectible: true)
    {
    }
    
    protected override Assembly Load(AssemblyName assemblyName) => null;
}