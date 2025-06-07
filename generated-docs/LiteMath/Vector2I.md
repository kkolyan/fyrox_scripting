# Vector2I
class in [FyroxLite](../README.md).[LiteMath](README.md)
## Description

(The code of this item is picked from Godot Engine).

 2-element structure that can be used to represent 2D grid coordinates or pairs of integers.

## Constructors
| Signature | Description |
|---|---|
| ( int <ins>x</ins>, int <ins>y</ins> ) |  Constructs a new `Vector2I` with the given components.      |
## Methods
| Return Type | Signature | Description |
|---|---|---|
| void | `Deconstruct` ( int <ins>x</ins>, int <ins>y</ins> ) |  Helper method for deconstruction into a tuple.  |
| [Vector2I](../LiteMath/Vector2I.md) | `Abs` (  ) |  Returns a new vector with all components in absolute values (i.e. positive).    |
| float | `Aspect` (  ) |  Returns the aspect ratio of this vector, the ratio of `X` to `Y` .    |
| [Vector2I](../LiteMath/Vector2I.md) | `Clamp` ( [Vector2I](../LiteMath/Vector2I.md) <ins>min</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>max</ins> ) |  Returns a new vector with all components clamped between the  components of `name` and `name` using `Mathf.Clamp(int, int, int)` .        |
| [Vector2I](../LiteMath/Vector2I.md) | `Clamp` ( int <ins>min</ins>, int <ins>max</ins> ) |  Returns a new vector with all components clamped between the `name` and `name` using `Mathf.Clamp(int, int, int)` .        |
| int | `DistanceSquaredTo` ( [Vector2I](../LiteMath/Vector2I.md) <ins>to</ins> ) |  Returns the squared distance between this vector and `name` .  This method runs faster than `DistanceTo` , so prefer it if  you need to compare vectors or need the squared distance for some formula.      |
| float | `DistanceTo` ( [Vector2I](../LiteMath/Vector2I.md) <ins>to</ins> ) |  Returns the distance between this vector and `name` .        |
| float | `Length` (  ) |  Returns the length (magnitude) of this vector.      |
| int | `LengthSquared` (  ) |  Returns the squared length (squared magnitude) of this vector.  This method runs faster than `Length` , so prefer it if  you need to compare vectors or need the squared length for some formula.    |
| [Vector2I](../LiteMath/Vector2I.md) | `Max` ( [Vector2I](../LiteMath/Vector2I.md) <ins>with</ins> ) |  Returns the result of the component-wise maximum between  this vector and `name` .  Equivalent to `new Vector2I(Mathf.Max(X, with.X), Mathf.Max(Y, with.Y))` .      |
| [Vector2I](../LiteMath/Vector2I.md) | `Max` ( int <ins>with</ins> ) |  Returns the result of the component-wise maximum between  this vector and `name` .  Equivalent to `new Vector2I(Mathf.Max(X, with), Mathf.Max(Y, with))` .      |
| [Vector2I](../LiteMath/Vector2I.md) | `Min` ( [Vector2I](../LiteMath/Vector2I.md) <ins>with</ins> ) |  Returns the result of the component-wise minimum between  this vector and `name` .  Equivalent to `new Vector2I(Mathf.Min(X, with.X), Mathf.Min(Y, with.Y))` .      |
| [Vector2I](../LiteMath/Vector2I.md) | `Min` ( int <ins>with</ins> ) |  Returns the result of the component-wise minimum between  this vector and `name` .  Equivalent to `new Vector2I(Mathf.Min(X, with), Mathf.Min(Y, with))` .      |
| Axis | `MaxAxisIndex` (  ) |  Returns the axis of the vector's highest value. See `Axis` .  If both components are equal, this method returns `Axis.X` .    |
| Axis | `MinAxisIndex` (  ) |  Returns the axis of the vector's lowest value. See `Axis` .  If both components are equal, this method returns `Axis.Y` .    |
| [Vector2I](../LiteMath/Vector2I.md) | `Sign` (  ) |  Returns a vector with each component set to one or negative one, depending  on the signs of this vector's components, or zero if the component is zero,  by calling `Mathf.Sign(int)` on each component.    |
| [Vector2I](../LiteMath/Vector2I.md) | `Snapped` ( [Vector2I](../LiteMath/Vector2I.md) <ins>step</ins> ) |  Returns a new vector with each component snapped to the closest multiple of the corresponding component in `name` .      |
| [Vector2I](../LiteMath/Vector2I.md) | `Snapped` ( int <ins>step</ins> ) |  Returns a new vector with each component snapped to the closest multiple of `name` .      |
| bool | `Equals` ( object? <ins>obj</ins> ) |  Returns `true` if the vector is equal  to the given object ( `name` ).      |
| bool | `Equals` ( [Vector2I](../LiteMath/Vector2I.md) <ins>other</ins> ) |  Returns `true` if the vectors are equal.      |
| int | `GetHashCode` (  ) |  Serves as the hash function for `Vector2I` .    |
| string | `ToString` (  ) |  Converts this `Vector2I` to a string.    |
| string | `ToString` ( string? <ins>format</ins> ) |  Converts this `Vector2I` to a string with the given `name` .    |
## Static Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `MinValue` | [Vector2I](../LiteMath/Vector2I.md) | get |  Min vector, a vector with all components equal to `int.MinValue` . Can be used as a negative integer equivalent of `Vector2.Inf` .  Equivalent to `new Vector2I(int.MinValue, int.MinValue)` .  |
| `MaxValue` | [Vector2I](../LiteMath/Vector2I.md) | get |  Max vector, a vector with all components equal to `int.MaxValue` . Can be used as an integer equivalent of `Vector2.Inf` .  Equivalent to `new Vector2I(int.MaxValue, int.MaxValue)` .  |
| `Zero` | [Vector2I](../LiteMath/Vector2I.md) | get |  Zero vector, a vector with all components set to `0` .  Equivalent to `new Vector2I(0, 0)` .  |
| `One` | [Vector2I](../LiteMath/Vector2I.md) | get |  One vector, a vector with all components set to `1` .  Equivalent to `new Vector2I(1, 1)` .  |
| `Up` | [Vector2I](../LiteMath/Vector2I.md) | get |  Up unit vector. Y is down in 2D, so this vector points -Y.  Equivalent to `new Vector2I(0, -1)` .  |
| `Down` | [Vector2I](../LiteMath/Vector2I.md) | get |  Down unit vector. Y is down in 2D, so this vector points +Y.  Equivalent to `new Vector2I(0, 1)` .  |
| `Right` | [Vector2I](../LiteMath/Vector2I.md) | get |  Right unit vector. Represents the direction of right.  Equivalent to `new Vector2I(1, 0)` .  |
| `Left` | [Vector2I](../LiteMath/Vector2I.md) | get |  Left unit vector. Represents the direction of left.  Equivalent to `new Vector2I(-1, 0)` .  |
## Operators
| Return Type | Signature | Description |
|---|---|---|
| [Vector2I](../LiteMath/Vector2I.md) | `+` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Adds each component of the `Vector2I` with the components of the given `Vector2I` .        |
| [Vector2I](../LiteMath/Vector2I.md) | `-` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Subtracts each component of the `Vector2I` by the components of the given `Vector2I` .        |
| [Vector2I](../LiteMath/Vector2I.md) | `-` ( [Vector2I](../LiteMath/Vector2I.md) <ins>vec</ins> ) |  Returns the negative value of the `Vector2I` .  This is the same as writing `new Vector2I(-v.X, -v.Y)` .  This operation flips the direction of the vector while  keeping the same magnitude.      |
| [Vector2I](../LiteMath/Vector2I.md) | `*` ( [Vector2I](../LiteMath/Vector2I.md) <ins>vec</ins>, int <ins>scale</ins> ) |  Multiplies each component of the `Vector2I` by the given `int` .        |
| [Vector2I](../LiteMath/Vector2I.md) | `*` ( int <ins>scale</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>vec</ins> ) |  Multiplies each component of the `Vector2I` by the given `int` .        |
| [Vector2I](../LiteMath/Vector2I.md) | `*` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Multiplies each component of the `Vector2I` by the components of the given `Vector2I` .        |
| [Vector2I](../LiteMath/Vector2I.md) | `/` ( [Vector2I](../LiteMath/Vector2I.md) <ins>vec</ins>, int <ins>divisor</ins> ) |  Divides each component of the `Vector2I` by the given `int` .        |
| [Vector2I](../LiteMath/Vector2I.md) | `/` ( [Vector2I](../LiteMath/Vector2I.md) <ins>vec</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>divisorv</ins> ) |  Divides each component of the `Vector2I` by the components of the given `Vector2I` .        |
| [Vector2I](../LiteMath/Vector2I.md) | `%` ( [Vector2I](../LiteMath/Vector2I.md) <ins>vec</ins>, int <ins>divisor</ins> ) |  Gets the remainder of each component of the `Vector2I` with the components of the given `int` .  This operation uses truncated division, which is often not desired  as it does not work well with negative numbers.  Consider using `Mathf.PosMod(int, int)` instead  if you want to handle negative numbers.          |
| [Vector2I](../LiteMath/Vector2I.md) | `%` ( [Vector2I](../LiteMath/Vector2I.md) <ins>vec</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>divisorv</ins> ) |  Gets the remainder of each component of the `Vector2I` with the components of the given `Vector2I` .  This operation uses truncated division, which is often not desired  as it does not work well with negative numbers.  Consider using `Mathf.PosMod(int, int)` instead  if you want to handle negative numbers.          |
| bool | `==` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Returns `true` if the vectors are equal.        |
| bool | `!=` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Returns `true` if the vectors are not equal.        |
| bool | `<` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Compares two `Vector2I` vectors by first checking if  the X value of the `name` vector is less than  the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors.  This operator is useful for sorting vectors.        |
| bool | `>` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Compares two `Vector2I` vectors by first checking if  the X value of the `name` vector is greater than  the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors.  This operator is useful for sorting vectors.        |
| bool | `<=` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Compares two `Vector2I` vectors by first checking if  the X value of the `name` vector is less than  or equal to the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors.  This operator is useful for sorting vectors.        |
| bool | `>=` ( [Vector2I](../LiteMath/Vector2I.md) <ins>left</ins>, [Vector2I](../LiteMath/Vector2I.md) <ins>right</ins> ) |  Compares two `Vector2I` vectors by first checking if  the X value of the `name` vector is greater than  or equal to the X value of the `name` vector.  If the X values are exactly equal, then it repeats this check  with the Y values of the two vectors.  This operator is useful for sorting vectors.        |
