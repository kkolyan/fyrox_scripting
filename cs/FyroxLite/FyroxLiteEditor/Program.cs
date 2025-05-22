using FyroxLite;

public class Program
{
    public static void Main(string[] args)
    {
        if (args.Length < 1)
        {
            Console.WriteLine("Working dir not selected");
            Environment.Exit(-1);
            return;
        }

        var workingDir = args[0];
        FyroxExecutor.RunEditor(workingDir);
    }
}
