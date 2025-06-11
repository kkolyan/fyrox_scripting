using System.Drawing;


public class Game : GlobalScript
{
    public Node Player;
    [Transient] public List<Vector3>? Beacons;
    private int _frags;
    private int _wounds;
    private Text _hud;

    protected override void OnGlobalInit(string? initialSceneOverride)
    {
        Scene.LoadAsync(initialSceneOverride ?? "data/scene.rgs");

        _hud = Text.New(new TextBuilder
        {
            FontSize = 40,
            Foreground = new Brush
            {
                SolidColor = Color.Black
            }
        });

        Beacons = new List<Vector3>();
    }

    protected override void OnGlobalUpdate()
    {
        if (Input.IsKeyDown(KeyCode.Escape))
        {
            Console.WriteLine("User requested exit");
            Environment.Exit(0);
        }
        _hud.TextAsync = $"Wounds: {_wounds}\nKilled Guards: {_frags}";
    }

    public void IncFrags()
    {
        _frags += 1;
    }

    public void IncWounds()
    {
        _wounds += 1;
    }
}