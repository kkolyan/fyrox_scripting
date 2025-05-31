
using System.Reflection;

namespace FyroxLite;

internal static class ScriptsMetadataManager
{
    internal static Assembly? PlayerAssembly;
    internal static string? EditorWorkingDir;
    private static FyroxLoadContext? _loadContext;

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
            Console.WriteLine($"{nameof(EditorWorkingDir)} defined, scanning external assembly");
            if (_loadContext != null)
            {
                _loadContext.Unload();
                GC.Collect();
                GC.WaitForPendingFinalizers();
                _loadContext = null;
            }
            
            Console.WriteLine($"Current path is {Path.GetFullPath(".")}");

            var assemblyPath = GetScriptsAssemblyPathInternal();
            Log.Info($"Loading C# assembly file at {assemblyPath}");
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
                Log.Err("============================");
                Log.Err("C# assembly file not found.");
                Log.Err("Please compile your C# project.");
                Log.Err("Until then You may see obscure errors in console, and some Fyrox function may not work.");
                Log.Err("============================");
                
                scripts = new List<NativeScriptMetadata>();
            }
        }
        else
        {
            Console.WriteLine($"{nameof(EditorWorkingDir)} is not defined, scanning own assembly");
            scripts = ScriptsMetadataExtractor.ScanAssemblyForScripts(PlayerAssembly);
        }

        Console.WriteLine($"Returning {scripts.Count} scripts");
        return NativeScriptMetadata_slice.FromFacade(scripts);
    }
}