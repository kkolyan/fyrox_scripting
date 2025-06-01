using System.Reflection;
using System.Runtime.InteropServices;

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

        RunEditor(workingDir);
    }

    private static void RunEditor(string? workingDir)
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
        entryMethod.Invoke(null, [workingDir]);
    }
}