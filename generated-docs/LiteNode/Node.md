# Node
class in FyroxLite.LiteNode
## Properties
| Name | Type | Access | Description |
|---|---|---|---|
| Name | string | get |  |
| Alive | bool | get |  |
| GlobalPosition | [Vector3](../LiteMath/Vector3.md) | get |  |
| LocalPosition | [Vector3](../LiteMath/Vector3.md) | get / set |  |
| LocalRotation | [Quaternion](../LiteMath/Quaternion.md) | get / set |  |
| Valid | bool | get |  |
| Parent | [Node](../LiteNode/Node.md) | get |  |
| GlobalRotation | [Quaternion](../LiteMath/Quaternion.md) | get |  |
| Tag | string | get / set |  |
## Methods
| Return Type | Signature | Description |
|---|---|---|
| [RigidBody](../LitePhysics/RigidBody.md)? | [AsRigidBody](##) (  ) |  |
| void | [Destroy](##) (  ) |  |
| void | [SendHierarchical](##) ( [RoutingStrategy](../LiteNode/RoutingStrategy.md) routing, object payload ) | Sends a hierarchical script message with the given payload. |
| void | [SubscribeTo](##) (  ) |  |
| [Node](../LiteNode/Node.md)? | [FindColliderInChildren](##) (  ) |  |
| T | [AddScript](##) <`T`> (  ) |  |
| T? | [FindScript](##) <`T`> (  ) |  |
| bool | [TagIs](##) ( string tag ) |  |

