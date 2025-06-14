using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Syntax;
using System.Text.Json;
using System.Text.Json.Serialization;
using Microsoft.CodeAnalysis;


class Program
{
    static void Main(string[] args)
    {
        if (args.Length < 1 || !File.Exists(args[0]))
        {
            Console.Error.WriteLine("Usage: ParseCSharp <input_file.cs> [output_file.json]");
            Environment.Exit(1);
        }

        string outputFile = null;
        if (args.Length > 1)
        {
            outputFile = args[1];
        }

        var code = File.ReadAllText(args[0]);
        var tree = CSharpSyntaxTree.ParseText(code);
        var root = tree.GetCompilationUnitRoot();

        var classes = new List<CsClass>();
        var enums = new List<CsEnum>();
        
        foreach (var nsDecl in root.ChildNodes().OfType<NamespaceDeclarationSyntax>())
        {
            var ns = nsDecl.Name.ToString();
            ExtractEnums(nsDecl, enums, ns);
            ExtractClasses(nsDecl, classes, ns);
        }
        
        foreach (var nsDecl in root.ChildNodes().OfType<FileScopedNamespaceDeclarationSyntax>())
        {
            var ns = nsDecl.Name.ToString();
            ExtractEnums(nsDecl, enums, ns);
            ExtractClasses(nsDecl, classes, ns);
        }

        ExtractEnums(root, enums, "");

        ExtractClasses(root, classes, "");

        var json = JsonSerializer.Serialize(new CsFile
            {
                classes = classes,
                enums = enums
            },
            new JsonSerializerOptions
            {
                WriteIndented = true,
                IncludeFields = true,
                DefaultIgnoreCondition = JsonIgnoreCondition.WhenWritingNull
            });
        if (outputFile == null)
        {
            Console.WriteLine(json);
        }
        else
        {
            File.WriteAllText(outputFile, json);
        }
    }

    private static void ExtractClasses(CSharpSyntaxNode root, List<CsClass> classes, string ns)
    {
        foreach (var cls in root.ChildNodes().OfType<TypeDeclarationSyntax>())
        {
            var methods = new List<CsMethod>();

            foreach (var method in cls.Members.OfType<MethodDeclarationSyntax>())
            {
                if (!HasModifier(method.Modifiers, "public"))
                {
                    continue;
                }
                methods.Add(new CsMethod
                {
                    name = method.Identifier.Text,
                    description = DocsExtract.ExtractDocs(method),
                    is_static = method.Modifiers.Any(it => it.ToString() == "static"),
                    parameters = method.ParameterList.Parameters.Select(it => new CsParam
                    {
                        name = it.Identifier.ToString(),
                        ty = ExtractType(it.Type),
                    }).ToList(),
                    return_ty = ExtractType(method.ReturnType),
                });
            }
            
            var constructors = new List<CsConstructor>();

            foreach (var member in cls.Members.OfType<ConstructorDeclarationSyntax>())
            {
                if (!HasModifier(member.Modifiers, "public"))
                {
                    continue;
                }
                constructors.Add(new CsConstructor
                {
                    description = DocsExtract.ExtractDocs(member),
                    parameters = member.ParameterList.Parameters.Select(it => new CsParam
                    {
                        name = it.Identifier.ToString(),
                        ty = ExtractType(it.Type),
                    }).ToList(),
                });
            }

            var operators = new List<CsMethod>();

            foreach (var op in cls.Members.OfType<OperatorDeclarationSyntax>())
            {
                if (!HasModifier(op.Modifiers, "public"))
                {
                    continue;
                }
                operators.Add(new CsMethod
                {
                    name = op.OperatorToken.Text,
                    description = DocsExtract.ExtractDocs(op),
                    is_static = op.Modifiers.Any(it => it.ToString() == "static"),
                    parameters = op.ParameterList.Parameters.Select(it => new CsParam
                    {
                        name = it.Identifier.ToString(),
                        ty = ExtractType(it.Type),
                    }).ToList(),
                    return_ty = ExtractType(op.ReturnType),
                });
            }
           
            var fields = new List<CsField>();

            foreach (var field in cls.Members.OfType<FieldDeclarationSyntax>())
            {
                if (!HasModifier(field.Modifiers, "public"))
                {
                    continue;
                }
                fields.Add(new CsField
                {
                    name = field.Declaration.Variables.Single().Identifier.ToString(),
                    description = DocsExtract.ExtractDocs(field),
                    is_static = field.Modifiers.Any(it => it.ToString() == "static"),
                    is_const = field.Modifiers.Any(it => it.ToString() == "const"),
                    ty = ExtractType(field.Declaration.Type),
                    initializer = field.Declaration.Variables.Single().Initializer?.Value.ToString()
                });
            }

            var props = new List<CsProperty>();

            foreach (var prop in cls.Members.OfType<PropertyDeclarationSyntax>())
            {
                if (!HasModifier(prop.Modifiers, "public"))
                {
                    continue;
                }

                var accessorList = prop.AccessorList;
                props.Add(new CsProperty
                {
                    name = prop.Identifier.Text,
                    description = DocsExtract.ExtractDocs(prop),
                    is_static = prop.Modifiers.Any(it => it.ToString() == "static"),
                    ty = ExtractType(prop.Type),
                    get = accessorList?.Accessors.Any(it => it.Keyword.ToString() == "get") ?? true,
                    set = accessorList?.Accessors.Any(it => it.Keyword.ToString() == "set") ?? false,
                });
            }

            classes.Add(new CsClass
            {
                is_struct = cls is StructDeclarationSyntax,
                ns = ns,
                name = cls.Identifier.Text,
                methods = methods,
                properties = props,
                operators = operators,
                description = DocsExtract.ExtractDocs(cls),
                fields = fields,
                constructors = constructors,
            });
        }
    }

    private static bool HasModifier(SyntaxTokenList modifiers, string modifier)
    {
        return modifiers.Any(it => it.ToString() == modifier);
    }

    private static void ExtractEnums(CSharpSyntaxNode root, List<CsEnum> enums, string ns)
    {
        foreach (var cls in root.ChildNodes().OfType<EnumDeclarationSyntax>())
        {
            enums.Add(new CsEnum
            {
                ns = ns,
                name = cls.Identifier.Text,
                members = cls.Members.Select(it => new CsEnumMember
                {
                    name = it.Identifier.Text,
                    description = DocsExtract.ExtractDocs(it),
                }).ToList(),
                description = DocsExtract.ExtractDocs(cls),
            });
        }
    }

    private static CsType ExtractType(TypeSyntax type)
    {
        return type switch
        {
            ArrayTypeSyntax it => new CsType { name = "[]", args = [ExtractType(it.ElementType)] },
            // AliasQualifiedNameSyntax it => new CsType {name = ""},
            // FunctionPointerTypeSyntax it => new CsType {name = ""},
            // GenericNameSyntax it => new CsType {name = ""},
            IdentifierNameSyntax it => new CsType { name = it.Identifier.Text },
            // QualifiedNameSyntax it => new CsType {name = ""},
            SimpleNameSyntax it => new CsType { name = it.Identifier.Text },
            // NameSyntax it => new CsType {name = ""},
            NullableTypeSyntax it => new CsType { name = "?", args = [ExtractType(it.ElementType)] },
            // OmittedTypeArgumentSyntax it => new CsType {name = ""},
            // PointerTypeSyntax it => new CsType {name = ""},
            PredefinedTypeSyntax it => new CsType { name = it.Keyword.ToString() },
            RefTypeSyntax it => new CsType {name = "ref", args = [ExtractType(it.Type)]},
            // ScopedTypeSyntax it => new CsType {name = ""},
            TupleTypeSyntax it => new CsType {name = type.ToString()},
            _ => throw new NotImplementedException(type.ToString()),
        };
    }
}