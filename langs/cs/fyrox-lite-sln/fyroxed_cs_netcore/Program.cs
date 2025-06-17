using System.Reflection;
using System.Runtime.InteropServices;

public partial class Program
{
    [LibraryImport("fyroxed_cs", EntryPoint = "ask_user_for_project_directory",
        SetLastError = true)]
    private static partial IntPtr AskUserForProjectDirectory();
    
    [LibraryImport("fyroxed_cs", EntryPoint = "prepare_project_directory",
        SetLastError = true)]
    private static partial int PrepareProjectDirectory(IntPtr workingDir, int isCli);

    [STAThread]
    public static void Main(string[] args)
    {
        string? workingDir;
        bool isCli;
        if (args.Length < 1)
        {
            // There are usability issues on Macos, and on Linux probably too.
            // Let's support UI selector for Windows only
            if (!OperatingSystem.IsWindows())
            {
                Console.WriteLine("usage: ./fyroxed_cs_netcore <path to the project>");
                return;
            }
            isCli = false;
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
            isCli = true;
            workingDir = Path.GetFullPath(args[0]).TrimEnd('/').TrimEnd('\\');
        }

        {
            var projectDirectoryPtr = Marshal.StringToHGlobalAnsi(workingDir);
            var result = PrepareProjectDirectory(projectDirectoryPtr, isCli ? 1 : 0);
            if (result == 0)
            {
                Console.WriteLine("failed to prepare project directory");
                return;
            }
        }

        RunEditor(workingDir, isCli);
    }

    private static void RunEditor(string? workingDir, bool isCli)
    {
        var executableDir = Path.GetDirectoryName(Assembly.GetExecutingAssembly().Location);
        var assemblyPath = $"{executableDir}/fyrox_lite_cs_netcore_4editor.dll";
        var entryClassName = "FyroxLite.FyroxEd";
        var entryPointClass = Assembly
                                  .LoadFile(assemblyPath)
                                  .GetType(entryClassName)
                              ?? throw new Exception($"class {entryClassName} not found in {assemblyPath}");
        var entryMethodName = "RunEditor";
        var entryMethod = entryPointClass.GetMethod(entryMethodName)
                          ?? throw new Exception($"method {entryMethodName} not found in class {entryClassName}");
        entryMethod.Invoke(null, [workingDir, isCli]);
    }
}