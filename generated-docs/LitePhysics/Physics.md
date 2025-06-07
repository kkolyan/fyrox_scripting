# Physics
class in [FyroxLite](../README.md).[LitePhysics](README.md)
## Constants
| Name | Type | Value | Description |
|---|---|---|---|
| ExcludeFixed | int | 1 << 1 | Exclude from the query any collider attached to a fixed rigid-body and colliders with no rigid-body attached. |
| ExcludeKinematic | int | 1 << 2 | Exclude from the query any collider attached to a kinematic rigid-body. |
| ExcludeDynamic | int | 1 << 3 | Exclude from the query any collider attached to a dynamic rigid-body. |
| ExcludeSensors | int | 1 << 4 | Exclude from the query any collider that is a sensor. |
| ExcludeSolids | int | 1 << 5 | Exclude from the query any collider that is not a sensor. |
| OnlyDynamic | int | LitePhysics :: EXCLUDE_FIXED | LitePhysics :: EXCLUDE_KINEMATIC | Excludes all colliders not attached to a dynamic rigid-body. |
| OnlyKinematic | int | LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_FIXED | Excludes all colliders not attached to a kinematic rigid-body. |
| OnlyFixed | int | LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_KINEMATIC | Exclude all colliders attached to a non-fixed rigid-body
(this will not exclude colliders not attached to any rigid-body). |
## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| List< [Intersection](../LitePhysics/Intersection.md) > | [CastRay](##) ( [RayCastOptions](../LitePhysics/RayCastOptions.md) opts ) |  |

