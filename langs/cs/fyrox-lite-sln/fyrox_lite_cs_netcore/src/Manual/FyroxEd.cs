namespace FyroxLite;

public static class FyroxEd
{
    public static void RunEditor(string? editorWorkingDir, bool isCli)
    {
        
        Launcher.Run(editor: true, editorWorkingDir: editorWorkingDir, playerAssembly: null, isCli: isCli);
    }
}