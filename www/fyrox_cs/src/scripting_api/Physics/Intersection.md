# Intersection
struct in [Physics](../Physics.md)

## Description
<p>A ray intersection result.</p>

## Fields
| Name | Type | Description |
|---|---|---|
| `Collider` | [Node](../Node/Node.md) | <p>A handle of the collider with which intersection was detected.</p> |
| `Normal` | [Vector3](../Math/Vector3.md) | <p>A normal at the intersection position.</p> |
| `Position` | [Vector3](../Math/Vector3.md) | <p>A position of the intersection in world coordinates.</p> |
| `Feature` | [FeatureId](../Physics/FeatureId.md) | <p>Additional data that contains a kind of the feature with which intersection was detected as well as its index.</p> <h1>Important notes.</h1> <p>FeatureId::Face might have index that is greater than amount of triangles in a triangle mesh, this means that intersection was detected from “back” side of a face. To “fix” that index, simply subtract amount of triangles of a triangle mesh from the value.</p> |
| `Toi` | float | <p>Distance from the ray origin.</p> |
