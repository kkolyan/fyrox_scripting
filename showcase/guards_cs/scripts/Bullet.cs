
[Uuid("12371d19-9f1a-4286-8486-add4ebaadaec")]
public class Bullet : NodeScript
{
    private Vector3 _velocity;
    private float _remainingSec;
    private Node _authorCollider;
    private float _fraction;

    public struct BulletSeed
    {
        public Prefab Prefab;
        public Vector3 Origin;
        public Vector3 Direction;
        public float InitialVelocity;
        public Node AuthorCollider;
        public float Range;
        public int Fraction;
    }

    private void Init(BulletSeed seed)
    {
        _velocity = seed.Direction.Normalized() * seed.InitialVelocity;
        _remainingSec = seed.Range / seed.InitialVelocity;
        _authorCollider = seed.AuthorCollider;
        _fraction = seed.Fraction;
    }

    public static void Spawn(BulletSeed seed)
    {
        var orientation = Basis.LookingAt(seed.Direction, Vector3.Up).GetRotationQuaternion();
        var bullet = seed.Prefab.InstantiateAt(seed.Origin, orientation);
        bullet.FindScript<Bullet>().Init(seed);
    }

    protected override void OnUpdate(float deltaTime)
    {
        _remainingSec -= deltaTime;
        if (_remainingSec <= 0.0f)
        {
            Node.Destroy();
            return;
        }

        List<Intersection> results = Physics.CastRay(new RayCastOptions
        {
            RayOrigin = Node.LocalPosition,
            RayDirection = _velocity.Normalized(),
            MaxLen = _velocity.Length() * deltaTime,
            SortResults = true
        });

        foreach (var hit in results)
        {
            if (hit.Collider != _authorCollider)
            {
                // scene from the Lua version of game is used, and Lua stores any number as f32
                var fraction = (int)this._fraction;

                hit.Collider.SendHierarchical(RoutingStrategy.Up, new BulletHitMessage { Fraction = fraction });
                Node.Destroy();
                return;
            }
        }

        Node.LocalPosition += _velocity * deltaTime;
    }
}