[Uuid("c5671d19-9f1a-4286-8486-add4ebaadaec")]
public class Player : NodeScript
{
    private float _sensitivity;
    private Node _camera;
    private float _power;
    private Prefab _bullet;
    private float _initialBulletVelocity;
    private float _shootingRange;
    private float _reloadDelaySec;

    [HideInInspector] [Transient] private float _reloadSec;

    [HideInInspector] [Transient] private bool _published;

    [HideInInspector] [Transient] private Node _collider;

    [HideInInspector] [Transient] private float _aimY;

    private const int FractionPlayer = 0;

    protected override void OnInit()
    {
        Window.CursorGrab = CursorGrabMode.Confined;
        _collider = Node.FindColliderInChildren() ?? throw new Exception("player collider missing");
    }

    protected override void OnStart()
    {
        Node.SubscribeTo<BulletHitMessage>();
    }

    protected override void OnMessage(object message)
    {
        if (message is BulletHitMessage hitMessage && hitMessage.Fraction != FractionPlayer)
        {
            GlobalScript.Get<Game>().IncWounds();
            Console.WriteLine("player wounded!");
        }
    }

    protected override void OnUpdate(float dt)
    {
        if (_reloadSec > 0.0f)
        {
            _reloadSec -= dt;
        }

        if (!_published)
        {
            _published = true;
            GlobalScript.Get<Game>().Player = Node;
        }

        if (Input.IsMouseButtonPressed(Input.MouseLeft))
        {
            if (_reloadSec <= 0.0f)
            {
                _reloadSec = _reloadDelaySec;
                Fire();
            }
        }

        var moveDelta = Vector3.Zero;

        if (Input.IsKeyPressed(KeyCode.W))
        {
            moveDelta.Z += 1.0f;
        }

        if (Input.IsKeyPressed(KeyCode.S))
        {
            moveDelta.Z -= 1.0f;
        }

        if (Input.IsKeyPressed(KeyCode.A))
        {
            moveDelta.X += 1.0f;
        }

        if (Input.IsKeyPressed(KeyCode.D))
        {
            moveDelta.X -= 1.0f;
        }

        Turn(-Input.MouseMove.X);
        Aim(Input.MouseMove.Y);

        if (moveDelta.Length() > Mathf.Epsilon)
        {
            moveDelta = moveDelta.Normalized();
        }

        Node.AsRigidBody().Value.ApplyForce(Node.LocalRotation * moveDelta * _power);
    }

    private void Turn(float x)
    {
        Node.LocalRotation *= new Quaternion(Vector3.Up, _sensitivity * x);
    }

    private void Aim(float y)
    {
        _aimY += y * _sensitivity;
        _aimY = Mathf.Clamp(_aimY, -MathF.PI / 2.0f, MathF.PI / 2.0f);

        _camera.LocalRotation = new Quaternion(Vector3.Left, _aimY);
    }

    private void Fire()
    {
        Bullet.Spawn(new Bullet.BulletSeed
        {
            Prefab = _bullet,
            Origin = _camera.GlobalPosition,
            Direction = _camera.GlobalRotation * Vector3.Forward,
            InitialVelocity = _initialBulletVelocity,
            AuthorCollider = _collider,
            Range = _shootingRange,
            Fraction = FractionPlayer,
        });
    }
}