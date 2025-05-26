namespace FyroxLite;

public static class FyroxEd
{
    public static void RunEditor(string? editorWorkingDir)
    {
        
        Launcher.Run(editor: true, editorWorkingDir: editorWorkingDir, playerAssembly: null);
    }
}