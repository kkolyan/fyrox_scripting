using Microsoft.CodeAnalysis;
using Microsoft.CodeAnalysis.CSharp;
using Microsoft.CodeAnalysis.CSharp.Syntax;

public class DocsExtract
{
    public static List<CsXmlNode> ExtractDocs(CSharpSyntaxNode cls)
    {
        var docs = cls.GetLeadingTrivia()
            .Where(it =>
                it.IsKind(SyntaxKind.SingleLineDocumentationCommentTrivia) ||
                it.IsKind(SyntaxKind.MultiLineDocumentationCommentTrivia))
            .SelectMany(it =>
            {
                if (it.HasStructure)
                {
                    return ToDto(it.GetStructure().ChildNodes().GetEnumerator()).DropNulls().ToList();
                }

                throw new NotImplementedException();
            })
            .ToList();
        return docs;
    }
    

    private static IEnumerable<CsXmlNode?> ToDto<T>(IEnumerator<T> stream)
        where T : SyntaxNode
    {
        while (true)
        {
            SyntaxNode? next = stream.Next();
            if (next == null)
            {
                yield break;
            }

            if (next is XmlNodeSyntax node)
            {
                if (node is XmlElementSyntax e)
                {
                    yield return new CsXmlNode
                    {
                        element_name = e.StartTag.Name.ToString(),
                        element_children = ToDto(new SyntaxListEnumeratorAdapter<XmlNodeSyntax>(e.Content.GetEnumerator())).DropNulls().ToList()
                    };
                    continue;
                }

                if (node is XmlTextSyntax t)
                {
                    yield return new CsXmlNode { text = t.ToString() };
                    continue;
                }

                if (node is XmlEmptyElementSyntax empty)
                {
                    var attrsDict = new Dictionary<string, string>();
                    foreach (var attr in empty.Attributes)
                    {
                        string value;
                        if (attr is XmlCrefAttributeSyntax cref)
                        {
                            if (cref.Cref is NameMemberCrefSyntax member)
                            {
                                value = member.Name.ToString();
                            }
                            else if (cref.Cref is QualifiedCrefSyntax q)
                            {
                                value = q.ToString();
                            }
                            else throw new NotImplementedException($"unknown cref type: {cref.Cref}");
                        }
                        else if (attr is XmlNameAttributeSyntax name)
                        {
                            value = name.Name.ToString();
                        }
                        else if (attr is XmlTextAttributeSyntax text)
                        {
                            value = text.TextTokens.ToString();
                        }
                        else throw new NotImplementedException($"unknown attribute type: {attr}");

                        attrsDict.Add(attr.Name.ToString(), value);
                    }

                    yield return new CsXmlNode
                    {
                        element_name = empty.Name.ToString(),
                        element_attrs = attrsDict,
                    };
                    continue;
                }

                // throw new NotImplementedException(node.ToString());
            }
            
            yield return Unknown(next);
            continue;
        }
    }

    private static CsXmlNode Unknown(object n)
    {
        throw new NotImplementedException();
        // return new CsXmlNode { unknown = n };
    }
}