namespace FyroxLite;

public static class FyroxDll
{
#if FYROX_EDITOR
    public const string Name = "fyroxed_cs";
#else
    public const string Name = "fyrox_lite_cs";
#endif
}