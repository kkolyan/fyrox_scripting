# Color
struct in [Color](../Color.md)

## Description

(code of this item is picked from Godot Engine in compliance with MIT license).

 A color represented by red, green, blue, and alpha (RGBA) components.
 The alpha component is often used for transparency.
 Values are in floating-point and usually range from 0 to 1.
 Some properties (such as
`CanvasItem.Modulate`
) may accept values
 greater than 1 (overbright or HDR colors).

 If you want to supply values in a range of 0 to 255, you should use
`Color8`
and the
`r8`
/
`g8`
/
`b8`
/
`a8`
properties.


## Constructors
| Signature | Description |
|---|---|
| ( float <ins>r</ins>, float <ins>g</ins>, float <ins>b</ins>, float <ins>a</ins> ) |  Constructs a `Color` from RGBA values, typically on the range of 0 to 1.          |
| ( [Color](../Color/Color.md) <ins>c</ins>, float <ins>a</ins> ) |  Constructs a `Color` from an existing color and an alpha value.      |
| ( uint <ins>rgba</ins> ) |  Constructs a `Color` from an unsigned 32-bit integer in RGBA format  (each byte represents a color channel).    |
| ( ulong <ins>rgba</ins> ) |  Constructs a `Color` from an unsigned 64-bit integer in RGBA format  (each word represents a color channel).    |
| ( string <ins>code</ins> ) |  Constructs a `Color` either from an HTML color code or from a  standardized color name. Supported color names are the same as the `Colors` constants.      |
| ( string <ins>code</ins>, float <ins>alpha</ins> ) |  Constructs a `Color` either from an HTML color code or from a  standardized color name, with `name` on the range of 0 to 1. Supported  color names are the same as the `Colors` constants.      |

## Properties
| Name | Type | Access | Description |
|---|---|---|---|
| `R8` | byte | get / set |  Wrapper for `R` that uses the range 0 to 255 instead of 0 to 1.  Getting is equivalent to multiplying by 255 and rounding. Setting is equivalent to dividing by 255.  |
| `G8` | byte | get / set |  Wrapper for `G` that uses the range 0 to 255 instead of 0 to 1.  Getting is equivalent to multiplying by 255 and rounding. Setting is equivalent to dividing by 255.  |
| `B8` | byte | get / set |  Wrapper for `B` that uses the range 0 to 255 instead of 0 to 1.  Getting is equivalent to multiplying by 255 and rounding. Setting is equivalent to dividing by 255.  |
| `A8` | byte | get / set |  Wrapper for `A` that uses the range 0 to 255 instead of 0 to 1.  Getting is equivalent to multiplying by 255 and rounding. Setting is equivalent to dividing by 255.  |
| `H` | float | get / set |  The HSV hue of this color, on the range 0 to 1.  Getting is a long process, refer to the source code for details. Setting uses `FromHsv` .  |
| `S` | float | get / set |  The HSV saturation of this color, on the range 0 to 1.  Getting is equivalent to the ratio between the min and max RGB value. Setting uses `FromHsv` .  |
| `V` | float | get / set |  The HSV value (brightness) of this color, on the range 0 to 1.  Getting is equivalent to using `Math.Max(float, float)` on the RGB components. Setting uses `FromHsv` .  |
| `Luminance` | float | get |  Returns the light intensity of the color, as a value between 0.0 and 1.0 (inclusive).  This is useful when determining light or dark color. Colors with a luminance smaller  than 0.5 can be generally considered dark.  Note: `Luminance` relies on the color being in the linear color space to  return an accurate relative luminance value. If the color is in the sRGB color space  use `SrgbToLinear` to convert it to the linear color space first.  |

## Methods
| Return Type | Signature | Description |
|---|---|---|
| [Color](../Color/Color.md) | `Blend` ( [Color](../Color/Color.md) <ins>over</ins> ) |  Returns a new color resulting from blending this color over another.  If the color is opaque, the result is also opaque.  The second color may have a range of alpha values.      |
| [Color](../Color/Color.md) | `Clamp` ( [Color](../Color/Color.md)? <ins>min</ins>, [Color](../Color/Color.md)? <ins>max</ins> ) |  Returns a new color with all components clamped between the  components of `name` and `name` using `Mathf.Clamp(float, float, float)` .        |
| [Color](../Color/Color.md) | `Darkened` ( float <ins>amount</ins> ) |  Returns a new color resulting from making this color darker  by the specified ratio (on the range of 0 to 1).      |
| [Color](../Color/Color.md) | `Inverted` (  ) |  Returns the inverted color: `(1 - r, 1 - g, 1 - b, a)` .    |
| [Color](../Color/Color.md) | `Lightened` ( float <ins>amount</ins> ) |  Returns a new color resulting from making this color lighter  by the specified ratio (on the range of 0 to 1).      |
| [Color](../Color/Color.md) | `Lerp` ( [Color](../Color/Color.md) <ins>to</ins>, float <ins>weight</ins> ) |  Returns the result of the linear interpolation between  this color and `name` by amount `name` .        |
| [Color](../Color/Color.md) | `LinearToSrgb` (  ) |  Returns the color converted to the sRGB color space.  This method assumes the original color is in the linear color space.  See also `SrgbToLinear` which performs the opposite operation.    |
| [Color](../Color/Color.md) | `SrgbToLinear` (  ) |  Returns the color converted to linear color space.  This method assumes the original color already is in sRGB color space.  See also `LinearToSrgb` which performs the opposite operation.    |
| uint | `ToAbgr32` (  ) |  Returns the color converted to an unsigned 32-bit integer in ABGR  format (each byte represents a color channel).  ABGR is the reversed version of the default format.    |
| ulong | `ToAbgr64` (  ) |  Returns the color converted to an unsigned 64-bit integer in ABGR  format (each word represents a color channel).  ABGR is the reversed version of the default format.    |
| uint | `ToArgb32` (  ) |  Returns the color converted to an unsigned 32-bit integer in ARGB  format (each byte represents a color channel).  ARGB is more compatible with DirectX, but not used much in Godot.    |
| ulong | `ToArgb64` (  ) |  Returns the color converted to an unsigned 64-bit integer in ARGB  format (each word represents a color channel).  ARGB is more compatible with DirectX, but not used much in Godot.    |
| uint | `ToRgba32` (  ) |  Returns the color converted to an unsigned 32-bit integer in RGBA  format (each byte represents a color channel).  RGBA is Godot's default and recommended format.    |
| ulong | `ToRgba64` (  ) |  Returns the color converted to an unsigned 64-bit integer in RGBA  format (each word represents a color channel).  RGBA is Godot's default and recommended format.    |
| string | `ToHtml` ( bool <ins>includeAlpha</ins> ) |  Returns the color's HTML hexadecimal color string in RGBA format.      |
| void | `ToHsv` ( float <ins>hue</ins>, float <ins>saturation</ins>, float <ins>value</ins> ) |  Converts a color to HSV values. This is equivalent to using each of  the `h` / `s` / `v` properties, but much more efficient.        |
| bool | `Equals` ( object? <ins>obj</ins> ) |  Returns `true` if this color and `name` are equal.      |
| bool | `Equals` ( [Color](../Color/Color.md) <ins>other</ins> ) |  Returns `true` if the colors are exactly equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.      |
| bool | `IsEqualApprox` ( [Color](../Color/Color.md) <ins>other</ins> ) |  Returns `true` if this color and `name` are approximately equal,  by running `Mathf.IsEqualApprox(float, float)` on each component.      |
| int | `GetHashCode` (  ) |  Serves as the hash function for `Color` .    |
| string | `ToString` (  ) |  Converts this `Color` to a string.    |
| string | `ToString` ( string? <ins>format</ins> ) |  Converts this `Color` to a string with the given `name` .    |

## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| [Color](../Color/Color.md) | `FromHtml` ( ReadOnlySpan <ins>rgba</ins> ) |  Constructs a `Color` from the HTML hexadecimal color string in RGBA format.      |
| [Color](../Color/Color.md) | `Color8` ( byte <ins>r8</ins>, byte <ins>g8</ins>, byte <ins>b8</ins>, byte <ins>a8</ins> ) |  Returns a color constructed from integer red, green, blue, and alpha channels.  Each channel should have 8 bits of information ranging from 0 to 255.            |
| [Color](../Color/Color.md) | `FromHsv` ( float <ins>hue</ins>, float <ins>saturation</ins>, float <ins>value</ins>, float <ins>alpha</ins> ) |  Constructs a color from an HSV profile. The `name` , `name` , and `name` are typically  between 0.0 and 1.0.            |
| [Color](../Color/Color.md) | `FromRgbe9995` ( uint <ins>rgbe</ins> ) |  Encodes a `Color` from a RGBE9995 format integer.  See `Image.Format.Rgbe9995` .      |
| [Color](../Color/Color.md) | `FromString` ( string <ins>str</ins>, [Color](../Color/Color.md) <ins>@default</ins> ) |  Constructs a color from the given string, which can be either an HTML color  code or a named color. Returns `name` if the color cannot  be inferred from the string. Supported color names are the same as the `Colors` constants.        |
| bool | `HtmlIsValid` ( ReadOnlySpan <ins>color</ins> ) |  Returns `true` if `name` is a valid HTML hexadecimal  color string. The string must be a hexadecimal value (case-insensitive) of either 3,  4, 6 or 8 digits, and may be prefixed by a hash sign ( `#` ). This method is  identical to `StringExtensions.IsValidHtmlColor(string)` .      |

## Operators
| Return Type | Signature | Description |
|---|---|---|
| [Color](../Color/Color.md) | `+` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Adds each component of the `Color` with the components of the given `Color` .        |
| [Color](../Color/Color.md) | `-` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Subtracts each component of the `Color` by the components of the given `Color` .        |
| [Color](../Color/Color.md) | `-` ( [Color](../Color/Color.md) <ins>color</ins> ) |  Inverts the given color. This is equivalent to `Colors.White - c` or `new Color(1 - c.R, 1 - c.G, 1 - c.B, 1 - c.A)` .      |
| [Color](../Color/Color.md) | `*` ( [Color](../Color/Color.md) <ins>color</ins>, float <ins>scale</ins> ) |  Multiplies each component of the `Color` by the given `float` .        |
| [Color](../Color/Color.md) | `*` ( float <ins>scale</ins>, [Color](../Color/Color.md) <ins>color</ins> ) |  Multiplies each component of the `Color` by the given `float` .        |
| [Color](../Color/Color.md) | `*` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Multiplies each component of the `Color` by the components of the given `Color` .        |
| [Color](../Color/Color.md) | `/` ( [Color](../Color/Color.md) <ins>color</ins>, float <ins>scale</ins> ) |  Divides each component of the `Color` by the given `float` .        |
| [Color](../Color/Color.md) | `/` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Divides each component of the `Color` by the components of the given `Color` .        |
| bool | `==` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Returns `true` if the colors are exactly equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
| bool | `!=` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Returns `true` if the colors are not equal.  Note: Due to floating-point precision errors, consider using `IsEqualApprox` instead, which is more reliable.        |
| bool | `<` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Compares two `Color` s by first checking if  the red value of the `name` color is less than  the red value of the `name` color.  If the red values are exactly equal, then it repeats this check  with the green values of the two colors, then with the blue values,  and then with the alpha value.  This operator is useful for sorting colors.        |
| bool | `>` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Compares two `Color` s by first checking if  the red value of the `name` color is greater than  the red value of the `name` color.  If the red values are exactly equal, then it repeats this check  with the green values of the two colors, then with the blue values,  and then with the alpha value.  This operator is useful for sorting colors.        |
| bool | `<=` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Compares two `Color` s by first checking if  the red value of the `name` color is less than  or equal to the red value of the `name` color.  If the red values are exactly equal, then it repeats this check  with the green values of the two colors, then with the blue values,  and then with the alpha value.  This operator is useful for sorting colors.        |
| bool | `>=` ( [Color](../Color/Color.md) <ins>left</ins>, [Color](../Color/Color.md) <ins>right</ins> ) |  Compares two `Color` s by first checking if  the red value of the `name` color is greater than  or equal to the red value of the `name` color.  If the red values are exactly equal, then it repeats this check  with the green values of the two colors, then with the blue values,  and then with the alpha value.  This operator is useful for sorting colors.        |
