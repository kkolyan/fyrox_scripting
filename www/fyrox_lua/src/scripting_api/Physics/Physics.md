# Physics
class in [FyroxLite](../../scripting_api.md).[Physics](../Physics.md)
## Constants
| Name | Type | Value | Description |
|---|---|---|---|
| `EXCLUDE_FIXED` | int | 1 << 1 | Exclude from the query any collider attached to a fixed rigid-body and colliders with no rigid-body attached. |
| `EXCLUDE_KINEMATIC` | int | 1 << 2 | Exclude from the query any collider attached to a kinematic rigid-body. |
| `EXCLUDE_DYNAMIC` | int | 1 << 3 | Exclude from the query any collider attached to a dynamic rigid-body. |
| `EXCLUDE_SENSORS` | int | 1 << 4 | Exclude from the query any collider that is a sensor. |
| `EXCLUDE_SOLIDS` | int | 1 << 5 | Exclude from the query any collider that is not a sensor. |
| `ONLY_DYNAMIC` | int | LitePhysics :: EXCLUDE_FIXED | LitePhysics :: EXCLUDE_KINEMATIC | Excludes all colliders not attached to a dynamic rigid-body. |
| `ONLY_KINEMATIC` | int | LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_FIXED | Excludes all colliders not attached to a kinematic rigid-body. |
| `ONLY_FIXED` | int | LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_KINEMATIC | Exclude all colliders attached to a non-fixed rigid-body
(this will not exclude colliders not attached to any rigid-body). |
## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| List< [Intersection](../Physics/Intersection.md) > | `cast_ray` ( [RayCastOptions](../Physics/RayCastOptions.md) <ins>opts</ins> ) |  |

