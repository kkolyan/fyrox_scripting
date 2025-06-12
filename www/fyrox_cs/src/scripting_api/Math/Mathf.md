# Mathf
class in [FyroxLite](../../scripting_api.md).[Math](../Math.md)

## Description

(code of this item is picked from Godot Engine in compliance with MIT license).

 Provides constants and static methods for common mathematical functions.


## Constants
| Name | Type | Value | Description |
|---|---|---|---|
| `Tau` | float | (real_t)6.2831853071795864769252867666M |  The circle constant, the circumference of the unit circle in radians.  |
| `Pi` | float | (real_t)3.1415926535897932384626433833M |  Constant that represents how many times the diameter of a circle  fits around its perimeter. This is equivalent to `Mathf.Tau / 2` .  |
| `Inf` | float | real_t.PositiveInfinity |  Positive infinity. For negative infinity, use `-Mathf.Inf` .  |
| `NaN` | float | real_t.NaN |  "Not a Number", an invalid value. `NaN` has special properties, including  that it is not equal to itself. It is output by some invalid operations,  such as dividing zero by zero.  |
| `E` | float | (real_t)2.7182818284590452353602874714M |  The natural number `e` .  |
| `Sqrt2` | float | (real_t)1.4142135623730950488016887242M |  The square root of 2.  |
| `Epsilon` | float | EpsilonF |  A very small number used for float comparison with error tolerance.  1e-06 with single-precision floats, but 1e-14 if `REAL_T_IS_DOUBLE` .  |

## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| int | `Abs` ( int <ins>s</ins> ) |  Returns the absolute value of `name` (i.e. positive value).      |
| float | `Abs` ( float <ins>s</ins> ) |  Returns the absolute value of `name` (i.e. positive value).      |
| double | `Abs` ( double <ins>s</ins> ) |  Returns the absolute value of `name` (i.e. positive value).      |
| float | `Acos` ( float <ins>s</ins> ) |  Returns the arc cosine of `name` in radians.  Use to get the angle of cosine `name` .      |
| double | `Acos` ( double <ins>s</ins> ) |  Returns the arc cosine of `name` in radians.  Use to get the angle of cosine `name` .      |
| float | `Acosh` ( float <ins>s</ins> ) |  Returns the hyperbolic arc (also called inverse) cosine of `name` in radians.  Use it to get the angle from an angle's cosine in hyperbolic space if `name` is larger or equal to 1.      |
| double | `Acosh` ( double <ins>s</ins> ) |  Returns the hyperbolic arc (also called inverse) cosine of `name` in radians.  Use it to get the angle from an angle's cosine in hyperbolic space if `name` is larger or equal to 1.      |
| float | `AngleDifference` ( float <ins>from</ins>, float <ins>to</ins> ) |  Returns the difference between the two angles,  in range of - `Pi` , `Pi` .  When `name` and `name` are opposite,  returns - `Pi` if `name` is smaller than `name` ,  or `Pi` otherwise.        |
| double | `AngleDifference` ( double <ins>from</ins>, double <ins>to</ins> ) |  Returns the difference between the two angles,  in range of - `Pi` , `Pi` .  When `name` and `name` are opposite,  returns - `Pi` if `name` is smaller than `name` ,  or `Pi` otherwise.        |
| float | `Asin` ( float <ins>s</ins> ) |  Returns the arc sine of `name` in radians.  Use to get the angle of sine `name` .      |
| double | `Asin` ( double <ins>s</ins> ) |  Returns the arc sine of `name` in radians.  Use to get the angle of sine `name` .      |
| float | `Asinh` ( float <ins>s</ins> ) |  Returns the hyperbolic arc (also called inverse) sine of `name` in radians.  Use it to get the angle from an angle's sine in hyperbolic space if `name` is larger or equal to 1.      |
| double | `Asinh` ( double <ins>s</ins> ) |  Returns the hyperbolic arc (also called inverse) sine of `name` in radians.  Use it to get the angle from an angle's sine in hyperbolic space if `name` is larger or equal to 1.      |
| float | `Atan` ( float <ins>s</ins> ) |  Returns the arc tangent of `name` in radians.  Use to get the angle of tangent `name` .   The method cannot know in which quadrant the angle should fall.  See `Atan2` if you have both `y` and `x` .      |
| double | `Atan` ( double <ins>s</ins> ) |  Returns the arc tangent of `name` in radians.  Use to get the angle of tangent `name` .   The method cannot know in which quadrant the angle should fall.  See `Atan2` if you have both `y` and `x` .      |
| float | `Atan2` ( float <ins>y</ins>, float <ins>x</ins> ) |  Returns the arc tangent of `name` and `name` in radians.  Use to get the angle of the tangent of `y/x` . To compute the value, the method takes into  account the sign of both arguments in order to determine the quadrant.   Important note: The Y coordinate comes first, by convention.        |
| double | `Atan2` ( double <ins>y</ins>, double <ins>x</ins> ) |  Returns the arc tangent of `name` and `name` in radians.  Use to get the angle of the tangent of `y/x` . To compute the value, the method takes into  account the sign of both arguments in order to determine the quadrant.   Important note: The Y coordinate comes first, by convention.        |
| float | `Atanh` ( float <ins>s</ins> ) |  Returns the hyperbolic arc (also called inverse) tangent of `name` in radians.  Use it to get the angle from an angle's tangent in hyperbolic space if `name` is between -1 and 1 (non-inclusive).      |
| double | `Atanh` ( double <ins>s</ins> ) |  Returns the hyperbolic arc (also called inverse) tangent of `name` in radians.  Use it to get the angle from an angle's tangent in hyperbolic space if `name` is between -1 and 1 (non-inclusive).      |
| float | `Ceil` ( float <ins>s</ins> ) |  Rounds `name` upward (towards positive infinity).      |
| double | `Ceil` ( double <ins>s</ins> ) |  Rounds `name` upward (towards positive infinity).      |
| int | `Clamp` ( int <ins>value</ins>, int <ins>min</ins>, int <ins>max</ins> ) |  Clamps a `name` so that it is not less than `name` and not more than `name` .          |
| float | `Clamp` ( float <ins>value</ins>, float <ins>min</ins>, float <ins>max</ins> ) |  Clamps a `name` so that it is not less than `name` and not more than `name` .          |
| double | `Clamp` ( double <ins>value</ins>, double <ins>min</ins>, double <ins>max</ins> ) |  Clamps a `name` so that it is not less than `name` and not more than `name` .          |
| float | `Cos` ( float <ins>s</ins> ) |  Returns the cosine of angle `name` in radians.      |
| double | `Cos` ( double <ins>s</ins> ) |  Returns the cosine of angle `name` in radians.      |
| float | `Cosh` ( float <ins>s</ins> ) |  Returns the hyperbolic cosine of angle `name` in radians.      |
| double | `Cosh` ( double <ins>s</ins> ) |  Returns the hyperbolic cosine of angle `name` in radians.      |
| float | `CubicInterpolate` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>pre</ins>, float <ins>post</ins>, float <ins>weight</ins> ) |  Cubic interpolates between two values by the factor defined in `name` with pre and post values.              |
| double | `CubicInterpolate` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>pre</ins>, double <ins>post</ins>, double <ins>weight</ins> ) |  Cubic interpolates between two values by the factor defined in `name` with pre and post values.              |
| float | `CubicInterpolateAngle` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>pre</ins>, float <ins>post</ins>, float <ins>weight</ins> ) |  Cubic interpolates between two rotation values with shortest path  by the factor defined in `name` with pre and post values.  See also `LerpAngle` .              |
| double | `CubicInterpolateAngle` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>pre</ins>, double <ins>post</ins>, double <ins>weight</ins> ) |  Cubic interpolates between two rotation values with shortest path  by the factor defined in `name` with pre and post values.  See also `LerpAngle` .              |
| float | `CubicInterpolateInTime` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>pre</ins>, float <ins>post</ins>, float <ins>weight</ins>, float <ins>toT</ins>, float <ins>preT</ins>, float <ins>postT</ins> ) |  Cubic interpolates between two values by the factor defined in `name` with pre and post values.  It can perform smoother interpolation than `CubicInterpolate` by the time values.                    |
| double | `CubicInterpolateInTime` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>pre</ins>, double <ins>post</ins>, double <ins>weight</ins>, double <ins>toT</ins>, double <ins>preT</ins>, double <ins>postT</ins> ) |  Cubic interpolates between two values by the factor defined in `name` with pre and post values.  It can perform smoother interpolation than `CubicInterpolate` by the time values.                    |
| float | `CubicInterpolateAngleInTime` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>pre</ins>, float <ins>post</ins>, float <ins>weight</ins>, float <ins>toT</ins>, float <ins>preT</ins>, float <ins>postT</ins> ) |  Cubic interpolates between two rotation values with shortest path  by the factor defined in `name` with pre and post values.  See also `LerpAngle` .  It can perform smoother interpolation than `CubicInterpolateAngle` by the time values.                    |
| double | `CubicInterpolateAngleInTime` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>pre</ins>, double <ins>post</ins>, double <ins>weight</ins>, double <ins>toT</ins>, double <ins>preT</ins>, double <ins>postT</ins> ) |  Cubic interpolates between two rotation values with shortest path  by the factor defined in `name` with pre and post values.  See also `LerpAngle` .  It can perform smoother interpolation than `CubicInterpolateAngle` by the time values.                    |
| float | `BezierInterpolate` ( float <ins>start</ins>, float <ins>control1</ins>, float <ins>control2</ins>, float <ins>end</ins>, float <ins>t</ins> ) |  Returns the point at the given `name` on a one-dimensional Bezier curve defined by  the given `name` , `name` , and `name` points.              |
| double | `BezierInterpolate` ( double <ins>start</ins>, double <ins>control1</ins>, double <ins>control2</ins>, double <ins>end</ins>, double <ins>t</ins> ) |  Returns the point at the given `name` on a one-dimensional Bezier curve defined by  the given `name` , `name` , and `name` points.              |
| float | `BezierDerivative` ( float <ins>start</ins>, float <ins>control1</ins>, float <ins>control2</ins>, float <ins>end</ins>, float <ins>t</ins> ) |  Returns the derivative at the given `name` on a one dimensional Bezier curve defined by  the given `name` , `name` , and `name` points.              |
| double | `BezierDerivative` ( double <ins>start</ins>, double <ins>control1</ins>, double <ins>control2</ins>, double <ins>end</ins>, double <ins>t</ins> ) |  Returns the derivative at the given `name` on a one dimensional Bezier curve defined by  the given `name` , `name` , and `name` points.              |
| float | `DbToLinear` ( float <ins>db</ins> ) |  Converts from decibels to linear energy (audio).        |
| double | `DbToLinear` ( double <ins>db</ins> ) |  Converts from decibels to linear energy (audio).        |
| float | `DegToRad` ( float <ins>deg</ins> ) |  Converts an angle expressed in degrees to radians.      |
| double | `DegToRad` ( double <ins>deg</ins> ) |  Converts an angle expressed in degrees to radians.      |
| float | `Ease` ( float <ins>s</ins>, float <ins>curve</ins> ) |  Easing function, based on exponent. The `name` values are: `0` is constant, `1` is linear, `0` to `1` is ease-in, `1` or more is ease-out.  Negative values are in-out/out-in.        |
| double | `Ease` ( double <ins>s</ins>, double <ins>curve</ins> ) |  Easing function, based on exponent. The `name` values are: `0` is constant, `1` is linear, `0` to `1` is ease-in, `1` or more is ease-out.  Negative values are in-out/out-in.        |
| float | `Exp` ( float <ins>s</ins> ) |  The natural exponential function. It raises the mathematical  constant `e` to the power of `name` and returns it.      |
| double | `Exp` ( double <ins>s</ins> ) |  The natural exponential function. It raises the mathematical  constant `e` to the power of `name` and returns it.      |
| float | `Floor` ( float <ins>s</ins> ) |  Rounds `name` downward (towards negative infinity).      |
| double | `Floor` ( double <ins>s</ins> ) |  Rounds `name` downward (towards negative infinity).      |
| float | `InverseLerp` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>weight</ins> ) |  Returns a normalized value considering the given range.  This is the opposite of `Lerp` .          |
| double | `InverseLerp` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>weight</ins> ) |  Returns a normalized value considering the given range.  This is the opposite of `Lerp` .          |
| bool | `IsEqualApprox` ( float <ins>a</ins>, float <ins>b</ins> ) |  Returns `true` if `name` and `name` are approximately equal  to each other.  The comparison is done using a tolerance calculation with `Epsilon` .        |
| bool | `IsEqualApprox` ( double <ins>a</ins>, double <ins>b</ins> ) |  Returns `true` if `name` and `name` are approximately equal  to each other.  The comparison is done using a tolerance calculation with `Epsilon` .        |
| bool | `IsFinite` ( float <ins>s</ins> ) |  Returns whether `name` is a finite value, i.e. it is not `NaN` , positive infinite, or negative infinity.      |
| bool | `IsFinite` ( double <ins>s</ins> ) |  Returns whether `name` is a finite value, i.e. it is not `NaN` , positive infinite, or negative infinity.      |
| bool | `IsInf` ( float <ins>s</ins> ) |  Returns whether `name` is an infinity value (either positive infinity or negative infinity).      |
| bool | `IsInf` ( double <ins>s</ins> ) |  Returns whether `name` is an infinity value (either positive infinity or negative infinity).      |
| bool | `IsNaN` ( float <ins>s</ins> ) |  Returns whether `name` is a `NaN` ("Not a Number" or invalid) value.      |
| bool | `IsNaN` ( double <ins>s</ins> ) |  Returns whether `name` is a `NaN` ("Not a Number" or invalid) value.      |
| bool | `IsZeroApprox` ( float <ins>s</ins> ) |  Returns `true` if `name` is zero or almost zero.  The comparison is done using a tolerance calculation with `Epsilon` .   This method is faster than using `IsEqualApprox` with  one value as zero.      |
| bool | `IsZeroApprox` ( double <ins>s</ins> ) |  Returns `true` if `name` is zero or almost zero.  The comparison is done using a tolerance calculation with `Epsilon` .   This method is faster than using `IsEqualApprox` with  one value as zero.      |
| float | `Lerp` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>weight</ins> ) |  Linearly interpolates between two values by a normalized value.  This is the opposite `InverseLerp` .          |
| double | `Lerp` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>weight</ins> ) |  Linearly interpolates between two values by a normalized value.  This is the opposite `InverseLerp` .          |
| float | `LerpAngle` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>weight</ins> ) |  Linearly interpolates between two angles (in radians) by a normalized value.   Similar to `Lerp` ,  but interpolates correctly when the angles wrap around `Tau` .          |
| double | `LerpAngle` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>weight</ins> ) |  Linearly interpolates between two angles (in radians) by a normalized value.   Similar to `Lerp` ,  but interpolates correctly when the angles wrap around `Tau` .          |
| float | `LinearToDb` ( float <ins>linear</ins> ) |  Converts from linear energy to decibels (audio).  This can be used to implement volume sliders that behave as expected (since volume isn't linear).          |
| double | `LinearToDb` ( double <ins>linear</ins> ) |  Converts from linear energy to decibels (audio).  This can be used to implement volume sliders that behave as expected (since volume isn't linear).          |
| float | `Log` ( float <ins>s</ins> ) |  Natural logarithm. The amount of time needed to reach a certain level of continuous growth.   Note: This is not the same as the "log" function on most calculators, which uses a base 10 logarithm.      |
| double | `Log` ( double <ins>s</ins> ) |  Natural logarithm. The amount of time needed to reach a certain level of continuous growth.   Note: This is not the same as the "log" function on most calculators, which uses a base 10 logarithm.      |
| int | `Max` ( int <ins>a</ins>, int <ins>b</ins> ) |  Returns the maximum of two values.        |
| float | `Max` ( float <ins>a</ins>, float <ins>b</ins> ) |  Returns the maximum of two values.        |
| double | `Max` ( double <ins>a</ins>, double <ins>b</ins> ) |  Returns the maximum of two values.        |
| int | `Min` ( int <ins>a</ins>, int <ins>b</ins> ) |  Returns the minimum of two values.        |
| float | `Min` ( float <ins>a</ins>, float <ins>b</ins> ) |  Returns the minimum of two values.        |
| double | `Min` ( double <ins>a</ins>, double <ins>b</ins> ) |  Returns the minimum of two values.        |
| float | `MoveToward` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>delta</ins> ) |  Moves `name` toward `name` by the `name` value.   Use a negative `name` value to move away.          |
| double | `MoveToward` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>delta</ins> ) |  Moves `name` toward `name` by the `name` value.   Use a negative `name` value to move away.          |
| int | `NearestPo2` ( int <ins>value</ins> ) |  Returns the nearest larger power of 2 for the integer `name` .      |
| int | `PosMod` ( int <ins>a</ins>, int <ins>b</ins> ) |  Performs a canonical Modulus operation, where the output is on the range [0, `name` ).        |
| float | `PosMod` ( float <ins>a</ins>, float <ins>b</ins> ) |  Performs a canonical Modulus operation, where the output is on the range [0, `name` ).        |
| double | `PosMod` ( double <ins>a</ins>, double <ins>b</ins> ) |  Performs a canonical Modulus operation, where the output is on the range [0, `name` ).        |
| float | `Pow` ( float <ins>x</ins>, float <ins>y</ins> ) |  Returns the result of `name` raised to the power of `name` .        |
| double | `Pow` ( double <ins>x</ins>, double <ins>y</ins> ) |  Returns the result of `name` raised to the power of `name` .        |
| float | `RadToDeg` ( float <ins>rad</ins> ) |  Converts an angle expressed in radians to degrees.      |
| double | `RadToDeg` ( double <ins>rad</ins> ) |  Converts an angle expressed in radians to degrees.      |
| float | `Remap` ( float <ins>value</ins>, float <ins>inFrom</ins>, float <ins>inTo</ins>, float <ins>outFrom</ins>, float <ins>outTo</ins> ) |  Maps a `name` from [ `name` , `name` ]  to [ `name` , `name` ].              |
| double | `Remap` ( double <ins>value</ins>, double <ins>inFrom</ins>, double <ins>inTo</ins>, double <ins>outFrom</ins>, double <ins>outTo</ins> ) |  Maps a `name` from [ `name` , `name` ]  to [ `name` , `name` ].              |
| float | `RotateToward` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>delta</ins> ) |  Rotates `name` toward `name` by the `name` amount. Will not go past `name` .  Similar to `MoveToward` but interpolates correctly when the angles wrap around `Tau` .  If `name` is negative, this function will rotate away from `name` , toward the opposite angle, and will not go past the opposite angle.          |
| double | `RotateToward` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>delta</ins> ) |  Rotates `name` toward `name` by the `name` amount. Will not go past `name` .  Similar to `MoveToward` but interpolates correctly when the angles wrap around `Tau` .  If `name` is negative, this function will rotate away from `name` , toward the opposite angle, and will not go past the opposite angle.          |
| float | `Round` ( float <ins>s</ins> ) |  Rounds `name` to the nearest whole number,  with halfway cases rounded towards the nearest multiple of two.      |
| double | `Round` ( double <ins>s</ins> ) |  Rounds `name` to the nearest whole number,  with halfway cases rounded towards the nearest multiple of two.      |
| int | `Sign` ( int <ins>s</ins> ) |  Returns the sign of `name` : `-1` or `1` .  Returns `0` if `name` is `0` .      |
| int | `Sign` ( float <ins>s</ins> ) |  Returns the sign of `name` : `-1` or `1` .  Returns `0` if `name` is `0` .      |
| int | `Sign` ( double <ins>s</ins> ) |  Returns the sign of `name` : `-1` or `1` .  Returns `0` if `name` is `0` .      |
| float | `Sin` ( float <ins>s</ins> ) |  Returns the sine of angle `name` in radians.      |
| double | `Sin` ( double <ins>s</ins> ) |  Returns the sine of angle `name` in radians.      |
| float | `Sinh` ( float <ins>s</ins> ) |  Returns the hyperbolic sine of angle `name` in radians.      |
| double | `Sinh` ( double <ins>s</ins> ) |  Returns the hyperbolic sine of angle `name` in radians.      |
| float | `SmoothStep` ( float <ins>from</ins>, float <ins>to</ins>, float <ins>weight</ins> ) |  Returns a number smoothly interpolated between `name` and `name` ,  based on the `name` . Similar to `Lerp` ,  but interpolates faster at the beginning and slower at the end.          |
| double | `SmoothStep` ( double <ins>from</ins>, double <ins>to</ins>, double <ins>weight</ins> ) |  Returns a number smoothly interpolated between `name` and `name` ,  based on the `name` . Similar to `Lerp` ,  but interpolates faster at the beginning and slower at the end.          |
| float | `Sqrt` ( float <ins>s</ins> ) |  Returns the square root of `name` , where `name` is a non-negative number.   If you need negative inputs, use `System.Numerics.Complex` .      |
| double | `Sqrt` ( double <ins>s</ins> ) |  Returns the square root of `name` , where `name` is a non-negative number.   If you need negative inputs, use `System.Numerics.Complex` .      |
| int | `StepDecimals` ( double <ins>step</ins> ) |  Returns the position of the first non-zero digit, after the  decimal point. Note that the maximum return value is 10,  which is a design decision in the implementation.      |
| float | `Snapped` ( float <ins>s</ins>, float <ins>step</ins> ) |  Snaps float value `name` to a given `name` .  This can also be used to round a floating point number to an arbitrary number of decimals.        |
| double | `Snapped` ( double <ins>s</ins>, double <ins>step</ins> ) |  Snaps float value `name` to a given `name` .  This can also be used to round a floating point number to an arbitrary number of decimals.        |
| float | `Tan` ( float <ins>s</ins> ) |  Returns the tangent of angle `name` in radians.      |
| double | `Tan` ( double <ins>s</ins> ) |  Returns the tangent of angle `name` in radians.      |
| float | `Tanh` ( float <ins>s</ins> ) |  Returns the hyperbolic tangent of angle `name` in radians.      |
| double | `Tanh` ( double <ins>s</ins> ) |  Returns the hyperbolic tangent of angle `name` in radians.      |
| int | `Wrap` ( int <ins>value</ins>, int <ins>min</ins>, int <ins>max</ins> ) |  Wraps `name` between `name` and `name` .  Usable for creating loop-alike behavior or infinite surfaces.  If `name` is `0` , this is equivalent  to `PosMod` , so prefer using that instead.          |
| float | `Wrap` ( float <ins>value</ins>, float <ins>min</ins>, float <ins>max</ins> ) |  Wraps `name` between `name` and `name` .  Usable for creating loop-alike behavior or infinite surfaces.  If `name` is `0` , this is equivalent  to `PosMod` , so prefer using that instead.          |
| double | `Wrap` ( double <ins>value</ins>, double <ins>min</ins>, double <ins>max</ins> ) |  Wraps `name` between `name` and `name` .  Usable for creating loop-alike behavior or infinite surfaces.  If `name` is `0` , this is equivalent  to `PosMod` , so prefer using that instead.          |
| float | `PingPong` ( float <ins>value</ins>, float <ins>length</ins> ) |  Returns the `name` wrapped between `0` and the `name` .  If the limit is reached, the next value the function returned is decreased to the `0` side  or increased to the `name` side (like a triangle wave).  If `name` is less than zero, it becomes positive.        |
| double | `PingPong` ( double <ins>value</ins>, double <ins>length</ins> ) |  Returns the `name` wrapped between `0` and the `name` .  If the limit is reached, the next value the function returned is decreased to the `0` side  or increased to the `name` side (like a triangle wave).  If `name` is less than zero, it becomes positive.        |
| int | `DecimalCount` ( double <ins>s</ins> ) |  Returns the amount of digits after the decimal place.      |
| int | `DecimalCount` ( decimal <ins>s</ins> ) |  Returns the amount of digits after the decimal place.      |
| int | `CeilToInt` ( float <ins>s</ins> ) |  Rounds `name` upward (towards positive infinity).   This is the same as `Ceil` , but returns an `int` .      |
| int | `CeilToInt` ( double <ins>s</ins> ) |  Rounds `name` upward (towards positive infinity).   This is the same as `Ceil` , but returns an `int` .      |
| int | `FloorToInt` ( float <ins>s</ins> ) |  Rounds `name` downward (towards negative infinity).   This is the same as `Floor` , but returns an `int` .      |
| int | `FloorToInt` ( double <ins>s</ins> ) |  Rounds `name` downward (towards negative infinity).   This is the same as `Floor` , but returns an `int` .      |
| int | `RoundToInt` ( float <ins>s</ins> ) |  Rounds `name` to the nearest whole number.   This is the same as `Round` , but returns an `int` .      |
| int | `RoundToInt` ( double <ins>s</ins> ) |  Rounds `name` to the nearest whole number.   This is the same as `Round` , but returns an `int` .      |
| (float Sin, float Cos) | `SinCos` ( float <ins>s</ins> ) |  Returns the sine and cosine of angle `name` in radians.      |
| (double Sin, double Cos) | `SinCos` ( double <ins>s</ins> ) |  Returns the sine and cosine of angle `name` in radians.      |
| bool | `IsEqualApprox` ( float <ins>a</ins>, float <ins>b</ins>, float <ins>tolerance</ins> ) |  Returns `true` if `name` and `name` are approximately  equal to each other.  The comparison is done using the provided tolerance value.  If you want the tolerance to be calculated for you, use `IsEqualApprox` .          |
| bool | `IsEqualApprox` ( double <ins>a</ins>, double <ins>b</ins>, double <ins>tolerance</ins> ) |  Returns `true` if `name` and `name` are approximately  equal to each other.  The comparison is done using the provided tolerance value.  If you want the tolerance to be calculated for you, use `IsEqualApprox` .          |
