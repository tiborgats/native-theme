// The WinUI host (D4, spike W): opens an 800x600 window (the runner's display
// is 1024x768) holding one Button, always on top (the runner's desktop shows
// the out-of-box experience's privacy page above other windows), then, in
// turn, a MenuFlyout shown at the Button (once windowed, the default, and once
// constrained to the window), and a ToolTip attached to the Button (once
// opened by IsOpen, once by moving the pointer onto the Button with
// SendInput). Writes into the folder given by `--out`:
//   frames.toml        the runtime's versions, the window's bounds, the
//                      Button's bounds, the defaults the XAML does not state
//                      (RecordDefaults) and, per pass, each open popup's frame
//                      (VisualTreeHelper.GetOpenPopupsForXamlRoot, the child's
//                      TransformToVisual) and each visible top-level window of
//                      the process (the windowed popups' HWNDs)
//   desktop.toml       the desktop's visible top-level windows at the start
//   window.png         the window's content (RenderTargetBitmap)
//   popup-<pass>.png   each popup's child (RenderTargetBitmap)
//   screen-<pass>.png  the primary display (GDI), the popups on screen
// XAML coordinates are in effective pixels relative to the window's content;
// HWND rectangles and the screen captures are in physical pixels.
using System.Diagnostics;
using System.Globalization;
using System.Reflection;
using System.Runtime.InteropServices;
using System.Runtime.InteropServices.WindowsRuntime;
using System.Text;
using Microsoft.UI.Dispatching;
using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;
using Microsoft.UI.Xaml.Media.Imaging;
using Windows.Foundation;
using Windows.Graphics;
using Windows.Graphics.Imaging;
using Windows.Storage.Streams;

namespace WinUiHost;

public partial class App : Application
{
    private const int SettleMilliseconds = 1500;
    private const int EventTimeoutMilliseconds = 8000;

    private readonly Toml toml = new();
    private readonly Toml desktop = new();
    private readonly List<string> errors = new();
    private Toml.Section? result;
    private Window? window;
    private Grid? root;
    private Button? anchor;
    private DispatcherQueueTimer? watchdog;
    private string outDir = "winui-host-out";
    private bool pageBackground;
    private bool started;
    private bool finished;

    public App()
    {
        InitializeComponent();
        UnhandledException += (_, e) =>
        {
            e.Handled = true;
            errors.Add("unhandled: " + e.Exception);
            Finish(3);
        };
    }

    protected override void OnLaunched(LaunchActivatedEventArgs args)
    {
        string[] argv = Environment.GetCommandLineArgs();
        for (int i = 1; i + 1 < argv.Length; i++)
        {
            if (argv[i] == "--out")
            {
                outDir = argv[i + 1];
            }
        }
        Directory.CreateDirectory(outDir);
        result = toml.Add("[result]");

        anchor = new Button
        {
            Content = "Button",
            HorizontalAlignment = HorizontalAlignment.Center,
            VerticalAlignment = VerticalAlignment.Center,
        };
        root = new Grid();
        if (Resources.TryGetValue("ApplicationPageBackgroundThemeBrush", out object? background) && background is Brush brush)
        {
            root.Background = brush;
            pageBackground = true;
        }
        root.Children.Add(anchor);
        root.Loaded += async (_, _) =>
        {
            if (started)
            {
                return;
            }
            started = true;
            await RunAsync();
        };

        window = new Window { Title = "native-theme WinUI host", Content = root };
        window.AppWindow.MoveAndResize(new RectInt32(100, 60, 800, 600));
        if (window.AppWindow.Presenter is OverlappedPresenter presenter)
        {
            presenter.IsAlwaysOnTop = true;
        }

        watchdog = DispatcherQueue.GetForCurrentThread().CreateTimer();
        watchdog.Interval = TimeSpan.FromSeconds(150);
        watchdog.IsRepeating = false;
        watchdog.Tick += (_, _) =>
        {
            errors.Add("watchdog: the passes did not finish within 150 s");
            Finish(2);
        };
        watchdog.Start();

        MovePointer(0, 0);
        window.Activate();
        window.AppWindow.MoveInZOrderAtTop();
    }

    private async Task RunAsync()
    {
        try
        {
            await Task.Delay(SettleMilliseconds);
            RecordHost();
            RecordDesktop();
            RecordWindow();
            Toml.Section anchorSection = toml.Add("[anchor]");
            anchorSection.Str("type", anchor!.GetType().FullName ?? "");
            anchorSection.Size("actual_size", anchor.ActualWidth, anchor.ActualHeight);
            RecordBounds(anchorSection, "bounds", anchor, root);
            anchorSection.Str("png", await RenderPng(root!, "window.png", anchorSection));
            if (anchorSection.Lines.Any(line => line.StartsWith("png_error", StringComparison.Ordinal)))
            {
                errors.Add("window.png: RenderTargetBitmap failed");
            }
            CaptureScreen("screen-idle.png");
            await RecordDefaults();
            await FlyoutPass("menu_flyout", false);
            await FlyoutPass("menu_flyout_constrained", true);
            await ToolTipPass("tooltip_programmatic", false);
            await ToolTipPass("tooltip_hover", true);
        }
        catch (Exception e)
        {
            errors.Add("run: " + e);
        }
        Finish(errors.Count == 0 ? 0 : 1);
    }

    private void RecordHost()
    {
        Toml.Section host = toml.Add("[host]");
        host.Str("windows_app_sdk_package", typeof(App).Assembly.GetCustomAttributes<AssemblyMetadataAttribute>()
            .FirstOrDefault(a => a.Key == "WindowsAppSDKPackageVersion")?.Value ?? "");
        host.Str("windows_app_sdk_release", Try(() => Microsoft.Windows.ApplicationModel.WindowsAppRuntime.ReleaseInfo.AsString));
        host.Str("windows_app_sdk_runtime", Try(() => Microsoft.Windows.ApplicationModel.WindowsAppRuntime.RuntimeInfo.AsString));
        foreach (string dll in new[] { "Microsoft.UI.Xaml.dll", "Microsoft.UI.Xaml.Controls.dll", "Microsoft.WindowsAppRuntime.dll" })
        {
            string key = dll.Replace(".dll", "").Replace('.', '_').ToLowerInvariant();
            host.Str(key + "_version", FileVersion(dll));
            host.Str(key + "_loaded_from", LoadedFrom(dll));
        }
        host.Str("os_version", Environment.OSVersion.VersionString);
        host.Str("os_architecture", RuntimeInformation.OSArchitecture.ToString());
        host.Str("process_architecture", RuntimeInformation.ProcessArchitecture.ToString());
        host.Str("dotnet", RuntimeInformation.FrameworkDescription);
        host.Str("application_requested_theme", RequestedTheme.ToString());
        host.Str("root_actual_theme", root!.ActualTheme.ToString());
        host.Bool("root_background_application_page_background", pageBackground);
    }

    private void RecordWindow()
    {
        AppWindow appWindow = window!.AppWindow;
        IntPtr hwnd = WinRT.Interop.WindowNative.GetWindowHandle(window);
        Toml.Section section = toml.Add("[window]");
        section.Num("rasterization_scale", root!.XamlRoot.RasterizationScale);
        section.Int("dpi", Native.GetDpiForWindow(hwnd));
        section.Point("app_window_position", appWindow.Position.X, appWindow.Position.Y);
        section.Size("app_window_size", appWindow.Size.Width, appWindow.Size.Height);
        section.Size("app_window_client_size", appWindow.ClientSize.Width, appWindow.ClientSize.Height);
        if (Native.GetWindowRect(hwnd, out Native.RECT windowRect))
        {
            section.Rect("window_rect", windowRect);
        }
        if (Native.DwmGetWindowAttribute(hwnd, Native.DWMWA_EXTENDED_FRAME_BOUNDS, out Native.RECT frame, Marshal.SizeOf<Native.RECT>()) == 0)
        {
            section.Rect("extended_frame_bounds", frame);
        }
        Native.POINT origin = new();
        if (Native.ClientToScreen(hwnd, ref origin))
        {
            section.Point("client_origin", origin.X, origin.Y);
        }
        section.Rect("bounds", window.Bounds);
        section.Size("xaml_root_size", root.XamlRoot.Size.Width, root.XamlRoot.Size.Height);
        section.Size("content_actual_size", root.ActualWidth, root.ActualHeight);
        DisplayArea display = DisplayArea.GetFromWindowId(appWindow.Id, DisplayAreaFallback.Primary);
        section.Rect("display_outer_bounds", display.OuterBounds);
        section.Rect("display_work_area", display.WorkArea);
    }

    // Values the pinned XAML does not state (spike W's coverage report): the
    // C++ defaults of RepeatButton's timing and of the focus visual, and what
    // the AccentButtonStyle resolves to, read from controls of the running
    // WinUI. The accent button is added beside the anchor and removed again.
    private async Task RecordDefaults()
    {
        Toml.Section section = toml.Add("[defaults]");
        RepeatButton repeat = new();
        section.Int("repeat_button_delay", (uint)repeat.Delay);
        section.Int("repeat_button_interval", (uint)repeat.Interval);
        section.Str("button_focus_visual_primary_thickness", anchor!.FocusVisualPrimaryThickness.ToString());
        section.Str("button_focus_visual_secondary_thickness", anchor.FocusVisualSecondaryThickness.ToString());
        section.Str("button_focus_visual_margin", anchor.FocusVisualMargin.ToString());
        RecordControl(section, "button", anchor);

        if (Resources.TryGetValue("AccentButtonStyle", out object? style) && style is Style accentStyle)
        {
            Button accent = new()
            {
                Content = "Button",
                Style = accentStyle,
                HorizontalAlignment = HorizontalAlignment.Left,
                VerticalAlignment = VerticalAlignment.Top,
            };
            root!.Children.Add(accent);
            await Task.Delay(SettleMilliseconds);
            RecordControl(section, "accent_button", accent);
            root.Children.Remove(accent);
            await Task.Delay(SettleMilliseconds);
        }
        else
        {
            section.Str("accent_button_error", "AccentButtonStyle not found in the application's resources");
        }
    }

    private static void RecordControl(Toml.Section section, string name, Control control)
    {
        section.Str(name + "_padding", control.Padding.ToString());
        section.Str(name + "_border_thickness", control.BorderThickness.ToString());
        section.Str(name + "_corner_radius", control.CornerRadius.ToString());
        section.Num(name + "_font_size", control.FontSize);
        section.Str(name + "_font_family", control.FontFamily.Source);
        section.Str(name + "_font_weight", control.FontWeight.Weight.ToString(CultureInfo.InvariantCulture));
        section.Size(name + "_actual_size", control.ActualWidth, control.ActualHeight);
        section.Str(name + "_background", BrushText(control.Background));
        section.Str(name + "_foreground", BrushText(control.Foreground));
        section.Str(name + "_border_brush", BrushText(control.BorderBrush));
    }

    private static string BrushText(Brush? brush) => brush switch
    {
        null => "none",
        SolidColorBrush solid => $"#{solid.Color.A:X2}{solid.Color.R:X2}{solid.Color.G:X2}{solid.Color.B:X2} opacity {solid.Opacity.ToString("R", CultureInfo.InvariantCulture)}",
        _ => brush.GetType().FullName ?? "",
    };

    private async Task FlyoutPass(string pass, bool constrain)
    {
        MenuFlyout flyout = new() { ShouldConstrainToRootBounds = constrain };
        foreach (string text in new[] { "Cut", "Copy", "Paste" })
        {
            flyout.Items.Add(new MenuFlyoutItem { Text = text });
        }
        flyout.Items.Add(new MenuFlyoutSeparator());
        flyout.Items.Add(new MenuFlyoutItem { Text = "Select all" });

        TaskCompletionSource opened = new();
        TaskCompletionSource closed = new();
        flyout.Opened += (_, _) => opened.TrySetResult();
        flyout.Closed += (_, _) => closed.TrySetResult();

        Toml.Section section = toml.Add("[[popup]]");
        section.Str("pass", pass);
        section.Str("kind", "MenuFlyout");
        section.Str("shown_by", "MenuFlyout.ShowAt(anchor)");
        flyout.ShowAt(anchor);
        section.Bool("opened_event", await WithTimeout(opened.Task, EventTimeoutMilliseconds));
        await Task.Delay(SettleMilliseconds);
        section.Str("placement", flyout.Placement.ToString());
        section.Bool("should_constrain_to_root_bounds", flyout.ShouldConstrainToRootBounds);
        section.Bool("is_constrained_to_root_bounds", flyout.IsConstrainedToRootBounds);
        await RecordOpen(section, pass);
        flyout.Hide();
        section.Bool("closed_event", await WithTimeout(closed.Task, EventTimeoutMilliseconds));
        await Task.Delay(SettleMilliseconds);
    }

    private async Task ToolTipPass(string pass, bool hover)
    {
        ToolTip tip = new() { Content = "Tooltip" };
        TaskCompletionSource opened = new();
        TaskCompletionSource closed = new();
        tip.Opened += (_, _) => opened.TrySetResult();
        tip.Closed += (_, _) => closed.TrySetResult();
        ToolTipService.SetToolTip(anchor, tip);

        Toml.Section section = toml.Add("[[popup]]");
        section.Str("pass", pass);
        section.Str("kind", "ToolTip");
        if (hover)
        {
            IntPtr hwnd = WinRT.Interop.WindowNative.GetWindowHandle(window);
            Native.POINT origin = new();
            Native.ClientToScreen(hwnd, ref origin);
            Rect bounds = anchor!.TransformToVisual(root).TransformBounds(new Rect(0, 0, anchor.ActualWidth, anchor.ActualHeight));
            double scale = root!.XamlRoot.RasterizationScale;
            int x = origin.X + (int)Math.Round((bounds.X + bounds.Width / 2) * scale);
            int y = origin.Y + (int)Math.Round((bounds.Y + bounds.Height / 2) * scale);
            section.Str("shown_by", "SendInput moving the pointer onto the anchor's centre");
            section.Point("cursor", x, y);
            section.Bool("send_input", MovePointer(x - 20, y - 10) && MovePointer(x, y));
        }
        else
        {
            section.Str("shown_by", "ToolTip.IsOpen = true");
            tip.IsOpen = true;
        }
        section.Bool("opened_event", await WithTimeout(opened.Task, EventTimeoutMilliseconds));
        await Task.Delay(SettleMilliseconds);
        section.Str("placement", tip.Placement.ToString());
        section.Num("horizontal_offset", tip.HorizontalOffset);
        section.Num("vertical_offset", tip.VerticalOffset);
        if (tip.PlacementRect is Rect placementRect)
        {
            section.Rect("placement_rect", placementRect);
        }
        RecordBounds(section, "tooltip_bounds", tip, root);
        await RecordOpen(section, pass);
        tip.IsOpen = false;
        MovePointer(0, 0);
        section.Bool("closed_event", await WithTimeout(closed.Task, EventTimeoutMilliseconds));
        ToolTipService.SetToolTip(anchor, null);
        await Task.Delay(SettleMilliseconds);
    }

    private async Task RecordOpen(Toml.Section pass, string name)
    {
        IReadOnlyList<Popup> popups = VisualTreeHelper.GetOpenPopupsForXamlRoot(root!.XamlRoot);
        pass.Int("open_popups", (uint)popups.Count);
        int index = 0;
        foreach (Popup popup in popups)
        {
            Toml.Section section = toml.Add("[[popup.open]]");
            section.Bool("is_constrained_to_root_bounds", popup.IsConstrainedToRootBounds);
            section.Bool("should_constrain_to_root_bounds", popup.ShouldConstrainToRootBounds);
            section.Num("horizontal_offset", popup.HorizontalOffset);
            section.Num("vertical_offset", popup.VerticalOffset);
            RecordOrigin(section, "popup_origin", popup, root);
            if (popup.Child is FrameworkElement child)
            {
                section.Str("child_type", child.GetType().FullName ?? "");
                section.Size("child_actual_size", child.ActualWidth, child.ActualHeight);
                section.Str("child_margin", child.Margin.ToString());
                RecordBounds(section, "child_bounds", child, root);
                RecordBounds(section, "child_bounds_to_null", child, null);
                FrameworkElement? presenter = FindPresenter(child);
                if (presenter is not null && !ReferenceEquals(presenter, child))
                {
                    section.Str("presenter_type", presenter.GetType().FullName ?? "");
                    section.Str("presenter_margin", presenter.Margin.ToString());
                    RecordBounds(section, "presenter_bounds", presenter, root);
                }
                section.Str("png", await RenderPng(child, $"popup-{name}-{index}.png", section));
            }
            index++;
        }
        RecordWindows();
        CaptureScreen($"screen-{name}.png");
    }

    private void RecordWindows()
    {
        uint self = (uint)Environment.ProcessId;
        List<IntPtr> windows = new();
        Native.EnumWindows((hwnd, _) =>
        {
            Native.GetWindowThreadProcessId(hwnd, out uint pid);
            if (pid == self && Native.IsWindowVisible(hwnd))
            {
                windows.Add(hwnd);
            }
            return true;
        }, IntPtr.Zero);
        foreach (IntPtr hwnd in windows)
        {
            Toml.Section section = toml.Add("[[popup.hwnd]]");
            StringBuilder name = new(256);
            Native.GetClassName(hwnd, name, name.Capacity);
            section.Str("class", name.ToString());
            section.Bool("is_main_window", hwnd == WinRT.Interop.WindowNative.GetWindowHandle(window));
            if (Native.GetWindowRect(hwnd, out Native.RECT rect))
            {
                section.Rect("window_rect", rect);
            }
            if (Native.DwmGetWindowAttribute(hwnd, Native.DWMWA_EXTENDED_FRAME_BOUNDS, out Native.RECT frame, Marshal.SizeOf<Native.RECT>()) == 0)
            {
                section.Rect("extended_frame_bounds", frame);
            }
        }
    }

    private void RecordDesktop()
    {
        List<IntPtr> windows = new();
        Native.EnumWindows((hwnd, _) =>
        {
            if (Native.IsWindowVisible(hwnd))
            {
                windows.Add(hwnd);
            }
            return true;
        }, IntPtr.Zero);
        foreach (IntPtr hwnd in windows)
        {
            Toml.Section section = desktop.Add("[[window]]");
            StringBuilder name = new(256);
            Native.GetClassName(hwnd, name, name.Capacity);
            section.Str("class", name.ToString());
            StringBuilder title = new(256);
            Native.GetWindowText(hwnd, title, title.Capacity);
            section.Str("title", title.ToString());
            Native.GetWindowThreadProcessId(hwnd, out uint pid);
            section.Str("process", Try(() => Process.GetProcessById((int)pid).ProcessName));
            section.Bool("topmost", (Native.GetWindowLongPtr(hwnd, Native.GWL_EXSTYLE).ToInt64() & Native.WS_EX_TOPMOST) != 0);
            if (Native.GetWindowRect(hwnd, out Native.RECT rect))
            {
                section.Rect("window_rect", rect);
            }
        }
    }

    private static bool MovePointer(int x, int y)
    {
        RectInt32 screen = DisplayArea.Primary.OuterBounds;
        Native.INPUT[] input =
        {
            new()
            {
                type = Native.INPUT_MOUSE,
                mi = new Native.MOUSEINPUT
                {
                    dx = (int)Math.Round((x - screen.X) * 65535.0 / Math.Max(1, screen.Width - 1)),
                    dy = (int)Math.Round((y - screen.Y) * 65535.0 / Math.Max(1, screen.Height - 1)),
                    dwFlags = Native.MOUSEEVENTF_MOVE | Native.MOUSEEVENTF_ABSOLUTE,
                },
            },
        };
        return Native.SendInput((uint)input.Length, input, Marshal.SizeOf<Native.INPUT>()) == input.Length;
    }

    private static FrameworkElement? FindPresenter(DependencyObject element)
    {
        if (element is FrameworkElement framework && (framework.GetType().Name.EndsWith("Presenter", StringComparison.Ordinal) || framework is ToolTip))
        {
            return framework;
        }
        int count = VisualTreeHelper.GetChildrenCount(element);
        for (int i = 0; i < count; i++)
        {
            FrameworkElement? found = FindPresenter(VisualTreeHelper.GetChild(element, i));
            if (found is not null)
            {
                return found;
            }
        }
        return null;
    }

    private void RecordBounds(Toml.Section section, string key, FrameworkElement element, UIElement? relativeTo)
    {
        try
        {
            section.Rect(key, element.TransformToVisual(relativeTo).TransformBounds(new Rect(0, 0, element.ActualWidth, element.ActualHeight)));
        }
        catch (Exception e)
        {
            section.Str(key + "_error", e.Message);
        }
    }

    private void RecordOrigin(Toml.Section section, string key, UIElement element, UIElement? relativeTo)
    {
        try
        {
            Point point = element.TransformToVisual(relativeTo).TransformPoint(new Point(0, 0));
            section.Point(key, point.X, point.Y);
        }
        catch (Exception e)
        {
            section.Str(key + "_error", e.Message);
        }
    }

    private async Task<string> RenderPng(UIElement element, string name, Toml.Section section)
    {
        try
        {
            RenderTargetBitmap bitmap = new();
            await bitmap.RenderAsync(element);
            IBuffer pixels = await bitmap.GetPixelsAsync();
            using InMemoryRandomAccessStream stream = new();
            BitmapEncoder encoder = await BitmapEncoder.CreateAsync(BitmapEncoder.PngEncoderId, stream);
            encoder.SetPixelData(BitmapPixelFormat.Bgra8, BitmapAlphaMode.Premultiplied,
                (uint)bitmap.PixelWidth, (uint)bitmap.PixelHeight, 96, 96, pixels.ToArray());
            await encoder.FlushAsync();
            stream.Seek(0);
            using Stream source = stream.AsStreamForRead();
            using FileStream file = File.Create(Path.Combine(outDir, name));
            await source.CopyToAsync(file);
            section.Size(Path.GetFileNameWithoutExtension(name).Replace('-', '_') + "_pixels", bitmap.PixelWidth, bitmap.PixelHeight);
            return name;
        }
        catch (Exception e)
        {
            section.Str("png_error", e.Message);
            return "";
        }
    }

    private void CaptureScreen(string name)
    {
        try
        {
            RectInt32 bounds = DisplayArea.Primary.OuterBounds;
            using System.Drawing.Bitmap bitmap = new(bounds.Width, bounds.Height);
            using (System.Drawing.Graphics graphics = System.Drawing.Graphics.FromImage(bitmap))
            {
                graphics.CopyFromScreen(bounds.X, bounds.Y, 0, 0, new System.Drawing.Size(bounds.Width, bounds.Height));
            }
            bitmap.Save(Path.Combine(outDir, name), System.Drawing.Imaging.ImageFormat.Png);
        }
        catch (Exception e)
        {
            errors.Add($"{name}: {e.Message}");
        }
    }

    private void Finish(int code)
    {
        if (finished)
        {
            return;
        }
        finished = true;
        watchdog?.Stop();
        try
        {
            result?.Str("status", code == 0 ? "ok" : "failed");
            result?.StrArray("errors", errors);
            File.WriteAllText(Path.Combine(outDir, "frames.toml"), toml.Render());
            File.WriteAllText(Path.Combine(outDir, "desktop.toml"), desktop.Render());
        }
        catch (Exception e)
        {
            File.WriteAllText(Path.Combine(outDir, "error.txt"), e.ToString());
            code = 4;
        }
        Environment.Exit(code);
    }

    private static async Task<bool> WithTimeout(Task task, int milliseconds) =>
        await Task.WhenAny(task, Task.Delay(milliseconds)) == task;

    private static string Try(Func<string> read)
    {
        try
        {
            return read();
        }
        catch (Exception e)
        {
            return "error: " + e.Message;
        }
    }

    private static string FileVersion(string dll)
    {
        string path = Path.Combine(AppContext.BaseDirectory, dll);
        return File.Exists(path) ? FileVersionInfo.GetVersionInfo(path).FileVersion ?? "" : "absent";
    }

    private static string LoadedFrom(string dll)
    {
        foreach (ProcessModule module in Process.GetCurrentProcess().Modules)
        {
            if (string.Equals(module.ModuleName, dll, StringComparison.OrdinalIgnoreCase))
            {
                return module.FileName;
            }
        }
        return "not loaded";
    }
}

// A TOML document built in sections, so a section's keys can be added after
// the sections that follow it were created (a pass's PNG after its popups).
internal sealed class Toml
{
    private readonly List<Section> sections = new();

    public Section Add(string header)
    {
        Section section = new(header);
        sections.Add(section);
        return section;
    }

    public string Render()
    {
        StringBuilder text = new();
        foreach (Section section in sections)
        {
            text.Append(section.Header).Append('\n');
            foreach (string line in section.Lines)
            {
                text.Append(line).Append('\n');
            }
            text.Append('\n');
        }
        return text.ToString();
    }

    internal sealed class Section
    {
        public Section(string header) => Header = header;

        public string Header { get; }

        public List<string> Lines { get; } = new();

        public void Str(string key, string value) => Lines.Add($"{key} = {Quote(value)}");

        public void StrArray(string key, IEnumerable<string> values) =>
            Lines.Add($"{key} = [{string.Join(", ", values.Select(Quote))}]");

        public void Bool(string key, bool value) => Lines.Add($"{key} = {(value ? "true" : "false")}");

        public void Int(string key, uint value) => Lines.Add($"{key} = {value.ToString(CultureInfo.InvariantCulture)}");

        public void Num(string key, double value) => Lines.Add($"{key} = {Number(value)}");

        public void Point(string key, double x, double y) =>
            Lines.Add($"{key} = {{ x = {Number(x)}, y = {Number(y)} }}");

        public void Size(string key, double width, double height) =>
            Lines.Add($"{key} = {{ width = {Number(width)}, height = {Number(height)} }}");

        public void Rect(string key, Rect rect) =>
            Lines.Add($"{key} = {{ x = {Number(rect.X)}, y = {Number(rect.Y)}, width = {Number(rect.Width)}, height = {Number(rect.Height)} }}");

        public void Rect(string key, RectInt32 rect) =>
            Lines.Add($"{key} = {{ x = {rect.X}, y = {rect.Y}, width = {rect.Width}, height = {rect.Height} }}");

        public void Rect(string key, Native.RECT rect) =>
            Lines.Add($"{key} = {{ x = {rect.Left}, y = {rect.Top}, width = {rect.Right - rect.Left}, height = {rect.Bottom - rect.Top} }}");

        private static string Number(double value)
        {
            if (double.IsNaN(value))
            {
                return "nan";
            }
            if (double.IsInfinity(value))
            {
                return value > 0 ? "inf" : "-inf";
            }
            string text = value.ToString("R", CultureInfo.InvariantCulture);
            return text.Contains('.') || text.Contains('E') ? text : text + ".0";
        }

        private static string Quote(string value)
        {
            StringBuilder text = new("\"");
            foreach (char c in value)
            {
                switch (c)
                {
                    case '"': text.Append("\\\""); break;
                    case '\\': text.Append("\\\\"); break;
                    case '\n': text.Append("\\n"); break;
                    case '\r': text.Append("\\r"); break;
                    case '\t': text.Append("\\t"); break;
                    default:
                        if (char.IsControl(c))
                        {
                            text.Append("\\u").Append(((int)c).ToString("X4", CultureInfo.InvariantCulture));
                        }
                        else
                        {
                            text.Append(c);
                        }
                        break;
                }
            }
            return text.Append('"').ToString();
        }
    }
}

internal static class Native
{
    public const int DWMWA_EXTENDED_FRAME_BOUNDS = 9;
    public const int GWL_EXSTYLE = -20;
    public const long WS_EX_TOPMOST = 0x8;
    public const uint INPUT_MOUSE = 0;
    public const uint MOUSEEVENTF_MOVE = 0x1;
    public const uint MOUSEEVENTF_ABSOLUTE = 0x8000;

    [StructLayout(LayoutKind.Sequential)]
    public struct RECT
    {
        public int Left;
        public int Top;
        public int Right;
        public int Bottom;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct POINT
    {
        public int X;
        public int Y;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct MOUSEINPUT
    {
        public int dx;
        public int dy;
        public uint mouseData;
        public uint dwFlags;
        public uint time;
        public IntPtr dwExtraInfo;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct INPUT
    {
        public uint type;
        public MOUSEINPUT mi;
    }

    public delegate bool EnumWindowsProc(IntPtr hwnd, IntPtr lParam);

    [DllImport("user32.dll")]
    public static extern bool EnumWindows(EnumWindowsProc callback, IntPtr lParam);

    [DllImport("user32.dll")]
    public static extern uint GetWindowThreadProcessId(IntPtr hwnd, out uint processId);

    [DllImport("user32.dll")]
    public static extern bool IsWindowVisible(IntPtr hwnd);

    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int GetClassName(IntPtr hwnd, StringBuilder name, int capacity);

    [DllImport("user32.dll")]
    public static extern bool GetWindowRect(IntPtr hwnd, out RECT rect);

    [DllImport("user32.dll")]
    public static extern bool ClientToScreen(IntPtr hwnd, ref POINT point);

    [DllImport("user32.dll")]
    public static extern uint SendInput(uint count, INPUT[] inputs, int size);

    [DllImport("user32.dll", CharSet = CharSet.Unicode)]
    public static extern int GetWindowText(IntPtr hwnd, StringBuilder text, int capacity);

    [DllImport("user32.dll", EntryPoint = "GetWindowLongPtrW")]
    public static extern IntPtr GetWindowLongPtr(IntPtr hwnd, int index);

    [DllImport("user32.dll")]
    public static extern uint GetDpiForWindow(IntPtr hwnd);

    [DllImport("dwmapi.dll")]
    public static extern int DwmGetWindowAttribute(IntPtr hwnd, int attribute, out RECT value, int size);
}
