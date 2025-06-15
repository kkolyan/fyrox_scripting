// ReSharper disable InconsistentNaming

namespace FyroxLite;

internal partial struct NativeColor
{
    public static Color ToFacade(NativeColor value)
    {
        return Color.Color8(value._r, value._g, value._b, value._a);
    }

    public static NativeColor FromFacade(Color value)
    {
        return new NativeColor { _r = value.R8, _g = value.G8, _b = value.B8, _a = value.A8 };
    }
}