public class CsClass
{
    public string name;
    public List<CsMethod> methods;
    public List<CsMethod> operators;
    public List<CsParam> fields;
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

public class CsParam
{
    public string name;
    public CsType ty;
}

public class CsProperty
{
    public string name;
    public CsType ty;
    public bool get;
    public bool set;
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
    public string element_name;
    public List<CsXmlNode> element_children;
    public Dictionary<string, string> element_attrs;
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

        return $"Element({element_name})";
    }
}
