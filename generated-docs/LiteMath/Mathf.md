# Mathf
class in [FyroxLite](../README.md).[LiteMath](README.md)
## Description
(The code of this item is picked from Godot Engine).
## Constants
| Name | Type | Value | Description |
|---|---|---|---|
| `E` | float | (real_t)2.7182818284590452353602874714M |  The natural number `e` .  |
| `Sqrt2` | float | (real_t)1.4142135623730950488016887242M |  The square root of 2.  |
| `Epsilon` | float | EpsilonF |  A very small number used for float comparison with error tolerance.  1e-06 with single-precision floats, but 1e-14 if `REAL_T_IS_DOUBLE` .  |
## Static Methods
| Return Type | Signature | Description |
|---|---|---|
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
