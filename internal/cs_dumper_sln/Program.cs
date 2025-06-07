using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Syntax;
using System.Text.Json;
using System.Text.Json.Serialization;


class Program
{
    static void Main(string[] args)
    {
        if (args.Length < 1 || !File.Exists(args[0]))
        {
            Console.Error.WriteLine("Usage: ParseCSharp <file.cs>");
            Environment.Exit(1);
        }

        var code = File.ReadAllText(args[0]);
        var tree = CSharpSyntaxTree.ParseText(code);
        var root = tree.GetCompilationUnitRoot();

        var classes = new List<CsClass>();

        foreach (var cls in root.DescendantNodes().OfType<TypeDeclarationSyntax>())
        {
            var methods = new List<CsMethod>();

            foreach (var method in cls.Members.OfType<MethodDeclarationSyntax>())
            {
                methods.Add(new CsMethod
                {
                    name = method.Identifier.Text,
                    description = DocsExtract.ExtractDocs(method),
                });
            }

            var docs = DocsExtract.ExtractDocs(cls);
            classes.Add(new CsClass
            {
                name = cls.Identifier.Text,
                methods = methods,
                description = docs
            });
        }

        var json = JsonSerializer.Serialize(classes,
            new JsonSerializerOptions
            {
                WriteIndented = true,
                IncludeFields = true,
                DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
            });
        Console.WriteLine(json);
    }
}