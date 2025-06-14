// ReSharper disable UnusedMember.Global

namespace FyroxLite;

using System.Runtime.InteropServices;

[StructLayout(LayoutKind.Sequential, Pack = 1)]
public struct Color
{
    public byte R;
    public byte G;
    public byte B;
    public byte A;

    public Color(byte r, byte g, byte b, byte a)
    {
        R = r;
        G = g;
        B = b;
        A = a;
    }

    public static readonly Color WHITE = Repeat(255);
    public static readonly Color BLACK = Opaque(0, 0, 0);
    public static readonly Color RED = Opaque(255, 0, 0);
    public static readonly Color GREEN = Opaque(0, 255, 0);
    public static readonly Color BLUE = Opaque(0, 0, 255);
    public static readonly Color TRANSPARENT = Repeat(0);
    public static readonly Color MAROON = Opaque(128, 0, 0);
    public static readonly Color DARK_RED = Opaque(139, 0, 0);
    public static readonly Color BROWN = Opaque(165, 42, 42);
    public static readonly Color FIREBRICK = Opaque(178, 34, 34);
    public static readonly Color CRIMSON = Opaque(220, 20, 60);
    public static readonly Color TOMATO = Opaque(255, 99, 71);
    public static readonly Color CORAL = Opaque(255, 127, 80);
    public static readonly Color INDIAN_RED = Opaque(205, 92, 92);
    public static readonly Color LIGHT_CORAL = Opaque(240, 128, 128);
    public static readonly Color DARK_SALMON = Opaque(233, 150, 122);
    public static readonly Color SALMON = Opaque(250, 128, 114);
    public static readonly Color LIGHT_SALMON = Opaque(255, 160, 122);
    public static readonly Color ORANGE_RED = Opaque(255, 69, 0);
    public static readonly Color DARK_ORANGE = Opaque(255, 140, 0);
    public static readonly Color ORANGE = Opaque(255, 165, 0);
    public static readonly Color GOLD = Opaque(255, 215, 0);
    public static readonly Color DARK_GOLDEN_ROD = Opaque(184, 134, 11);
    public static readonly Color GOLDEN_ROD = Opaque(218, 165, 32);
    public static readonly Color PALE_GOLDEN_ROD = Opaque(238, 232, 170);
    public static readonly Color DARK_KHAKI = Opaque(189, 183, 107);
    public static readonly Color KHAKI = Opaque(240, 230, 140);
    public static readonly Color OLIVE = Opaque(128, 128, 0);
    public static readonly Color YELLOW = Opaque(255, 255, 0);
    public static readonly Color YELLOW_GREEN = Opaque(154, 205, 50);
    public static readonly Color DARK_OLIVE_GREEN = Opaque(85, 107, 47);
    public static readonly Color OLIVE_DRAB = Opaque(107, 142, 35);
    public static readonly Color LAWN_GREEN = Opaque(124, 252, 0);
    public static readonly Color CHARTREUSE = Opaque(127, 255, 0);
    public static readonly Color GREEN_YELLOW = Opaque(173, 255, 47);
    public static readonly Color DARK_GREEN = Opaque(0, 100, 0);
    public static readonly Color FOREST_GREEN = Opaque(34, 139, 34);
    public static readonly Color LIME = Opaque(0, 255, 0);
    public static readonly Color LIME_GREEN = Opaque(50, 205, 50);
    public static readonly Color LIGHT_GREEN = Opaque(144, 238, 144);
    public static readonly Color PALE_GREEN = Opaque(152, 251, 152);
    public static readonly Color DARK_SEA_GREEN = Opaque(143, 188, 143);
    public static readonly Color MEDIUM_SPRING_GREEN = Opaque(0, 250, 154);
    public static readonly Color SPRING_GREEN = Opaque(0, 255, 127);
    public static readonly Color SEA_GREEN = Opaque(46, 139, 87);
    public static readonly Color MEDIUM_AQUA_MARINE = Opaque(102, 205, 170);
    public static readonly Color MEDIUM_SEA_GREEN = Opaque(60, 179, 113);
    public static readonly Color LIGHT_SEA_GREEN = Opaque(32, 178, 170);
    public static readonly Color DARK_SLATE_GRAY = Opaque(47, 79, 79);
    public static readonly Color TEAL = Opaque(0, 128, 128);
    public static readonly Color DARK_CYAN = Opaque(0, 139, 139);
    public static readonly Color AQUA = Opaque(0, 255, 255);
    public static readonly Color CYAN = Opaque(0, 255, 255);
    public static readonly Color LIGHT_CYAN = Opaque(224, 255, 255);

    public static Color Opaque(byte r, byte g, byte b) => new(r, g, b, 255);

    public static Color Repeat(byte c) => new(c, c, c, c);

    public static Color RepeatOpaque(byte c) => new(c, c, c, 255);

    public static Color FromRgba(byte r, byte g, byte b, byte a) => new(r, g, b, a);

    public Color SrgbToLinear()
    {
        return new(
            (byte)(Math.Clamp(Math.Pow(R / 255f, 2.2f), 0f, 1f) * 255f),
            (byte)(Math.Clamp(Math.Pow(G / 255f, 2.2f), 0f, 1f) * 255f),
            (byte)(Math.Clamp(Math.Pow(B / 255f, 2.2f), 0f, 1f) * 255f),
            A
        );
    }

    public (float, float, float, float) SrgbToLinearF32()
    {
        return new(
            (float)Math.Clamp(Math.Pow(R / 255f, 2.2f), 0f, 1f),
            (float)Math.Clamp(Math.Pow(G / 255f, 2.2f), 0f, 1f),
            (float)Math.Clamp(Math.Pow(B / 255f, 2.2f), 0f, 1f),
            A / 255f
        );
    }

    public Color LinearToSrgb()
    {
        return new(
            (byte)(Math.Clamp(Math.Pow(R / 255f, 1f / 2.2f), 0f, 1f) * 255f),
            (byte)(Math.Clamp(Math.Pow(G / 255f, 1f / 2.2f), 0f, 1f) * 255f),
            (byte)(Math.Clamp(Math.Pow(B / 255f, 1f / 2.2f), 0f, 1f) * 255f),
            A
        );
    }

    public (float, float, float, float) AsFloatRgba() => new(R / 255f, G / 255f, B / 255f, A / 255f);

    public (float, float, float) AsFloatRgb() => new(R / 255f, G / 255f, B / 255f);

    public Color ToOpaque() => new(R, G, B, 255);

    public Color Lerp(Color other, float t)
    {
        return new(
            (byte)(R + t * (other.R - R)),
            (byte)(G + t * (other.G - G)),
            (byte)(B + t * (other.B - B)),
            (byte)(A + t * (other.A - A))
        );
    }

    public Color WithNewAlpha(byte a) => new(R, G, B, a);

    public static Color operator +(Color a, Color b)
    {
        return new(
            (byte)Math.Min(a.R + b.R, 255),
            (byte)Math.Min(a.G + b.G, 255),
            (byte)Math.Min(a.B + b.B, 255),
            (byte)Math.Min(a.A + b.A, 255)
        );
    }

    public static Color operator -(Color a, Color b)
    {
        return new(
            (byte)Math.Max(a.R - b.R, 0),
            (byte)Math.Max(a.G - b.G, 0),
            (byte)Math.Max(a.B - b.B, 0),
            (byte)Math.Max(a.A - b.A, 0)
        );
    }
}