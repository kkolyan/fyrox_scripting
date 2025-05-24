
using System.Reflection;

namespace FyroxLite;

internal static class ScriptsMetadataManager
{
    internal static Assembly? PlayerAssembly;
    internal static string? EditorWorkingDir;
    private static FyroxLoadContext? _loadContext;

    internal static NativeString_optional GetScriptsAssemblyPath()
    {
        return NativeString_optional.FromFacade(GetScriptsAssemblyPathInternal());
    }

    internal static string? GetScriptsAssemblyPathInternal()
    {
        if (EditorWorkingDir == null)
        {
            return null;
        }

        return $"{EditorWorkingDir}/bin/Debug/net8.0/{Path.GetFileName(EditorWorkingDir)}.dll";
    }
    
    internal static NativeScriptMetadata_slice GetScriptsMetadata()
    {
        List<NativeScriptMetadata> scripts;
        if (EditorWorkingDir != null)
        {
            if (_loadContext != null)
            {
                _loadContext.Unload();
                GC.Collect();
                GC.WaitForPendingFinalizers();
                _loadContext = null;
            }

            var assemblyPath = GetScriptsAssemblyPathInternal();
            Console.WriteLine($"Loading game scripts assembly file: {assemblyPath}");
            if (Path.Exists(assemblyPath))
            {
                _loadContext = new FyroxLoadContext();
                var assemblyBytes = File.ReadAllBytes(assemblyPath);
                var assembly = _loadContext.LoadFromStream(new MemoryStream(assemblyBytes));
                scripts = ScriptsMetadataExtractor.ScanAssemblyForScripts(assembly);     
                // do not lock file. we don't need classes, actually in editor.
                _loadContext.Unload();
                GC.Collect();
                GC.WaitForPendingFinalizers();
                _loadContext = null;           
            }
            else
            {
                scripts = new List<NativeScriptMetadata>();
            }
        }
        else
        {
            scripts = ScriptsMetadataExtractor.ScanAssemblyForScripts(PlayerAssembly);
        }

        return NativeScriptMetadata_slice.FromFacade(scripts);
    }
}