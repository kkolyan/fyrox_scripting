using System.Reflection;
using System.Runtime.Loader;

namespace FyroxLite;

internal class FyroxLoadContext: AssemblyLoadContext
{
    internal FyroxLoadContext() : base(isCollectible: true)
    {
    }
    
    protected override Assembly Load(AssemblyName assemblyName)
    {
        Console.WriteLine($"FyroxLoadContext: attempt to load {assemblyName}");
        if (assemblyName.Name == "FyroxLiteCs")
        {
            var assembly = typeof(NodeScript).Assembly;
            Console.WriteLine($"FyroxLoadContext: replacing it with assembly {assembly}");
            return assembly;
        } 
        return null;
    }
}