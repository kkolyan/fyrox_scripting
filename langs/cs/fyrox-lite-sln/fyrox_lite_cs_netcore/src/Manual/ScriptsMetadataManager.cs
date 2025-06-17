using System.Reflection;

namespace FyroxLite;

internal static class ScriptsMetadataManager
{
    private static List<NativeScriptMetadata>? _scripts;
    internal static Assembly? PlayerAssembly;
    internal static string? EditorWorkingDir;

    internal static string? GetScriptsAssemblyPathInternal()
    {
        if (EditorWorkingDir == null)
        {
            return null;
        }

        return $"{EditorWorkingDir}/bin/Debug/net9.0/{Path.GetFileName(EditorWorkingDir)}.dll";
    }

    internal static NativeScriptMetadata_slice GetScriptsMetadata()
    {
        Console.WriteLine($"Returning {_scripts!.Count} scripts");
        return NativeScriptMetadata_slice.FromFacade(_scripts!);
    }

    internal static void_result InitScriptsMetadata()
    {
        try
        {
            InitScriptsMetadataInternal();
            return void_result.FromFacade();
        }
        catch (Exception e)
        {
            return void_result.FromFacadeError(e.ToString());
        }
    }

    private static void InitScriptsMetadataInternal()
    {
        if (EditorWorkingDir == null)
        {
            Console.WriteLine($"{nameof(EditorWorkingDir)} is not defined, scanning own assembly");
            var scripts = new List<NativeScriptMetadata>();
            var errors = new List<string>();
            ScriptsMetadataExtractor.ScanAssemblyForScripts(PlayerAssembly!, scripts, errors);
            if (errors.Count > 0)
            {
                throw new StacklessException(string.Join("\n", errors));
            }
            _scripts = scripts;
            return;
        }

        Console.WriteLine($"{nameof(EditorWorkingDir)} defined, scanning external assembly");

        Console.WriteLine($"Current path is {Path.GetFullPath(".")}");

        var assemblyPath = GetScriptsAssemblyPathInternal();
        Log.Info($"Loading C# assembly file at {assemblyPath}");
        Console.WriteLine($"Loading game scripts assembly file: {assemblyPath}");
        if (!Path.Exists(assemblyPath))
        {
            throw new StacklessException("Assembly file not found. Please reopen editor. " +
                                         "If you haven't deleted assembly file, please contact Fyrox Lite maintainers.");
        }

        var loadContext = new FyroxLoadContext();
        try
        {
            var assemblyBytes = File.ReadAllBytes(assemblyPath);
            var assembly = loadContext.LoadFromStream(new MemoryStream(assemblyBytes));
            try
            {
                var scripts = new List<NativeScriptMetadata>();
                var errors = new List<string>();
                ScriptsMetadataExtractor.ScanAssemblyForScripts(assembly, scripts, errors);
                if (errors.Count > 0)
                {
                    throw new StacklessException(string.Join("\n", errors));
                }
                _scripts = scripts;
            }
            catch (Exception e)
            {
                throw new StacklessException(e.ToString());
            }
        }
        finally
        {
            try
            {
                // do not lock file. we don't need classes, actually in editor.
                loadContext.Unload();
                GC.Collect();
                GC.WaitForPendingFinalizers();
            }
            catch (Exception e)
            {
                Log.Warn($"Failed to dispose metadata loading state: {e}");
            }
        }
    }
}