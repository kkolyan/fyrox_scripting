// ReSharper disable InconsistentNaming

namespace FyroxLite;

internal partial struct NativeColor
{
    public static Color ToFacade(NativeColor value)
    {
        return new Color { R = value._r, G = value._g, B = value._b, A = value._a };
    }

    public static NativeColor FromFacade(Color value)
    {
        return new NativeColor { _r = value.R, _g = value.G, _b = value.B, _a = value.A };
    }
}