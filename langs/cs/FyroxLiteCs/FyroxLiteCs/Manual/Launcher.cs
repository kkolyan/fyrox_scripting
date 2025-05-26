using System.Numerics;
using System.Reflection;
using System.Runtime.InteropServices;

namespace FyroxLite;

public partial class Launcher
{
    
    [LibraryImport(FyroxDll.Name, EntryPoint = "fyrox_lite_executor_run",
        SetLastError = true)]
    private static partial void RunInternal();

    [LibraryImport(FyroxDll.Name, EntryPoint = "fyrox_lite_editor_run",
        SetLastError = true)]
    private static partial void RunEditorInternal(IntPtr workingDirectory, IntPtr assemblyPath);

    public static void RunGame()
    {
        /*
        necessary to avoid following crash on Windows:
        thread 'main' panicked at 'OleInitialize failed! Result was: `RPC_E_CHANGED_MODE`.
        Make sure other crates are not using multithreaded COM library on the same thread or disable drag and drop support.'

        Actually, this can be solved by `[STAThread]` over the Main method, but that's on user side, so let's keep user from such crucial things.
        */

        var thread = new Thread(() => Run(editor: false, editorWorkingDir: null, playerAssembly: Assembly.GetEntryAssembly()));
        if (OperatingSystem.IsWindows())
        {
            thread.SetApartmentState(ApartmentState.STA);
        }

        thread.Start();
        thread.Join();
    }

    internal static void Run(bool editor, string? editorWorkingDir, Assembly? playerAssembly)
    {
        ObjectRegistry.InitThread();
        NativeClassId.InitThread();
        PropertySetters.InitThread();

        if (editorWorkingDir == null && playerAssembly == null)
        {
            throw new Exception($"either of {nameof(editorWorkingDir)} or {playerAssembly} should be supplied");
        }

        ScriptsMetadataManager.EditorWorkingDir = editorWorkingDir == null ? null : Path.GetFullPath(editorWorkingDir);
        ScriptsMetadataManager.PlayerAssembly = playerAssembly;

        Console.WriteLine("initializing callbacks");

        FyroxNativeGlobal.init_fyrox_lite(
            new NativeScriptAppFunctions
            {
                get_scripts_metadata = ScriptsMetadataManager.GetScriptsMetadata,
                on_init = FyroxImpls.on_init,
                on_start = FyroxImpls.on_start,
                on_deinit = FyroxImpls.on_deinit,
                on_update = FyroxImpls.on_update,
                on_message = FyroxImpls.on_message,
                on_game_init = FyroxImpls.on_game_init,
                on_game_update = FyroxImpls.on_game_update,
                create_script_instance = FyroxImpls.create_script_instance,
                dispose_message = FyroxImpls.dispose_message,
                dispose_script = FyroxImpls.dispose_script,
            }, 
            is_editor: NativeBool.FromFacade(editor)
        );
        Console.WriteLine("running main loop");

        if (editor)
        {
            var fullPath = Path.GetFullPath(editorWorkingDir);
            Console.WriteLine($"Working directory: {fullPath}");
            var workingDirCString = Marshal.StringToHGlobalAnsi(fullPath);
            var assemblyPath = Marshal.StringToHGlobalAnsi(ScriptsMetadataManager.GetScriptsAssemblyPathInternal());
            RunEditorInternal(workingDirCString, assemblyPath: assemblyPath);
            Marshal.FreeHGlobal(workingDirCString);
        }
        else
        {
            RunInternal();
        }
    }
}