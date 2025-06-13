# Physics
class in [FyroxLite](../../scripting_api.md).[Physics](../Physics.md)

## Constants
| Name | Type | Value | Description |
|---|---|---|---|
| `ExcludeFixed` | int | 1 << 1 | <p>Exclude from the query any collider attached to a fixed rigid-body and colliders with no rigid-body attached.</p> |
| `ExcludeKinematic` | int | 1 << 2 | <p>Exclude from the query any collider attached to a kinematic rigid-body.</p> |
| `ExcludeDynamic` | int | 1 << 3 | <p>Exclude from the query any collider attached to a dynamic rigid-body.</p> |
| `ExcludeSensors` | int | 1 << 4 | <p>Exclude from the query any collider that is a sensor.</p> |
| `ExcludeSolids` | int | 1 << 5 | <p>Exclude from the query any collider that is not a sensor.</p> |
| `OnlyDynamic` | int | LitePhysics :: EXCLUDE_FIXED | LitePhysics :: EXCLUDE_KINEMATIC | <p>Excludes all colliders not attached to a dynamic rigid-body.</p> |
| `OnlyKinematic` | int | LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_FIXED | <p>Excludes all colliders not attached to a kinematic rigid-body.</p> |
| `OnlyFixed` | int | LitePhysics :: EXCLUDE_DYNAMIC | LitePhysics :: EXCLUDE_KINEMATIC | <p>Exclude all colliders attached to a non-fixed rigid-body (this will not exclude colliders not attached to any rigid-body).</p> |

## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| List< [Intersection](../Physics/Intersection.md) > | `CastRay` ( [RayCastOptions](../Physics/RayCastOptions.md) <ins>opts</ins> ) |  |
