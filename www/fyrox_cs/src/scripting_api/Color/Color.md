# Color
struct in [Color](../Color.md)

## Constructors
| Signature | Description |
|---|---|
| ( byte <ins>r</ins>, byte <ins>g</ins>, byte <ins>b</ins>, byte <ins>a</ins> ) |  |

## Methods
| Return Type | Signature | Description |
|---|---|---|
| [Color](../Color/Color.md) | `SrgbToLinear` (  ) |  |
| (float, float, float, float) | `SrgbToLinearF32` (  ) |  |
| [Color](../Color/Color.md) | `LinearToSrgb` (  ) |  |
| (float, float, float, float) | `AsFloatRgba` (  ) |  |
| (float, float, float) | `AsFloatRgb` (  ) |  |
| [Color](../Color/Color.md) | `ToOpaque` (  ) |  |
| [Color](../Color/Color.md) | `Lerp` ( [Color](../Color/Color.md) <ins>other</ins>, float <ins>t</ins> ) |  |
| [Color](../Color/Color.md) | `WithNewAlpha` ( byte <ins>a</ins> ) |  |

## Static Methods
| Return Type | Signature | Description |
|---|---|---|
| [Color](../Color/Color.md) | `Opaque` ( byte <ins>r</ins>, byte <ins>g</ins>, byte <ins>b</ins> ) |  |
| [Color](../Color/Color.md) | `Repeat` ( byte <ins>c</ins> ) |  |
| [Color](../Color/Color.md) | `RepeatOpaque` ( byte <ins>c</ins> ) |  |
| [Color](../Color/Color.md) | `FromRgba` ( byte <ins>r</ins>, byte <ins>g</ins>, byte <ins>b</ins>, byte <ins>a</ins> ) |  |

## Operators
| Return Type | Signature | Description |
|---|---|---|
| [Color](../Color/Color.md) | `+` ( [Color](../Color/Color.md) <ins>a</ins>, [Color](../Color/Color.md) <ins>b</ins> ) |  |
| [Color](../Color/Color.md) | `-` ( [Color](../Color/Color.md) <ins>a</ins>, [Color](../Color/Color.md) <ins>b</ins> ) |  |
