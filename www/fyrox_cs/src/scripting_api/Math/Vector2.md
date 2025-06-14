# Vector2
struct in [Math](../Math.md)

## Description

(code of this item is picked from Godot Engine in compliance with MIT license).

 2-element structure that can be used to represent positions in 2D space or any other pair of numeric values.


## Constructors
| Signature | Description |
|---|---|
| ( float <ins>x</ins>, float <ins>y</ins> ) |  Constructs a new `Vector2` with the given components.      |

## Methods
| Return Type | Signature | Description |
|---|---|---|
| void | `Deconstruct` ( float <ins>x</ins>, float <ins>y</ins> ) |  Helper method for deconstruction into a tuple.  |
| [Vector2](../Math/Vector2.md) | `Abs` (  ) |  Returns a new vector with all components in absolute values (i.e. positive).    |
| float | `Angle` (  ) |  Returns this vector's angle with respect to the X axis, or (1, 0) vector, in radians.   Equivalent to the result of `Mathf.Atan2(real_t, real_t)` when  called with the vector's `Y` and `X` as parameters: `Mathf.Atan2(v.Y, v.X)` .    |
| float | `AngleTo` ( [Vector2](../Math/Vector2.md) <ins>to</ins> ) |  Returns the angle to the given vector, in radians.      |
| float | `AngleToPoint` ( [Vector2](../Math/Vector2.md) <ins>to</ins> ) |  Returns the angle between the line connecting the two points and the X axis, in radians.      |
| float | `Aspect` (  ) |  Returns the aspect ratio of this vector, the ratio of `X` to `Y` .    |
| [Vector2](../Math/Vector2.md) | `Bounce` ( [Vector2](../Math/Vector2.md) <ins>normal</ins> ) |  Returns the vector "bounced off" from a plane defined by the given normal.      |
| [Vector2](../Math/Vector2.md) | `Ceil` (  ) |  Returns a new vector with all components rounded up (towards positive infinity).    |
| [Vector2](../Math/Vector2.md) | `Clamp` ( [Vector2](../Math/Vector2.md) <ins>min</ins>, [Vector2](../Math/Vector2.md) <ins>max</ins> ) |  Returns a new vector with all components clamped between the  components of `name` and `name` using `Mathf.Clamp(real_t, real_t, real_t)` .        |
| [Vector2](../Math/Vector2.md) | `Clamp` ( float <ins>min</ins>, float <ins>max</ins> ) |  Returns a new vector with all components clamped between the `name` and `name` using `Mathf.Clamp(real_t, real_t, real_t)` .        |
| float | `Cross` ( [Vector2](../Math/Vector2.md) <ins>with</ins> ) |  Returns the cross product of this vector and `name` .      |
| [Vector2](../Math/Vector2.md) | `CubicInterpolate` ( [Vector2](../Math/Vector2.md) <ins>b</ins>, [Vector2](../Math/Vector2.md) <ins>preA</ins>, [Vector2](../Math/Vector2.md) <ins>postB</ins>, float <ins>weight</ins> ) |  Performs a cubic interpolation between vectors `name` , this vector, `name` , and `name` , by the given amount `name` .            |
| [Vector2](../Math/Vector2.md) | `CubicInterpolateInTime` ( [Vector2](../Math/Vector2.md) <ins>b</ins>, [Vector2](../Math/Vector2.md) <ins>preA</ins>, [Vector2](../Math/Vector2.md) <ins>postB</ins>, float <ins>weight</ins>, float <ins>t</ins>, float <ins>preAT</ins>, float <ins>postBT</ins> ) |  Performs a cubic interpolation between vectors `name` , this vector, `name` , and `name` , by the given amount `name` .  It can perform smoother interpolation than `CubicInterpolate` by the time values.                  |
| [Vector2](../Math/Vector2.md) | `BezierInterpolate` ( [Vector2](../Math/Vector2.md) <ins>control1</ins>, [Vector2](../Math/Vector2.md) <ins>control2</ins>, [Vector2](../Math/Vector2.md) <ins>end</ins>, float <ins>t</ins> ) |  Returns the point at the given `name` on a one-dimensional Bezier curve defined by this vector  and the given `name` , `name` , and `name` points.            |
| [Vector2](../Math/Vector2.md) | `BezierDerivative` ( [Vector2](../Math/Vector2.md) <ins>control1</ins>, [Vector2](../Math/Vector2.md) <ins>control2</ins>, [Vector2](../Math/Vector2.md) <ins>end</ins>, float <ins>t</ins> ) |  Returns the derivative at the given `name` on the Bezier curve defined by this vector  and the given `name` , `name` , and `name` points.            |
| [Vector2](../Math/Vector2.md) | `DirectionTo` ( [Vector2](../Math/Vector2.md) <ins>to</ins> ) |  Returns the normalized vector pointing from this vector to `name` .      |
| float | `DistanceSquaredTo` ( [Vector2](../Math/Vector2.md) <ins>to</ins> ) |  Returns the squared distance between this vector and `name` .  This method runs faster than `DistanceTo` , so prefer it if  you need to compare vectors or need the squared distance for some formula.      |
| float | `DistanceTo` ( [Vector2](../Math/Vector2.md) <ins>to</ins> ) |  Returns the distance between this vector and `name` .      |
| float | `Dot` ( [Vector2](../Math/Vector2.md) <ins>with</ins> ) |  Returns the dot product of this vector and `name` .      |
| [Vector2](../Math/Vector2.md) | `Floor` (  ) |  Returns a new vector with all components rounded down (towards negative infinity).    |
| [Vector2](../Math/Vector2.md) | `Inverse` (  ) |  Returns the inverse of this vector. This is the same as `new Vector2(1 / v.X, 1 / v.Y)` .    |
| bool | `IsFinite` (  ) |  Returns `true` if this vector is finite, by calling `Mathf.IsFinite(real_t)` on each component.    |
| bool | `IsNormalized` (  ) |  Returns `true` if the vector is normalized, and `false` otherwise.    |
| float | `Length` (  ) |  Returns the length (magnitude) of this vector.      |
| float | `LengthSquared` (  ) |  Returns the squared length (squared magnitude) of this vector.  This method runs faster than `Length` , so prefer it if  you need to compare vectors or need the squared length for some formula.    |
| [Vector2](../Math/Vector2.md) | `Lerp` ( [Vector2](../Math/Vector2.md) <ins>to</ins>, float <ins>weight</ins> ) |  Returns the result of the linear interpolation between  this vector and `name` by amount `name` .        |
| [Vector2](../Math/Vector2.md) | `LimitLength` ( float <ins>length</ins> ) |  Returns the vector with a maximum length by limiting its length to `name` .      |
| [Vector2](../Math/Vector2.md) | `Max` ( [Vector2](../Math/Vector2.md) <ins>with</ins> ) |  Returns the result of the component-wise maximum between  this vector and `name` .  Equivalent to `new Vector2(Mathf.Max(X, with.X), Mathf.Max(Y, with.Y))` .      |
| [Vector2](../Math/Vector2.md) | `Max` ( float <ins>with</ins> ) |  Returns the result of the component-wise maximum between  this vector and `name` .  Equivalent to `new Vector2(Mathf.Max(X, with), Mathf.Max(Y, with))` .      |
| [Vector2](../Math/Vector2.md) | `Min` ( [Vector2](../Math/Vector2.md) <ins>with</ins> ) |  Returns the result of the component-wise minimum between  this vector and `name` .  Equivalent to `new Vector2(Mathf.Min(X, with.X), Mathf.Min(Y, with.Y))` .      |
| [Vector2](../Math/Vector2.md) | `Min` ( float <ins>with</ins> ) |  Returns the result of the component-wise minimum between  this vector and `name` .  Equivalent to `new Vector2(Mathf.Min(X, with), Mathf.Min(Y, with))` .      |
| Axis | `MaxAxisIndex` (  ) |  Returns the axis of the vector's highest value. See `Axis` .  If both components are equal, this method returns `Axis.X` .    |
| Axis | `MinAxisIndex` (  ) |  Returns the axis of the vector's lowest value. See `Axis` .  If both components are equal, this method returns `Axis.Y` .    |
| [Vector2](../Math/Vector2.md) | `MoveToward` ( [Vector2](../Math/Vector2.md) <ins>to</ins>, float <ins>delta</ins> ) |  Moves this vector toward `name` by the fixed `name` amount.        |
| [Vector2](../Math/Vector2.md) | `Normalized` (  ) |  Returns the vector scaled to unit length. Equivalent to `v / v.Length()` .    |
| [Vector2](../Math/Vector2.md) | `PosMod` ( float <ins>mod</ins> ) |  Returns a vector composed of the `Mathf.PosMod(real_t, real_t)` of this vector's components  and `name` .      |
| [Vector2](../Math/Vector2.md) | `PosMod` ( [Vector2](../Math/Vector2.md) <ins>modv</ins> ) |  Returns a vector composed of the `Mathf.PosMod(real_t, real_t)` of this vector's components  and `name` 's components.      |
| [Vector2](../Math/Vector2.md) | `Project` ( [Vector2](../Math/Vector2.md) <ins>onNormal</ins> ) |  Returns a new vector resulting from projecting this vector onto the given vector `name` .  The resulting new vector is parallel to `name` .  See also `Slide` .  Note: If the vector `name` is a zero vector, the components of the resulting new vector will be `real_t.NaN` .      |
| [Vector2](../Math/Vector2.md) | `Reflect` ( [Vector2](../Math/Vector2.md) <ins>normal</ins> ) |  Returns this vector reflected from a plane defined by the given `name` .      |
| [Vector2](../Math/Vector2.md) | `Rotated` ( float <ins>angle</ins> ) |  Rotates this vector by `name` radians.      |
| [Vector2](../Math/Vector2.md) | `Round` (  ) |  Returns this vector with all components rounded to the nearest integer,  with halfway cases rounded towards the nearest multiple of two.    |
| [Vector2](../Math/Vector2.md) | `Sign` (  ) |  Returns a vector with each component set to one or negative one, depending  on the signs of this vector's components, or zero if the component is zero,  by calling `Mathf.Sign(real_t)` on each component.    |
| [Vector2](../Math/Vector2.md) | `Slerp` ( [Vector2](../Math/Vector2.md) <ins>to</ins>, float <ins>weight</ins> ) |  Returns the result of the spherical linear interpolation between  this vector and `name` by amount `name` .   This method also handles interpolating the lengths if the input vectors  have different lengths. For the special case of one or both input vectors  having zero length, this method behaves like `Lerp` .        |
| [Vector2](../Math/Vector2.md) | `Slide` ( [Vector2](../Math/Vector2.md) <ins>normal</ins> ) |  Returns a new vector resulting from sliding this vector along a line with normal `name` .  The resulting new vector is perpendicular to `name` , and is equivalent to this vector minus its projection on `name` .  See also `Project` .  Note: The vector `name` must be normalized. See also `Normalized` .      |
| [Vector2](../Math/Vector2.md) | `Snapped` ( [Vector2](../Math/Vector2.md) <ins>step</ins> ) |  Returns a new vector with each component snapped to the nearest multiple of the corresponding component in `name` .  This can also be used to round to an arbitrary number of decimals.      |
| [Vector2](../Math/Vector2.md) | `Snapped` ( float <ins>step</ins> ) |  Returns a new vector with each component snapped to the nearest multiple of `name` .  This can also be used to round to an arbitrary number of decimals.      |
| [Vector2](../Math/Vector2.md) | `Orthogonal` (  ) |  Returns a perpendicular vector rotated 90 degrees counter-clockwise  compared to the original, with the same length.    |
| bool | `Equals` ( object? <ins>obj</ins> ) |  Returns `true` if the vector is exactly equal  to the given object ( `name` ).  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.      |
| bool | `Equals` ( [Vector2](../Math/Vector2.md) <ins>other</ins> ) |  Returns `true` if the vectors are exactly equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.      |
| bool | `IsEqualApprox` ( [Vector2](../Math/Vector2.md) <ins>other</ins> ) |  Returns `true` if this vector and `name` are approximately equal,  by running `Mathf.IsEqualApprox(real_t, real_t)` on each component.      |
| bool | `IsZeroApprox` (  ) |  Returns `true` if this vector's values are approximately zero,  by running `Mathf.IsZeroApprox(real_t)` on each component.  This method is faster than using `IsEqualApprox` with one value  as a zero vector.    |
| int | `GetHashCode` (  ) |  Serves as the hash function for `Vector2` .    |
| string | `ToString` (  ) |  Converts this `Vector2` to a string.    |
| string | `ToString` ( string? <ins>format</ins> ) |  Converts this `Vector2` to a string with the given `name` .    |

## Static Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `Zero` | [Vector2](../Math/Vector2.md) | get |  Zero vector, a vector with all components set to `0` .  Equivalent to `new Vector2(0, 0)` .  |
| `One` | [Vector2](../Math/Vector2.md) | get |  One vector, a vector with all components set to `1` .  Equivalent to `new Vector2(1, 1)` .  |
| `Inf` | [Vector2](../Math/Vector2.md) | get |  Infinity vector, a vector with all components set to `Mathf.Inf` .  Equivalent to `new Vector2(Mathf.Inf, Mathf.Inf)` .  |
| `Up` | [Vector2](../Math/Vector2.md) | get |  Up unit vector. Y is down in 2D, so this vector points -Y.  Equivalent to `new Vector2(0, -1)` .  |
| `Down` | [Vector2](../Math/Vector2.md) | get |  Down unit vector. Y is down in 2D, so this vector points +Y.  Equivalent to `new Vector2(0, 1)` .  |
| `Right` | [Vector2](../Math/Vector2.md) | get |  Right unit vector. Represents the direction of right.  Equivalent to `new Vector2(1, 0)` .  |
| `Left` | [Vector2](../Math/Vector2.md) | get |  Left unit vector. Represents the direction of left.  Equivalent to `new Vector2(-1, 0)` .  |

## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| [Vector2](../Math/Vector2.md) | `FromAngle` ( float <ins>angle</ins> ) |  Creates a unit Vector2 rotated to the given angle. This is equivalent to doing `Vector2(Mathf.Cos(angle), Mathf.Sin(angle))` or `Vector2.Right.Rotated(angle)` .      |

## Operators
| Return Type | Signature | Description |
|---|---|---|
| [Vector2](../Math/Vector2.md) | `+` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Adds each component of the `Vector2` with the components of the given `Vector2` .        |
| [Vector2](../Math/Vector2.md) | `-` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Subtracts each component of the `Vector2` by the components of the given `Vector2` .        |
| [Vector2](../Math/Vector2.md) | `-` ( [Vector2](../Math/Vector2.md) <ins>vec</ins> ) |  Returns the negative value of the `Vector2` .  This is the same as writing `new Vector2(-v.X, -v.Y)` .  This operation flips the direction of the vector while  keeping the same magnitude.  With floats, the number zero can be either positive or negative.      |
| [Vector2](../Math/Vector2.md) | `*` ( [Vector2](../Math/Vector2.md) <ins>vec</ins>, float <ins>scale</ins> ) |  Multiplies each component of the `Vector2` by the given `real_t` .        |
| [Vector2](../Math/Vector2.md) | `*` ( float <ins>scale</ins>, [Vector2](../Math/Vector2.md) <ins>vec</ins> ) |  Multiplies each component of the `Vector2` by the given `real_t` .        |
| [Vector2](../Math/Vector2.md) | `*` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Multiplies each component of the `Vector2` by the components of the given `Vector2` .        |
| [Vector2](../Math/Vector2.md) | `/` ( [Vector2](../Math/Vector2.md) <ins>vec</ins>, float <ins>divisor</ins> ) |  Divides each component of the `Vector2` by the given `real_t` .        |
| [Vector2](../Math/Vector2.md) | `/` ( [Vector2](../Math/Vector2.md) <ins>vec</ins>, [Vector2](../Math/Vector2.md) <ins>divisorv</ins> ) |  Divides each component of the `Vector2` by the components of the given `Vector2` .        |
| [Vector2](../Math/Vector2.md) | `%` ( [Vector2](../Math/Vector2.md) <ins>vec</ins>, float <ins>divisor</ins> ) |  Gets the remainder of each component of the `Vector2` with the components of the given `real_t` .  This operation uses truncated division, which is often not desired  as it does not work well with negative numbers.  Consider using `PosMod` instead  if you want to handle negative numbers.          |
| [Vector2](../Math/Vector2.md) | `%` ( [Vector2](../Math/Vector2.md) <ins>vec</ins>, [Vector2](../Math/Vector2.md) <ins>divisorv</ins> ) |  Gets the remainder of each component of the `Vector2` with the components of the given `Vector2` .  This operation uses truncated division, which is often not desired  as it does not work well with negative numbers.  Consider using `PosMod` instead  if you want to handle negative numbers.          |
| bool | `==` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Returns `true` if the vectors are exactly equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
| bool | `!=` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Returns `true` if the vectors are not equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
| bool | `<` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Compares two `Vector2` vectors by first checking if  the X value of the `name` vector is less than  the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors.  This operator is useful for sorting vectors.        |
| bool | `>` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Compares two `Vector2` vectors by first checking if  the X value of the `name` vector is greater than  the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors.  This operator is useful for sorting vectors.        |
| bool | `<=` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Compares two `Vector2` vectors by first checking if  the X value of the `name` vector is less than  or equal to the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors.  This operator is useful for sorting vectors.        |
| bool | `>=` ( [Vector2](../Math/Vector2.md) <ins>left</ins>, [Vector2](../Math/Vector2.md) <ins>right</ins> ) |  Compares two `Vector2` vectors by first checking if  the X value of the `name` vector is greater than  or equal to the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors.  This operator is useful for sorting vectors.        |
