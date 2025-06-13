# RayCastOptions
struct in [FyroxLite](../../scripting_api.md).[Physics](../Physics.md)

## Fields
| Name | Type | Description |
|---|---|---|
| `ray_origin` | [Vector3](../Math/Vector3.md) | <p>A ray origin.</p> |
| `ray_direction` | [Vector3](../Math/Vector3.md) | <p>A ray direction. Can be non-normalized.</p> |
| `max_len` | float | <p>Maximum distance of cast.</p> |
| `groups` | [InteractionGroups](../Physics/InteractionGroups.md)? | <p>Groups to check.</p> |
| `sort_results` | bool | <p>Whether to sort intersections from closest to farthest.</p> |
