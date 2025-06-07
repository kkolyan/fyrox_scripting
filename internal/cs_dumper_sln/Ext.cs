public static class Ext
{
    public static T? Next<T>(this IEnumerator<T> stream)
    {
        if (stream.MoveNext())
        {
            return stream.Current;
        }

        return default;
    }

    public static IEnumerable<T> DropNulls<T>(this IEnumerable<T?> source)
    {
        return source.Where(it => it != null);
    }
}