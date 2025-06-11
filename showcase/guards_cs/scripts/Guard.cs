
[Uuid("9f8183d3-2a4a-4951-a6e6-5fbc9c479e2e")]
public class Guard : NodeScript
{
    private float _reloadDelaySec;
    private float _gunHeight;
    private float _switchWaypointTimeoutSec;
    private Prefab _bulletPrefab;
    private float _initialBulletVelocity;
    private float _attackRange;
    private float _beaconReachedDistance;
    private float _movePower;
 
    [HideInInspector] [Transient] private float _reloadingSec;   
    [HideInInspector] [Transient] private float _waypointSec;
    [HideInInspector] [Transient] private Vector3? _currentWaypoint;
    [HideInInspector] [Transient] private Node _collider;
    [HideInInspector] [Transient] private int _id;

    private const int FRACTION_GUARDS = 1;

    public void Init(int id)
    {
        this._id = id;
    }

    private bool TryAttackPlayer()
    {
        var playerPos = GlobalScript.Get<Game>().Player.GlobalPosition;
        var selfPos = Node.GlobalPosition;
        var sightVector = playerPos - selfPos;

        if (CanSeePlayer(playerPos, sightVector))
        {
            Bullet.Spawn(new Bullet.BulletSeed
            {
                Prefab = _bulletPrefab,
                Origin = selfPos + Vector3.Up * _gunHeight,
                Direction = sightVector,
                InitialVelocity = _initialBulletVelocity,
                AuthorCollider = _collider,
                Range = _attackRange,
                Fraction = FRACTION_GUARDS
            });
            _reloadingSec = _reloadDelaySec;
            return true;
        }

        return false;
    }

    private bool CanSeePlayer(Vector3 playerPos, Vector3 sightVector)
    {
        var results = Physics.CastRay(new RayCastOptions
        {
            RayOrigin = playerPos,
            RayDirection = sightVector.Normalized(),
            MaxLen = sightVector.Length(),
            SortResults = true
        });
        foreach (var hit in results)
        {
            var node = hit.Collider;
            if (node != _collider)
            {
                while (node.Alive)
                {
                    if (node.FindScript<Player>() != null)
                    {
                        return true;
                    }

                    node = node.Parent;
                }

                return false;
            }
        }

        return false;
    }

    private void MoveToWaypoint(float dt)
    {
        _waypointSec += dt;
        if (_waypointSec > _switchWaypointTimeoutSec)
        {
            _currentWaypoint = null;
            _waypointSec = 0.0f;
        }

        if (_currentWaypoint == null)
        {
            var beacons = GlobalScript.Get<Game>().Beacons;
            _currentWaypoint = beacons[new Random().Next(beacons.Count)];
        }

        var vectorToBeacon = _currentWaypoint.Value - Node.LocalPosition;
        if (vectorToBeacon.Length() < _beaconReachedDistance)
        {
            _currentWaypoint = null;
        }
        else
        {
            Node.AsRigidBody().Value.ApplyForce(vectorToBeacon.Normalized() * _movePower);
        }
    }

    protected override void OnInit()
    {
        _collider = Node.FindColliderInChildren() ?? throw new Exception("Collider not found under Guard node");
    }

    protected override void OnStart()
    {
        Node.SubscribeTo<BulletHitMessage>();
    }

    protected override void OnUpdate(float dt)
    {
        if (_reloadingSec > 0.0f)
        {
            _reloadingSec -= dt;
        }

        if (_reloadingSec > 0.0f || !TryAttackPlayer())
        {
            MoveToWaypoint(dt);
        }
    }

    protected override void OnMessage(object message)
    {
        if (message is BulletHitMessage hit && hit.Fraction != FRACTION_GUARDS)
        {
            Node.Destroy();
            GlobalScript.Get<Game>().IncFrags();
            Console.WriteLine("Guard killed!");
        }
    }
}