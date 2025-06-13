# Node
class in [FyroxLite](../../scripting_api.md).[Node](../Node.md)

## Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `Name` | string | get |  |
| `Alive` | bool | get |  |
| `GlobalPosition` | [Vector3](../Math/Vector3.md) | get |  |
| `LocalPosition` | [Vector3](../Math/Vector3.md) | get / set |  |
| `LocalRotation` | [Quaternion](../Math/Quaternion.md) | get / set |  |
| `Valid` | bool | get |  |
| `Parent` | [Node](../Node/Node.md) | get |  |
| `GlobalRotation` | [Quaternion](../Math/Quaternion.md) | get |  |
| `Tag` | string | get / set |  |

## Methods
| Return Type | Signature | Description |
|---|---|---|
| [RigidBody](../Physics/RigidBody.md)? | `AsRigidBody` (  ) |  |
| void | `Destroy` (  ) |  |
| void | `SendHierarchical` ( [RoutingStrategy](../Node/RoutingStrategy.md) <ins>routing</ins>, object <ins>payload</ins> ) | <p>Sends a hierarchical script message with the given payload.</p> |
| void | `SubscribeTo` <`T`> (  ) |  |
| [Node](../Node/Node.md)? | `FindColliderInChildren` (  ) |  |
| `T` | `AddScript` <`T`> (  ) |  |
| `T`? | `FindScript` <`T`> (  ) |  |
| bool | `TagIs` ( string <ins>tag</ins> ) |  |
