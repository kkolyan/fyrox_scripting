using System.Reflection;

namespace FyroxLite;

public static class ScriptsMetadataExtractor
{
    internal static void ScanAssemblyForScripts(Assembly assembly, List<NativeScriptMetadata> scripts, List<string> errors)
    {
        NativeClassId.Clear();
        PropertySetters.Clear();

        Console.WriteLine($"scanning assembly {assembly}");
        foreach (var type in assembly.GetTypes())
        {
            if (!type.IsClass || type.IsAbstract)
            {
                continue;
            }

            if (type.BaseType.Name == "GlobalScript")
            {
                Console.WriteLine($"registering global script {type.FullName}");

                // var uuidAttr = type.GetCustomAttribute<UuidAttribute>();
                // if (uuidAttr == null)
                // {
                //     throw new Exception($"Invalid script {type}: [Uuid] attribute required for Node scripts");
                // }
                //
                // var uuid = uuidAttr.Uuid;
                var uuid = Guid.NewGuid();
                RegisterScript(type, scripts, uuid, NativeScriptKind.Global, errors);
            }

            if (type.BaseType.Name == "NodeScript")
            {
                Console.WriteLine($"registering node script {type.FullName}");

                var uuidAttr = type.GetCustomAttribute<UuidAttribute>();
                if (uuidAttr == null)
                {
                    errors.Add($"Invalid script {type}: [Uuid] attribute required for Node scripts");
                }

                RegisterScript(type, scripts, uuidAttr?.Uuid, NativeScriptKind.Node, errors);
            }
        }
    }

    private static void RegisterScript(Type type, List<NativeScriptMetadata> scripts, Guid? uuid, NativeScriptKind kind, List<string> errors)
    {
        try
        {
            FyroxImpls.CreateInstance(type);
        }
        catch (MissingMethodException e)
        {
            errors.Add(e.Message);
        }
        
        var properties = new List<NativeScriptProperty>();
        var propertySetters = new Dictionary<string, (NativeValueType, PropertySetters.SetPropertyDelegate)>();

        var fieldInfos = type.GetFields(BindingFlags.Instance | BindingFlags.NonPublic | BindingFlags.Public);
        foreach (var field in fieldInfos)
        {
            var hideInInspector = kind == NativeScriptKind.Global ||
                                  field.GetCustomAttribute<HideInInspectorAttribute>() != null;
            var transient = field.GetCustomAttribute<TransientAttribute>() != null;
            Console.WriteLine($"  registering property {field.Name}. transient: {transient}, hidden: {hideInInspector}");
            if (hideInInspector && transient)
            {
                continue;
            }

            var ft = ExtractFieldType(field, errors);
            if (ft == null)
            {
                continue;
            }

            var (fieldType, fieldSetter) = ft.Value;
            properties.Add(new NativeScriptProperty
            {
                id = properties.Count,
                name = NativeString.FromFacade(field.Name),
                ty = fieldType,
                hide_in_inspector = NativeBool.FromFacade(hideInInspector),
                transient = NativeBool.FromFacade(transient),
            });
            propertySetters.Add(field.Name, (fieldType, (o, value) => fieldSetter!(o, value)));
        }

        PropertySetters.Register(type, propertySetters);

        var classId = scripts.Count + 1;
        if (classId > NativeClassId.MaxScriptClassCount)
        {
            errors.Add(
                $"WTF: attempt to use more than {NativeClassId.MaxScriptClassCount} script classes. " +
                $"{NativeClassId.MaxScriptClassCount} seemed to be MORE THAN reasonable limitation. " +
                $"Please report your case to developer.");
            return; 
        }

        if (errors.Count > 0)
        {
            return; 
        }

        var metadata = new NativeScriptMetadata
        {
            id = new NativeClassId(classId),
            uuid = NativeString.FromFacade(uuid.ToString()),
            kind = kind,
            name = NativeString.FromFacade(type.Name),
            has_global_on_init = HasDeclaredMethod(type, nameof(GlobalScript.OnGlobalInit), [typeof(string)]),
            has_global_on_update = HasDeclaredMethod(type, nameof(GlobalScript.OnGlobalUpdate), []),
            has_node_on_init = HasDeclaredMethod(type, nameof(NodeScript.OnInit), []),
            has_node_on_start = HasDeclaredMethod(type, nameof(NodeScript.OnStart), []),
            has_node_on_deinit = HasDeclaredMethod(type, nameof(NodeScript.OnDeinit), []),
            has_node_on_update = HasDeclaredMethod(type, nameof(NodeScript.OnUpdate), [typeof(float)]),
            has_node_on_message = HasDeclaredMethod(type, nameof(NodeScript.OnMessage), [typeof(object)]),
            properties = NativeScriptProperty_slice.FromFacade(properties),
        };
        NativeClassId.Register(type, metadata.id);
        scripts.Add(metadata);
    }

    private delegate void SetField(object o, NativeValue value);

    private static (NativeValueType, SetField)? ExtractFieldType(FieldInfo field, List<string> errors)
    {
        // @formatter:off
        
        // value types
        
        var ownerType = field.DeclaringType;
        var type = field.FieldType;
        
        const string hint = "Mark it with [Transient] and [HideInInspector] if you still want to use it without persistence and inspecting support.";
        
        if (field.IsInitOnly) {
            errors.Add($"Field `{ownerType!.FullName}::{field.Name}` is readonly. {hint}");
        }
        
        if (type == typeof(bool)) return (NativeValueType.@bool, (o, value) => field.SetValue(o, NativeBool.ToFacade(value.@bool)));
        if (type == typeof(float)) return (NativeValueType.f32, (o, value) => field.SetValue(o, value.f32));
        if (type == typeof(double)) return (NativeValueType.f64, (o, value) => field.SetValue(o, value.f64));
        if (type == typeof(short)) return (NativeValueType.i16, (o, value) => field.SetValue(o, value.i16));
        if (type == typeof(int)) return (NativeValueType.i32, (o, value) => field.SetValue(o, value.i32));
        if (type == typeof(long)) return (NativeValueType.i64, (o, value) => field.SetValue(o, value.i64));
        if (type == typeof(string)) return (NativeValueType.String, (o, value) => field.SetValue(o, NativeString.ToFacade(value.String)));
        if (type == typeof(Vector3)) return (NativeValueType.Vector3, (o, value) => field.SetValue(o, NativeVector3.ToFacade(value.Vector3)));
        if (type == typeof(Quaternion)) return (NativeValueType.Quaternion, (o, value) => field.SetValue(o, NativeQuaternion.ToFacade(value.Quaternion)));
        if (type == typeof(Prefab)) return (NativeValueType.Prefab, (o, value) => field.SetValue(o, new Prefab(value.Handle)));
        
        // @formatter:on

        // node-backed types, prefabs
        var fromHandle = type.GetConstructor(BindingFlags.Public | BindingFlags.NonPublic | BindingFlags.Instance,
            [typeof(NativeHandle)]);
        if (fromHandle != null)
        {
            return (NativeValueType.Handle, (o, value) => field.SetValue(o, fromHandle.Invoke([value.Handle])));
        }
        
        errors.Add($"Field `{ownerType!.FullName}::{field.Name}` ({type}) has unsupported field type. {hint}");

        if (errors.Count > 0)
        {
            return null;
        }

        return (default, null);
    }

    private static NativeBool HasDeclaredMethod(Type type, string name, Type[] paramTypes)
    {
        var method = type.GetMethod(name,
            BindingFlags.DeclaredOnly | BindingFlags.Instance | BindingFlags.NonPublic,
            paramTypes);
        return NativeBool.FromFacade(method != null);
    }
}