
// ReSharper disable InconsistentNaming
public class CsFile
{
    public List<CsClass> classes;
    public List<CsEnum> enums;
}
public class CsEnum
{
    public string name;
    public string ns;
    public List<CsEnumMember> members;
    public List<CsXmlNode> description;
}

public class CsEnumMember
{
    public string name;
    public List<CsXmlNode> description;
}

public class CsClass
{
    public string name;
    public string ns;
    public bool is_struct;
    public List<CsMethod> methods;
    public List<CsMethod> operators;
    public List<CsConstructor> constructors;
    public List<CsField> fields;
    public List<CsProperty> properties;
    public List<CsXmlNode> description;
}

public class CsMethod
{
    public string name;
    public bool is_static;
    public CsType return_ty;
    public List<CsParam> parameters;
    public List<CsXmlNode> description;
}

public class CsConstructor
{
    public List<CsParam> parameters;
    public List<CsXmlNode> description;
}

public class CsField
{
    public string name;
    public bool is_static;
    public bool is_const;
    public CsType ty;
    public List<CsXmlNode> description;
    public string? initializer;
}

public class CsParam
{
    public string name;
    public CsType ty;
}

public class CsProperty
{
    public string name;
    public bool is_static;
    public CsType ty;
    public bool get;
    public bool set;
    public string? expression;
    public List<CsXmlNode> description;
}

public class CsType
{
    public string name;
    public List<CsType> args;
}

public class CsXmlNode
{
    public string text;
    public CsXmlElement? element;
    public object unknown;

    public override string ToString()
    {
        if (text != null)
        {
            return $"TextNode({text})";
        }

        if (unknown != null)
        {
            return $"UnknownNode({unknown})";
        }

        return $"Element({element.name})";
    }
}

public class CsXmlElement
{
    public string name;
    public List<CsXmlNode> children;
    public Dictionary<string, string> attrs;
}
