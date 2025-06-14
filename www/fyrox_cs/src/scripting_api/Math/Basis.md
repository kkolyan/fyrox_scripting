# Basis
struct in [Math](../Math.md)

## Description

(code of this item is picked from Godot Engine in compliance with MIT license).

 3×3 matrix used for 3D rotation and scale.
 Almost always used as an orthogonal basis for a Transform.

 Contains 3 vector fields X, Y and Z as its columns, which are typically
 interpreted as the local basis vectors of a 3D transformation. For such use,
 it is composed of a scaling and a rotation matrix, in that order (M = R.S).

 Can also be accessed as array of 3D vectors. These vectors are normally
 orthogonal to each other, but are not necessarily normalized (due to scaling).

 For more information, read this documentation article:
 https://docs.godotengine.org/en/latest/tutorials/math/matrices_and_transforms.html


## Constructors
| Signature | Description |
|---|---|
| ( [Quaternion](../Math/Quaternion.md) <ins>quaternion</ins> ) |  Constructs a pure rotation basis matrix from the given quaternion.    |
| ( [Vector3](../Math/Vector3.md) <ins>axis</ins>, float <ins>angle</ins> ) |  Constructs a pure rotation basis matrix, rotated around the given `name` by `name` (in radians). The axis must be a normalized vector.      |
| ( [Vector3](../Math/Vector3.md) <ins>column0</ins>, [Vector3](../Math/Vector3.md) <ins>column1</ins>, [Vector3](../Math/Vector3.md) <ins>column2</ins> ) |  Constructs a basis matrix from 3 axis vectors (matrix columns).        |
| ( float <ins>xx</ins>, float <ins>yx</ins>, float <ins>zx</ins>, float <ins>xy</ins>, float <ins>yy</ins>, float <ins>zy</ins>, float <ins>xz</ins>, float <ins>yz</ins>, float <ins>zz</ins> ) |  Constructs a transformation matrix from the given components.  Arguments are named such that xy is equal to calling `X.Y` .                    |

## Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `X` | [Vector3](../Math/Vector3.md) | get / set |  The basis matrix's X vector (column 0).  Equivalent to `Column0` and array index `[0]` .  |
| `Y` | [Vector3](../Math/Vector3.md) | get / set |  The basis matrix's Y vector (column 1).  Equivalent to `Column1` and array index `[1]` .  |
| `Z` | [Vector3](../Math/Vector3.md) | get / set |  The basis matrix's Z vector (column 2).  Equivalent to `Column2` and array index `[2]` .  |
| `Column0` | [Vector3](../Math/Vector3.md) | get / set |  Column 0 of the basis matrix (the X vector).  Equivalent to `X` and array index `[0]` .  |
| `Column1` | [Vector3](../Math/Vector3.md) | get / set |  Column 1 of the basis matrix (the Y vector).  Equivalent to `Y` and array index `[1]` .  |
| `Column2` | [Vector3](../Math/Vector3.md) | get / set |  Column 2 of the basis matrix (the Z vector).  Equivalent to `Z` and array index `[2]` .  |
| `Scale` | [Vector3](../Math/Vector3.md) | get |  Assuming that the matrix is the combination of a rotation and scaling,  return the absolute value of scaling factors along each axis.  |

## Methods
| Return Type | Signature | Description |
|---|---|---|
| float | `Determinant` (  ) |  Returns the determinant of the basis matrix. If the basis is  uniformly scaled, its determinant is the square of the scale.   A negative determinant means the basis has a negative scale.  A zero determinant means the basis isn't invertible,  and is usually considered invalid.    |
| [Vector3](../Math/Vector3.md) | `GetEuler` ( [EulerOrder](../Math/EulerOrder.md) <ins>order</ins> ) |  Returns the basis's rotation in the form of Euler angles.  The Euler order depends on the `name` parameter,  by default it uses the YXZ convention: when decomposing,  first Z, then X, and Y last. The returned vector contains  the rotation angles in the format (X angle, Y angle, Z angle).   Consider using the `GetRotationQuaternion` method instead, which  returns a `Quaternion` quaternion instead of Euler angles.      |
| [Quaternion](../Math/Quaternion.md) | `GetRotationQuaternion` (  ) |  Returns the `Basis` 's rotation in the form of a `Quaternion` . See `GetEuler` if you  need Euler angles, but keep in mind quaternions should generally  be preferred to Euler angles.    |
| [Basis](../Math/Basis.md) | `Inverse` (  ) |  Returns the inverse of the matrix.    |
| bool | `IsFinite` (  ) |  Returns `true` if this basis is finite, by calling `Mathf.IsFinite(real_t)` on each component.    |
| [Basis](../Math/Basis.md) | `Orthonormalized` (  ) |  Returns the orthonormalized version of the basis matrix (useful to  call occasionally to avoid rounding errors for orthogonal matrices).  This performs a Gram-Schmidt orthonormalization on the basis of the matrix.    |
| [Basis](../Math/Basis.md) | `Rotated` ( [Vector3](../Math/Vector3.md) <ins>axis</ins>, float <ins>angle</ins> ) |  Introduce an additional rotation around the given `name` by `name` (in radians). The axis must be a normalized vector.        |
| [Basis](../Math/Basis.md) | `Scaled` ( [Vector3](../Math/Vector3.md) <ins>scale</ins> ) |  Introduce an additional scaling specified by the given 3D scaling factor.      |
| [Basis](../Math/Basis.md) | `Slerp` ( [Basis](../Math/Basis.md) <ins>target</ins>, float <ins>weight</ins> ) |  Assuming that the matrix is a proper rotation matrix, slerp performs  a spherical-linear interpolation with another rotation matrix.        |
| float | `Tdotx` ( [Vector3](../Math/Vector3.md) <ins>with</ins> ) |  Transposed dot product with the X axis of the matrix.      |
| float | `Tdoty` ( [Vector3](../Math/Vector3.md) <ins>with</ins> ) |  Transposed dot product with the Y axis of the matrix.      |
| float | `Tdotz` ( [Vector3](../Math/Vector3.md) <ins>with</ins> ) |  Transposed dot product with the Z axis of the matrix.      |
| [Basis](../Math/Basis.md) | `Transposed` (  ) |  Returns the transposed version of the basis matrix.    |
| bool | `Equals` ( object? <ins>obj</ins> ) |  Returns `true` if the `Basis` is  exactly equal to the given object ( `name` ).  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.      |
| bool | `Equals` ( [Basis](../Math/Basis.md) <ins>other</ins> ) |  Returns `true` if the basis matrices are exactly  equal. Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.      |
| bool | `IsEqualApprox` ( [Basis](../Math/Basis.md) <ins>other</ins> ) |  Returns `true` if this basis and `name` are approximately equal,  by running `Vector3.IsEqualApprox(Vector3)` on each component.      |
| int | `GetHashCode` (  ) |  Serves as the hash function for `Basis` .    |
| string | `ToString` (  ) |  Converts this `Basis` to a string.    |
| string | `ToString` ( string? <ins>format</ins> ) |  Converts this `Basis` to a string with the given `name` .    |

## Static Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `Identity` | [Basis](../Math/Basis.md) | get |  The identity basis, with no rotation or scaling applied.  This is used as a replacement for `Basis()` in GDScript.  Do not use `new Basis()` with no arguments in C#, because it sets all values to zero.  Equivalent to `new Basis(Vector3.Right, Vector3.Up, Vector3.Back)` .  |
| `FlipX` | [Basis](../Math/Basis.md) | get |  The basis that will flip something along the X axis when used in a transformation.  Equivalent to `new Basis(Vector3.Left, Vector3.Up, Vector3.Back)` .  |
| `FlipY` | [Basis](../Math/Basis.md) | get |  The basis that will flip something along the Y axis when used in a transformation.  Equivalent to `new Basis(Vector3.Right, Vector3.Down, Vector3.Back)` .  |
| `FlipZ` | [Basis](../Math/Basis.md) | get |  The basis that will flip something along the Z axis when used in a transformation.  Equivalent to `new Basis(Vector3.Right, Vector3.Up, Vector3.Forward)` .  |

## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| [Basis](../Math/Basis.md) | `LookingAt` ( [Vector3](../Math/Vector3.md) <ins>target</ins>, [Vector3](../Math/Vector3.md)? <ins>up</ins>, bool <ins>useModelFront</ins> ) |  Creates a `Basis` with a rotation such that the forward  axis (-Z) points towards the `name` position.  The up axis (+Y) points as close to the `name` vector  as possible while staying perpendicular to the forward axis.  The resulting Basis is orthonormalized.  The `name` and `name` vectors  cannot be zero, and cannot be parallel to each other.          |
| [Basis](../Math/Basis.md) | `LookingAt` ( [Vector3](../Math/Vector3.md) <ins>target</ins>, [Vector3](../Math/Vector3.md) <ins>up</ins> ) |  The same as LookingAt  |
| [Basis](../Math/Basis.md) | `FromEuler` ( [Vector3](../Math/Vector3.md) <ins>euler</ins>, [EulerOrder](../Math/EulerOrder.md) <ins>order</ins> ) |  Constructs a Basis matrix from Euler angles in the specified rotation order. By default, use YXZ order (most common).      |
| [Basis](../Math/Basis.md) | `FromScale` ( [Vector3](../Math/Vector3.md) <ins>scale</ins> ) |  Constructs a pure scale basis matrix with no rotation or shearing.  The scale values are set as the main diagonal of the matrix,  and all of the other parts of the matrix are zero.      |

## Operators
| Return Type | Signature | Description |
|---|---|---|
| [Basis](../Math/Basis.md) | `*` ( [Basis](../Math/Basis.md) <ins>left</ins>, [Basis](../Math/Basis.md) <ins>right</ins> ) |  Composes these two basis matrices by multiplying them  together. This has the effect of transforming the second basis  (the child) by the first basis (the parent).        |
| [Vector3](../Math/Vector3.md) | `*` ( [Basis](../Math/Basis.md) <ins>basis</ins>, [Vector3](../Math/Vector3.md) <ins>vector</ins> ) |  Returns a Vector3 transformed (multiplied) by the basis matrix.        |
| [Vector3](../Math/Vector3.md) | `*` ( [Vector3](../Math/Vector3.md) <ins>vector</ins>, [Basis](../Math/Basis.md) <ins>basis</ins> ) |  Returns a Vector3 transformed (multiplied) by the inverse basis matrix,  under the assumption that the transformation basis is orthonormal (i.e. rotation/reflection  is fine, scaling/skew is not). `vector * basis` is equivalent to `basis.Transposed() * vector` . See `Transposed` .  For transforming by inverse of a non-orthonormal basis (e.g. with scaling) `basis.Inverse() * vector` can be used instead. See `Inverse` .        |
| bool | `==` ( [Basis](../Math/Basis.md) <ins>left</ins>, [Basis](../Math/Basis.md) <ins>right</ins> ) |  Returns `true` if the basis matrices are exactly  equal. Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
| bool | `!=` ( [Basis](../Math/Basis.md) <ins>left</ins>, [Basis](../Math/Basis.md) <ins>right</ins> ) |  Returns `true` if the basis matrices are not equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
