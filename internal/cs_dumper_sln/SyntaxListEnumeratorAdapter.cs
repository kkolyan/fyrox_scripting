using System.Collections;
using Microsoft.CodeAnalysis;

public class SyntaxListEnumeratorAdapter<T>: IEnumerator<T>
where T: SyntaxNode
{
    private SyntaxList<T>.Enumerator _target;

    public SyntaxListEnumeratorAdapter(SyntaxList<T>.Enumerator target) => _target = target;

    public bool MoveNext() => _target.MoveNext();

    public void Reset() => _target.Reset();

    public T Current => _target.Current;

    object? IEnumerator.Current => Current;

    public void Dispose()
    {
    }
}