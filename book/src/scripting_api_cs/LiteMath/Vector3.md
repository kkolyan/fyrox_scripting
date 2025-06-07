# Vector3
struct in [FyroxLite](../../scripting_api_cs.md).[LiteMath](../LiteMath.md)
## Description

(code of this item is picked from Godot Engine in compliance with MIT license).

 3-element structure that can be used to represent positions in 3D space or any other pair of numeric values.

## Constructors
| Signature | Description |
|---|---|
| ( float <ins>x</ins>, float <ins>y</ins>, float <ins>z</ins> ) |  Constructs a new `Vector3` with the given components.        |
## Methods
| Return Type | Signature | Description |
|---|---|---|
| void | `Deconstruct` ( float <ins>x</ins>, float <ins>y</ins>, float <ins>z</ins> ) |  Helper method for deconstruction into a tuple.  |
| [Vector3](../LiteMath/Vector3.md) | `Abs` (  ) |  Returns a new vector with all components in absolute values (i.e. positive).    |
| float | `AngleTo` ( [Vector3](../LiteMath/Vector3.md) <ins>to</ins> ) |  Returns the unsigned minimum angle to the given vector, in radians.      |
| [Vector3](../LiteMath/Vector3.md) | `Bounce` ( [Vector3](../LiteMath/Vector3.md) <ins>normal</ins> ) |  Returns this vector "bounced off" from a plane defined by the given normal.      |
| [Vector3](../LiteMath/Vector3.md) | `Ceil` (  ) |  Returns a new vector with all components rounded up (towards positive infinity).    |
| [Vector3](../LiteMath/Vector3.md) | `Clamp` ( [Vector3](../LiteMath/Vector3.md) <ins>min</ins>, [Vector3](../LiteMath/Vector3.md) <ins>max</ins> ) |  Returns a new vector with all components clamped between the  components of `name` and `name` using `Mathf.Clamp(real_t, real_t, real_t)` .        |
| [Vector3](../LiteMath/Vector3.md) | `Clamp` ( float <ins>min</ins>, float <ins>max</ins> ) |  Returns a new vector with all components clamped between the `name` and `name` using `Mathf.Clamp(real_t, real_t, real_t)` .        |
| [Vector3](../LiteMath/Vector3.md) | `Cross` ( [Vector3](../LiteMath/Vector3.md) <ins>with</ins> ) |  Returns the cross product of this vector and `name` .      |
| [Vector3](../LiteMath/Vector3.md) | `CubicInterpolate` ( [Vector3](../LiteMath/Vector3.md) <ins>b</ins>, [Vector3](../LiteMath/Vector3.md) <ins>preA</ins>, [Vector3](../LiteMath/Vector3.md) <ins>postB</ins>, float <ins>weight</ins> ) |  Performs a cubic interpolation between vectors `name` , this vector, `name` , and `name` , by the given amount `name` .            |
| [Vector3](../LiteMath/Vector3.md) | `CubicInterpolateInTime` ( [Vector3](../LiteMath/Vector3.md) <ins>b</ins>, [Vector3](../LiteMath/Vector3.md) <ins>preA</ins>, [Vector3](../LiteMath/Vector3.md) <ins>postB</ins>, float <ins>weight</ins>, float <ins>t</ins>, float <ins>preAT</ins>, float <ins>postBT</ins> ) |  Performs a cubic interpolation between vectors `name` , this vector, `name` , and `name` , by the given amount `name` .  It can perform smoother interpolation than `CubicInterpolate` by the time values.                  |
| [Vector3](../LiteMath/Vector3.md) | `BezierInterpolate` ( [Vector3](../LiteMath/Vector3.md) <ins>control1</ins>, [Vector3](../LiteMath/Vector3.md) <ins>control2</ins>, [Vector3](../LiteMath/Vector3.md) <ins>end</ins>, float <ins>t</ins> ) |  Returns the point at the given `name` on a one-dimensional Bezier curve defined by this vector  and the given `name` , `name` , and `name` points.            |
| [Vector3](../LiteMath/Vector3.md) | `BezierDerivative` ( [Vector3](../LiteMath/Vector3.md) <ins>control1</ins>, [Vector3](../LiteMath/Vector3.md) <ins>control2</ins>, [Vector3](../LiteMath/Vector3.md) <ins>end</ins>, float <ins>t</ins> ) |  Returns the derivative at the given `name` on the Bezier curve defined by this vector  and the given `name` , `name` , and `name` points.            |
| [Vector3](../LiteMath/Vector3.md) | `DirectionTo` ( [Vector3](../LiteMath/Vector3.md) <ins>to</ins> ) |  Returns the normalized vector pointing from this vector to `name` .      |
| float | `DistanceSquaredTo` ( [Vector3](../LiteMath/Vector3.md) <ins>to</ins> ) |  Returns the squared distance between this vector and `name` .  This method runs faster than `DistanceTo` , so prefer it if  you need to compare vectors or need the squared distance for some formula.      |
| float | `DistanceTo` ( [Vector3](../LiteMath/Vector3.md) <ins>to</ins> ) |  Returns the distance between this vector and `name` .        |
| float | `Dot` ( [Vector3](../LiteMath/Vector3.md) <ins>with</ins> ) |  Returns the dot product of this vector and `name` .      |
| [Vector3](../LiteMath/Vector3.md) | `Floor` (  ) |  Returns a new vector with all components rounded down (towards negative infinity).    |
| [Vector3](../LiteMath/Vector3.md) | `Inverse` (  ) |  Returns the inverse of this vector. This is the same as `new Vector3(1 / v.X, 1 / v.Y, 1 / v.Z)` .    |
| bool | `IsFinite` (  ) |  Returns `true` if this vector is finite, by calling `Mathf.IsFinite(real_t)` on each component.    |
| bool | `IsNormalized` (  ) |  Returns `true` if the vector is normalized, and `false` otherwise.    |
| float | `Length` (  ) |  Returns the length (magnitude) of this vector.      |
| float | `LengthSquared` (  ) |  Returns the squared length (squared magnitude) of this vector.  This method runs faster than `Length` , so prefer it if  you need to compare vectors or need the squared length for some formula.    |
| [Vector3](../LiteMath/Vector3.md) | `Lerp` ( [Vector3](../LiteMath/Vector3.md) <ins>to</ins>, float <ins>weight</ins> ) |  Returns the result of the linear interpolation between  this vector and `name` by amount `name` .        |
| [Vector3](../LiteMath/Vector3.md) | `LimitLength` ( float <ins>length</ins> ) |  Returns the vector with a maximum length by limiting its length to `name` .      |
| [Vector3](../LiteMath/Vector3.md) | `Max` ( [Vector3](../LiteMath/Vector3.md) <ins>with</ins> ) |  Returns the result of the component-wise maximum between  this vector and `name` .  Equivalent to `new Vector3(Mathf.Max(X, with.X), Mathf.Max(Y, with.Y), Mathf.Max(Z, with.Z))` .      |
| [Vector3](../LiteMath/Vector3.md) | `Max` ( float <ins>with</ins> ) |  Returns the result of the component-wise maximum between  this vector and `name` .  Equivalent to `new Vector3(Mathf.Max(X, with), Mathf.Max(Y, with), Mathf.Max(Z, with))` .      |
| [Vector3](../LiteMath/Vector3.md) | `Min` ( [Vector3](../LiteMath/Vector3.md) <ins>with</ins> ) |  Returns the result of the component-wise minimum between  this vector and `name` .  Equivalent to `new Vector3(Mathf.Min(X, with.X), Mathf.Min(Y, with.Y), Mathf.Min(Z, with.Z))` .      |
| Axis | `MaxAxisIndex` (  ) |  Returns the axis of the vector's highest value. See `Axis` .  If all components are equal, this method returns `Axis.X` .    |
| Axis | `MinAxisIndex` (  ) |  Returns the axis of the vector's lowest value. See `Axis` .  If all components are equal, this method returns `Axis.Z` .    |
| [Vector3](../LiteMath/Vector3.md) | `MoveToward` ( [Vector3](../LiteMath/Vector3.md) <ins>to</ins>, float <ins>delta</ins> ) |  Moves this vector toward `name` by the fixed `name` amount.        |
| [Vector3](../LiteMath/Vector3.md) | `Normalized` (  ) |  Returns the vector scaled to unit length. Equivalent to `v / v.Length()` .    |
| [Basis](../LiteMath/Basis.md) | `Outer` ( [Vector3](../LiteMath/Vector3.md) <ins>with</ins> ) |  Returns the outer product with `name` .      |
| [Vector3](../LiteMath/Vector3.md) | `PosMod` ( float <ins>mod</ins> ) |  Returns a vector composed of the `Mathf.PosMod(real_t, real_t)` of this vector's components  and `name` .      |
| [Vector3](../LiteMath/Vector3.md) | `PosMod` ( [Vector3](../LiteMath/Vector3.md) <ins>modv</ins> ) |  Returns a vector composed of the `Mathf.PosMod(real_t, real_t)` of this vector's components  and `name` 's components.      |
| [Vector3](../LiteMath/Vector3.md) | `Project` ( [Vector3](../LiteMath/Vector3.md) <ins>onNormal</ins> ) |  Returns a new vector resulting from projecting this vector onto the given vector `name` .  The resulting new vector is parallel to `name` .  See also `Slide` .  Note: If the vector `name` is a zero vector, the components of the resulting new vector will be `real_t.NaN` .      |
| [Vector3](../LiteMath/Vector3.md) | `Reflect` ( [Vector3](../LiteMath/Vector3.md) <ins>normal</ins> ) |  Returns this vector reflected from a plane defined by the given `name` .      |
| [Vector3](../LiteMath/Vector3.md) | `Rotated` ( [Vector3](../LiteMath/Vector3.md) <ins>axis</ins>, float <ins>angle</ins> ) |  Rotates this vector around a given `name` vector by `name` (in radians).  The `name` vector must be a normalized vector.        |
| [Vector3](../LiteMath/Vector3.md) | `Round` (  ) |  Returns this vector with all components rounded to the nearest integer,  with halfway cases rounded towards the nearest multiple of two.    |
| [Vector3](../LiteMath/Vector3.md) | `Sign` (  ) |  Returns a vector with each component set to one or negative one, depending  on the signs of this vector's components, or zero if the component is zero,  by calling `Mathf.Sign(real_t)` on each component.    |
| float | `SignedAngleTo` ( [Vector3](../LiteMath/Vector3.md) <ins>to</ins>, [Vector3](../LiteMath/Vector3.md) <ins>axis</ins> ) |  Returns the signed angle to the given vector, in radians.  The sign of the angle is positive in a counter-clockwise  direction and negative in a clockwise direction when viewed  from the side specified by the `name` .        |
| [Vector3](../LiteMath/Vector3.md) | `Slerp` ( [Vector3](../LiteMath/Vector3.md) <ins>to</ins>, float <ins>weight</ins> ) |  Returns the result of the spherical linear interpolation between  this vector and `name` by amount `name` .   This method also handles interpolating the lengths if the input vectors  have different lengths. For the special case of one or both input vectors  having zero length, this method behaves like `Lerp` .        |
| [Vector3](../LiteMath/Vector3.md) | `Slide` ( [Vector3](../LiteMath/Vector3.md) <ins>normal</ins> ) |  Returns a new vector resulting from sliding this vector along a plane with normal `name` .  The resulting new vector is perpendicular to `name` , and is equivalent to this vector minus its projection on `name` .  See also `Project` .  Note: The vector `name` must be normalized. See also `Normalized` .      |
| [Vector3](../LiteMath/Vector3.md) | `Snapped` ( [Vector3](../LiteMath/Vector3.md) <ins>step</ins> ) |  Returns a new vector with each component snapped to the nearest multiple of the corresponding component in `name` .  This can also be used to round to an arbitrary number of decimals.      |
| [Vector3](../LiteMath/Vector3.md) | `Snapped` ( float <ins>step</ins> ) |  Returns a new vector with each component snapped to the nearest multiple of `name` .  This can also be used to round to an arbitrary number of decimals.      |
| bool | `Equals` ( object? <ins>obj</ins> ) |  Returns `true` if the vector is exactly equal  to the given object ( `name` ).  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.      |
| bool | `Equals` ( [Vector3](../LiteMath/Vector3.md) <ins>other</ins> ) |  Returns `true` if the vectors are exactly equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.      |
| bool | `IsEqualApprox` ( [Vector3](../LiteMath/Vector3.md) <ins>other</ins> ) |  Returns `true` if this vector and `name` are approximately equal,  by running `Mathf.IsEqualApprox(real_t, real_t)` on each component.      |
| bool | `IsZeroApprox` (  ) |  Returns `true` if this vector's values are approximately zero,  by running `Mathf.IsZeroApprox(real_t)` on each component.  This method is faster than using `IsEqualApprox` with one value  as a zero vector.    |
| int | `GetHashCode` (  ) |  Serves as the hash function for `Vector3` .    |
| string | `ToString` (  ) |  Converts this `Vector3` to a string.    |
| string | `ToString` ( string? <ins>format</ins> ) |  Converts this `Vector3` to a string with the given `name` .    |
## Static Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `Zero` | [Vector3](../LiteMath/Vector3.md) | get |  Zero vector, a vector with all components set to `0` .  Equivalent to `new Vector3(0, 0, 0)` .  |
| `One` | [Vector3](../LiteMath/Vector3.md) | get |  One vector, a vector with all components set to `1` .  Equivalent to `new Vector3(1, 1, 1)` .  |
| `Inf` | [Vector3](../LiteMath/Vector3.md) | get |  Infinity vector, a vector with all components set to `Mathf.Inf` .  Equivalent to `new Vector3(Mathf.Inf, Mathf.Inf, Mathf.Inf)` .  |
| `Up` | [Vector3](../LiteMath/Vector3.md) | get |  Up unit vector.  Equivalent to `new Vector3(0, 1, 0)` .  |
| `Down` | [Vector3](../LiteMath/Vector3.md) | get |  Down unit vector.  Equivalent to `new Vector3(0, -1, 0)` .  |
| `Right` | [Vector3](../LiteMath/Vector3.md) | get |  Right unit vector. Represents the local direction of right,  and the global direction of east.  Equivalent to `new Vector3(1, 0, 0)` .  |
| `Left` | [Vector3](../LiteMath/Vector3.md) | get |  Left unit vector. Represents the local direction of left,  and the global direction of west.  Equivalent to `new Vector3(-1, 0, 0)` .  |
| `Forward` | [Vector3](../LiteMath/Vector3.md) | get |  Forward unit vector. Represents the local direction of forward,  and the global direction of north.  Equivalent to `new Vector3(0, 0, -1)` .  |
| `Back` | [Vector3](../LiteMath/Vector3.md) | get |  Back unit vector. Represents the local direction of back,  and the global direction of south.  Equivalent to `new Vector3(0, 0, 1)` .  |
| `ModelLeft` | [Vector3](../LiteMath/Vector3.md) | get |  Unit vector pointing towards the left side of imported 3D assets.  |
| `ModelRight` | [Vector3](../LiteMath/Vector3.md) | get |  Unit vector pointing towards the right side of imported 3D assets.  |
| `ModelTop` | [Vector3](../LiteMath/Vector3.md) | get |  Unit vector pointing towards the top side (up) of imported 3D assets.  |
| `ModelBottom` | [Vector3](../LiteMath/Vector3.md) | get |  Unit vector pointing towards the bottom side (down) of imported 3D assets.  |
| `ModelFront` | [Vector3](../LiteMath/Vector3.md) | get |  Unit vector pointing towards the front side (facing forward) of imported 3D assets.  |
| `ModelRear` | [Vector3](../LiteMath/Vector3.md) | get |  Unit vector pointing towards the rear side (back) of imported 3D assets.  |
## Operators
| Return Type | Signature | Description |
|---|---|---|
| [Vector3](../LiteMath/Vector3.md) | `+` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Adds each component of the `Vector3` with the components of the given `Vector3` .        |
| [Vector3](../LiteMath/Vector3.md) | `-` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Subtracts each component of the `Vector3` by the components of the given `Vector3` .        |
| [Vector3](../LiteMath/Vector3.md) | `-` ( [Vector3](../LiteMath/Vector3.md) <ins>vec</ins> ) |  Returns the negative value of the `Vector3` .  This is the same as writing `new Vector3(-v.X, -v.Y, -v.Z)` .  This operation flips the direction of the vector while  keeping the same magnitude.  With floats, the number zero can be either positive or negative.      |
| [Vector3](../LiteMath/Vector3.md) | `*` ( [Vector3](../LiteMath/Vector3.md) <ins>vec</ins>, float <ins>scale</ins> ) |  Multiplies each component of the `Vector3` by the given `real_t` .        |
| [Vector3](../LiteMath/Vector3.md) | `*` ( float <ins>scale</ins>, [Vector3](../LiteMath/Vector3.md) <ins>vec</ins> ) |  Multiplies each component of the `Vector3` by the given `real_t` .        |
| [Vector3](../LiteMath/Vector3.md) | `*` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Multiplies each component of the `Vector3` by the components of the given `Vector3` .        |
| [Vector3](../LiteMath/Vector3.md) | `/` ( [Vector3](../LiteMath/Vector3.md) <ins>vec</ins>, float <ins>divisor</ins> ) |  Divides each component of the `Vector3` by the given `real_t` .        |
| [Vector3](../LiteMath/Vector3.md) | `/` ( [Vector3](../LiteMath/Vector3.md) <ins>vec</ins>, [Vector3](../LiteMath/Vector3.md) <ins>divisorv</ins> ) |  Divides each component of the `Vector3` by the components of the given `Vector3` .        |
| [Vector3](../LiteMath/Vector3.md) | `%` ( [Vector3](../LiteMath/Vector3.md) <ins>vec</ins>, float <ins>divisor</ins> ) |  Gets the remainder of each component of the `Vector3` with the components of the given `real_t` .  This operation uses truncated division, which is often not desired  as it does not work well with negative numbers.  Consider using `PosMod` instead  if you want to handle negative numbers.          |
| [Vector3](../LiteMath/Vector3.md) | `%` ( [Vector3](../LiteMath/Vector3.md) <ins>vec</ins>, [Vector3](../LiteMath/Vector3.md) <ins>divisorv</ins> ) |  Gets the remainder of each component of the `Vector3` with the components of the given `Vector3` .  This operation uses truncated division, which is often not desired  as it does not work well with negative numbers.  Consider using `PosMod` instead  if you want to handle negative numbers.          |
| bool | `==` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Returns `true` if the vectors are exactly equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
| bool | `!=` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Returns `true` if the vectors are not equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
| bool | `<` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Compares two `Vector3` vectors by first checking if  the X value of the `name` vector is less than  the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors, and then with the Z values.  This operator is useful for sorting vectors.        |
| bool | `>` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Compares two `Vector3` vectors by first checking if  the X value of the `name` vector is greater than  the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors, and then with the Z values.  This operator is useful for sorting vectors.        |
| bool | `<=` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Compares two `Vector3` vectors by first checking if  the X value of the `name` vector is less than  or equal to the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors, and then with the Z values.  This operator is useful for sorting vectors.        |
| bool | `>=` ( [Vector3](../LiteMath/Vector3.md) <ins>left</ins>, [Vector3](../LiteMath/Vector3.md) <ins>right</ins> ) |  Compares two `Vector3` vectors by first checking if  the X value of the `name` vector is greater than  or equal to the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors, and then with the Z values.  This operator is useful for sorting vectors.        |
