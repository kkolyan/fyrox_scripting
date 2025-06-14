# Brush
struct in [UI](../UI.md)

## Description
<p>Brush defines a way to fill an arbitrary surface.</p>

## Fields
| Name | Type | Description |
|---|---|---|
| `SolidColor` | [Color](../Color/Color.md)? | <p>A brush, that fills a surface with a solid color.</p> |
| `LinearGradient` | [LinearGradient](../UI/LinearGradient.md)? | <p>A brush, that fills a surface with a linear gradient, which is defined by two points in local coordinates and a set of stop points. See [<code>GradientPoint</code>] for more info.</p> |
| `RadialGradient` | [RadialGradient](../UI/RadialGradient.md)? | <p>A brush, that fills a surface with a radial gradient, which is defined by a center point in local coordinates and a set of stop points. See [<code>GradientPoint</code>] for more info.</p> |
