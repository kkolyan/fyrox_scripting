using System.Runtime.InteropServices;
using FyroxLite;

public partial class Program
{
    
    [LibraryImport("fyroxed_cs", EntryPoint = "ask_user_for_project_directory",
        SetLastError = true)]
    private static partial IntPtr AskUserForProjectDirectory();
    
    [STAThread]
    public static void Main(string[] args)
    {
        string? workingDir;
        if (args.Length < 1)
        {
            var projectDirectoryPtr = AskUserForProjectDirectory();
            if (projectDirectoryPtr == IntPtr.Zero)
            {
                Console.WriteLine("user aborted project directory selection");
                return;
            }
            workingDir = Marshal.PtrToStringAnsi(projectDirectoryPtr);
        }
        else
        {
            workingDir = args[0];
        }
        FyroxEd.RunEditor(workingDir);
    }
}
