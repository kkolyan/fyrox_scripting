# Quaternion
struct in [FyroxLite](../../scripting_api.md).[Math](../Math.md)

## Description

(code of this item is picked from Godot Engine in compliance with MIT license).

 A unit quaternion used for representing 3D rotations.
 Quaternions need to be normalized to be used for rotation.

 It is similar to
`Basis`
, which implements matrix
 representation of rotations, and can be parametrized using both
 an axis-angle pair or Euler angles. Basis stores rotation, scale,
 and shearing, while Quaternion only stores rotation.

 Due to its compactness and the way it is stored in memory, certain
 operations (obtaining axis-angle and performing SLERP, in particular)
 are more efficient and robust against floating-point errors.


## Constructors
| Signature | Description |
|---|---|
| ( float <ins>x</ins>, float <ins>y</ins>, float <ins>z</ins>, float <ins>w</ins> ) |  Constructs a `Quaternion` defined by the given values.          |
| ( [Basis](../Math/Basis.md) <ins>basis</ins> ) |  Constructs a `Quaternion` from the given `Basis` .    |
| ( [Vector3](../Math/Vector3.md) <ins>axis</ins>, float <ins>angle</ins> ) |  Constructs a `Quaternion` that will rotate around the given axis  by the specified angle. The axis must be a normalized vector.      |
| ( [Vector3](../Math/Vector3.md) <ins>arcFrom</ins>, [Vector3](../Math/Vector3.md) <ins>arcTo</ins> ) |  |

## Methods
| Return Type | Signature | Description |
|---|---|---|
| float | `AngleTo` ( [Quaternion](../Math/Quaternion.md) <ins>to</ins> ) |  Returns the angle between this quaternion and `name` .  This is the magnitude of the angle you would need to rotate  by to get from one to the other.   Note: This method has an abnormally high amount  of floating-point error, so methods such as `Mathf.IsZeroApprox(real_t)` will not work reliably.      |
| [Quaternion](../Math/Quaternion.md) | `SphericalCubicInterpolate` ( [Quaternion](../Math/Quaternion.md) <ins>b</ins>, [Quaternion](../Math/Quaternion.md) <ins>preA</ins>, [Quaternion](../Math/Quaternion.md) <ins>postB</ins>, float <ins>weight</ins> ) |  Performs a spherical cubic interpolation between quaternions `name` , this quaternion, `name` , and `name` , by the given amount `name` .            |
| [Quaternion](../Math/Quaternion.md) | `SphericalCubicInterpolateInTime` ( [Quaternion](../Math/Quaternion.md) <ins>b</ins>, [Quaternion](../Math/Quaternion.md) <ins>preA</ins>, [Quaternion](../Math/Quaternion.md) <ins>postB</ins>, float <ins>weight</ins>, float <ins>bT</ins>, float <ins>preAT</ins>, float <ins>postBT</ins> ) |  Performs a spherical cubic interpolation between quaternions `name` , this quaternion, `name` , and `name` , by the given amount `name` .  It can perform smoother interpolation than `SphericalCubicInterpolate` by the time values.                  |
| float | `Dot` ( [Quaternion](../Math/Quaternion.md) <ins>b</ins> ) |  Returns the dot product of two quaternions.      |
| [Quaternion](../Math/Quaternion.md) | `Exp` (  ) |  |
| float | `GetAngle` (  ) |  |
| [Vector3](../Math/Vector3.md) | `GetAxis` (  ) |  |
| [Vector3](../Math/Vector3.md) | `GetEuler` ( [EulerOrder](../Math/EulerOrder.md) <ins>order</ins> ) |  Returns Euler angles (in the YXZ convention: when decomposing,  first Z, then X, and Y last) corresponding to the rotation  represented by the unit quaternion. Returned vector contains  the rotation angles in the format (X angle, Y angle, Z angle).    |
| [Quaternion](../Math/Quaternion.md) | `Inverse` (  ) |  Returns the inverse of the quaternion.    |
| bool | `IsFinite` (  ) |  Returns `true` if this quaternion is finite, by calling `Mathf.IsFinite(real_t)` on each component.    |
| bool | `IsNormalized` (  ) |  Returns whether the quaternion is normalized or not.    |
| [Quaternion](../Math/Quaternion.md) | `Log` (  ) |  |
| float | `Length` (  ) |  Returns the length (magnitude) of the quaternion.    Equivalent to `Mathf.Sqrt(LengthSquared)` .  |
| float | `LengthSquared` (  ) |  Returns the squared length (squared magnitude) of the quaternion.  This method runs faster than `Length` , so prefer it if  you need to compare quaternions or need the squared length for some formula.  Equivalent to `Dot(this)` .  |
| [Quaternion](../Math/Quaternion.md) | `Normalized` (  ) |  Returns a copy of the quaternion, normalized to unit length.    |
| [Quaternion](../Math/Quaternion.md) | `Slerp` ( [Quaternion](../Math/Quaternion.md) <ins>to</ins>, float <ins>weight</ins> ) |  Returns the result of the spherical linear interpolation between  this quaternion and `name` by amount `name` .   Note: Both quaternions must be normalized.        |
| [Quaternion](../Math/Quaternion.md) | `Slerpni` ( [Quaternion](../Math/Quaternion.md) <ins>to</ins>, float <ins>weight</ins> ) |  Returns the result of the spherical linear interpolation between  this quaternion and `name` by amount `name` , but without  checking if the rotation path is not bigger than 90 degrees.        |
| bool | `Equals` ( object? <ins>obj</ins> ) |  Returns `true` if this quaternion and `name` are equal.      |
| bool | `Equals` ( [Quaternion](../Math/Quaternion.md) <ins>other</ins> ) |  Returns `true` if this quaternion and `name` are equal.      |
| bool | `IsEqualApprox` ( [Quaternion](../Math/Quaternion.md) <ins>other</ins> ) |  Returns `true` if this quaternion and `name` are approximately equal,  by running `Mathf.IsEqualApprox(real_t, real_t)` on each component.      |
| int | `GetHashCode` (  ) |  Serves as the hash function for `Quaternion` .    |
| string | `ToString` (  ) |  Converts this `Quaternion` to a string.    |
| string | `ToString` ( string? <ins>format</ins> ) |  Converts this `Quaternion` to a string with the given `name` .    |

## Static Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `Identity` | [Quaternion](../Math/Quaternion.md) | get |  The identity quaternion, representing no rotation.  Equivalent to an identity `Basis` matrix. If a vector is transformed by  an identity quaternion, it will not change.  Equivalent to `new Quaternion(0, 0, 0, 1)` .  |

## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| [Quaternion](../Math/Quaternion.md) | `FromEuler` ( [Vector3](../Math/Vector3.md) <ins>eulerYXZ</ins> ) |  Constructs a `Quaternion` that will perform a rotation specified by  Euler angles (in the YXZ convention: when decomposing, first Z, then X, and Y last),  given in the vector format as (X angle, Y angle, Z angle).    |

## Operators
| Return Type | Signature | Description |
|---|---|---|
| [Quaternion](../Math/Quaternion.md) | `*` ( [Quaternion](../Math/Quaternion.md) <ins>left</ins>, [Quaternion](../Math/Quaternion.md) <ins>right</ins> ) |  Composes these two quaternions by multiplying them together.  This has the effect of rotating the second quaternion  (the child) by the first quaternion (the parent).        |
| [Vector3](../Math/Vector3.md) | `*` ( [Quaternion](../Math/Quaternion.md) <ins>quaternion</ins>, [Vector3](../Math/Vector3.md) <ins>vector</ins> ) |  Returns a Vector3 rotated (multiplied) by the quaternion.        |
| [Vector3](../Math/Vector3.md) | `*` ( [Vector3](../Math/Vector3.md) <ins>vector</ins>, [Quaternion](../Math/Quaternion.md) <ins>quaternion</ins> ) |  Returns a Vector3 rotated (multiplied) by the inverse quaternion. `vector * quaternion` is equivalent to `quaternion.Inverse() * vector` . See `Inverse` .        |
| [Quaternion](../Math/Quaternion.md) | `+` ( [Quaternion](../Math/Quaternion.md) <ins>left</ins>, [Quaternion](../Math/Quaternion.md) <ins>right</ins> ) |  Adds each component of the left `Quaternion` to the right `Quaternion` . This operation is not  meaningful on its own, but it can be used as a part of a  larger expression, such as approximating an intermediate  rotation between two nearby rotations.        |
| [Quaternion](../Math/Quaternion.md) | `-` ( [Quaternion](../Math/Quaternion.md) <ins>left</ins>, [Quaternion](../Math/Quaternion.md) <ins>right</ins> ) |  Subtracts each component of the left `Quaternion` by the right `Quaternion` . This operation is not  meaningful on its own, but it can be used as a part of a  larger expression.        |
| [Quaternion](../Math/Quaternion.md) | `-` ( [Quaternion](../Math/Quaternion.md) <ins>quat</ins> ) |  Returns the negative value of the `Quaternion` .  This is the same as writing `new Quaternion(-q.X, -q.Y, -q.Z, -q.W)` . This operation  results in a quaternion that represents the same rotation.      |
| [Quaternion](../Math/Quaternion.md) | `*` ( [Quaternion](../Math/Quaternion.md) <ins>left</ins>, float <ins>right</ins> ) |  Multiplies each component of the `Quaternion` by the given `real_t` . This operation is not  meaningful on its own, but it can be used as a part of a  larger expression.        |
| [Quaternion](../Math/Quaternion.md) | `*` ( float <ins>left</ins>, [Quaternion](../Math/Quaternion.md) <ins>right</ins> ) |  Multiplies each component of the `Quaternion` by the given `real_t` . This operation is not  meaningful on its own, but it can be used as a part of a  larger expression.        |
| [Quaternion](../Math/Quaternion.md) | `/` ( [Quaternion](../Math/Quaternion.md) <ins>left</ins>, float <ins>right</ins> ) |  Divides each component of the `Quaternion` by the given `real_t` . This operation is not  meaningful on its own, but it can be used as a part of a  larger expression.        |
| bool | `==` ( [Quaternion](../Math/Quaternion.md) <ins>left</ins>, [Quaternion](../Math/Quaternion.md) <ins>right</ins> ) |  Returns `true` if the quaternions are exactly equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
| bool | `!=` ( [Quaternion](../Math/Quaternion.md) <ins>left</ins>, [Quaternion](../Math/Quaternion.md) <ins>right</ins> ) |  Returns `true` if the quaternions are not equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
