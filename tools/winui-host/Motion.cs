// The motion and capture passes of the 0.8.0 measurement plan (§2) the WinUI
// host runs after Reads.cs's: each records a small screen region frame by
// frame on a worker thread (GDI, as fast as it returns, every frame timed from
// the pass's start) while the UI thread acts, and writes motion-<name>.png (the
// frames side by side, in time order), motion-<name>.csv (each frame's time,
// its centre pixel and its mean colour) and a [[motion]] table (the region,
// the actions and their times, what was read).
//   WH-motion    scroll_into_view (U50): focus moved to a field below a
//                ScrollViewer's view, its ViewChanging offsets over time;
//                brush transitions between colours of different alpha (U157);
//                a theme switch, idle and during a running brush transition
//                (U183); Mica as the window is deactivated (U184); an element
//                collapsed and shown again, and client-area animations turned
//                off, during a transition (U185); a list item reselected
//                during its pill's fade (U188); a new unchecked CheckBox's
//                first frames (U193); a checked RadioButton held pressed
//                (U162); Mouse Vanish turned on and a key typed (U502)
//   WH-captures  the caret of a focused TextBox, left-to-right and at a
//                direction boundary (U507); a MenuFlyout's shadow over white
//                and over black (U511); an item dragged over a drop target
//                (U513)
//   all-hosts    the window minimised during a brush transition (U187); the
//                focus flags of a keyboard-focused button while the window is
//                inactive (U190)
using System.Diagnostics;
using System.Globalization;
using System.Runtime.InteropServices;
using System.Text;
using Microsoft.UI;
using Microsoft.UI.Windowing;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Input;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation;
using Windows.Graphics;
using Color = Windows.UI.Color;

namespace WinUiHost;

public partial class App
{
    private sealed record Frame(double Milliseconds, System.Drawing.Bitmap Image);

    private sealed class Pass
    {
        public Pass(Toml.Section section) => Section = section;

        public Toml.Section Section { get; }

        public Stopwatch Clock { get; } = new();

        public List<string> Actions { get; } = new();

        public void Act(string action) =>
            Actions.Add($"{Clock.Elapsed.TotalMilliseconds.ToString("F1", CultureInfo.InvariantCulture)} ms {action}");
    }

    private async Task MotionPasses()
    {
        await Guarded("scroll_into_view", ScrollIntoViewPass);
        await Guarded("color_space", ColorSpacePass);
        await Guarded("theme_change", ThemeChangePass);
        await Guarded("presence", PresencePass);
        await Guarded("list_reselect", ListReselectPass);
        await Guarded("checkbox_first_frame", CheckBoxFirstFramePass);
        await Guarded("radio_pressed", RadioPressedPass);
        await Guarded("caret", CaretPass);
        await Guarded("flyout_shadow", FlyoutShadowPass);
        await Guarded("drop", DropPass);
        await Guarded("mouse_vanish", MouseVanishPass);
        await Guarded("minimise", MinimisePass);
        await Guarded("deactivate", DeactivatePass);
    }

    // A pass's failure is recorded, and the passes after it still run.
    private async Task Guarded(string name, Func<Task> pass)
    {
        try
        {
            await pass();
        }
        catch (Exception e)
        {
            Toml.Section section = toml.Add("[[motion_error]]");
            section.Str("pass", name);
            section.Str("error", e.ToString());
            errors.Add($"{name}: {e.Message}");
        }
    }

    private Pass NewPass(string name)
    {
        Toml.Section section = toml.Add("[[motion]]");
        section.Str("name", name);
        return new Pass(section);
    }

    // The element's rectangle on the screen, in physical pixels, grown by
    // `margin` effective pixels on each side.
    private RectInt32 ScreenRect(FrameworkElement element, double margin = 0)
    {
        IntPtr hwnd = WinRT.Interop.WindowNative.GetWindowHandle(window);
        Native.POINT origin = new();
        Native.ClientToScreen(hwnd, ref origin);
        Rect bounds = element.TransformToVisual(root).TransformBounds(new Rect(0, 0, element.ActualWidth, element.ActualHeight));
        double scale = root!.XamlRoot.RasterizationScale;
        int x = origin.X + (int)Math.Round((bounds.X - margin) * scale);
        int y = origin.Y + (int)Math.Round((bounds.Y - margin) * scale);
        int width = Math.Max(1, (int)Math.Round((bounds.Width + 2 * margin) * scale));
        int height = Math.Max(1, (int)Math.Round((bounds.Height + 2 * margin) * scale));
        return new RectInt32(x, y, width, height);
    }

    private static (int X, int Y) Centre(RectInt32 rect) => (rect.X + rect.Width / 2, rect.Y + rect.Height / 2);

    // Records `rect` until `milliseconds` after the pass's clock started.
    private static Task<List<Frame>> Record(Pass pass, RectInt32 rect, double milliseconds) => Task.Run(() =>
    {
        List<Frame> frames = new();
        while (pass.Clock.Elapsed.TotalMilliseconds < milliseconds && frames.Count < 600)
        {
            double at = pass.Clock.Elapsed.TotalMilliseconds;
            System.Drawing.Bitmap bitmap = new(rect.Width, rect.Height);
            using (System.Drawing.Graphics graphics = System.Drawing.Graphics.FromImage(bitmap))
            {
                graphics.CopyFromScreen(rect.X, rect.Y, 0, 0, new System.Drawing.Size(rect.Width, rect.Height));
            }
            frames.Add(new Frame(at, bitmap));
            Thread.Sleep(4);
        }
        return frames;
    });

    // Writes the frames that differ from the frame before them (the first
    // always), side by side, and every frame's row in the CSV.
    private void SaveFrames(Pass pass, string name, RectInt32 rect, List<Frame> frames)
    {
        Toml.Section section = pass.Section;
        section.Rect("screen_rect", rect);
        section.Int("frames", (uint)frames.Count);
        StringBuilder csv = new("ms,distinct_index,centre,mean\n");
        List<Frame> distinct = new();
        string? previous = null;
        foreach (Frame frame in frames)
        {
            string hash = Hash(frame.Image);
            if (hash != previous)
            {
                distinct.Add(frame);
                previous = hash;
            }
            System.Drawing.Color centre = frame.Image.GetPixel(frame.Image.Width / 2, frame.Image.Height / 2);
            csv.Append(frame.Milliseconds.ToString("F1", CultureInfo.InvariantCulture)).Append(',')
                .Append(distinct.Count - 1).Append(',')
                .Append(Hex(centre)).Append(',')
                .Append(Mean(frame.Image)).Append('\n');
        }
        File.WriteAllText(Path.Combine(outDir, $"motion-{name}.csv"), csv.ToString());
        section.Int("distinct_frames", (uint)distinct.Count);
        section.Str("csv", $"motion-{name}.csv");
        if (distinct.Count > 0)
        {
            int width = rect.Width * distinct.Count;
            using System.Drawing.Bitmap strip = new(width, rect.Height);
            using (System.Drawing.Graphics graphics = System.Drawing.Graphics.FromImage(strip))
            {
                for (int i = 0; i < distinct.Count; i++)
                {
                    graphics.DrawImageUnscaled(distinct[i].Image, i * rect.Width, 0);
                }
            }
            strip.Save(Path.Combine(outDir, $"motion-{name}.png"), System.Drawing.Imaging.ImageFormat.Png);
            section.Str("png", $"motion-{name}.png");
            section.StrArray("png_frame_ms", distinct.Select(f => f.Milliseconds.ToString("F1", CultureInfo.InvariantCulture)));
        }
        section.StrArray("actions", pass.Actions);
        foreach (Frame frame in frames)
        {
            frame.Image.Dispose();
        }
    }

    private static string Hash(System.Drawing.Bitmap bitmap)
    {
        StringBuilder text = new();
        for (int y = 0; y < bitmap.Height; y++)
        {
            for (int x = 0; x < bitmap.Width; x++)
            {
                text.Append(bitmap.GetPixel(x, y).ToArgb().ToString("x8", CultureInfo.InvariantCulture));
            }
        }
        return Convert.ToHexString(System.Security.Cryptography.SHA256.HashData(Encoding.ASCII.GetBytes(text.ToString())));
    }

    private static string Hex(System.Drawing.Color c) => $"#{c.R:x2}{c.G:x2}{c.B:x2}";

    private static string Mean(System.Drawing.Bitmap bitmap)
    {
        long r = 0, g = 0, b = 0;
        int count = bitmap.Width * bitmap.Height;
        for (int y = 0; y < bitmap.Height; y++)
        {
            for (int x = 0; x < bitmap.Width; x++)
            {
                System.Drawing.Color c = bitmap.GetPixel(x, y);
                r += c.R;
                g += c.G;
                b += c.B;
            }
        }
        return count == 0 ? "" : $"#{r / count:x2}{g / count:x2}{b / count:x2}";
    }

    private static Border TransitionBorder(Color colour) => new()
    {
        Width = 60,
        Height = 40,
        Background = new SolidColorBrush(colour),
        BackgroundTransition = new BrushTransition { Duration = TimeSpan.FromMilliseconds(1000) },
        HorizontalAlignment = HorizontalAlignment.Left,
        VerticalAlignment = VerticalAlignment.Top,
    };

    // A white panel at the window's top left holding `child`, so what lies
    // beneath a translucent colour is known.
    private Grid WhiteHost(UIElement child)
    {
        Grid host = new()
        {
            Background = new SolidColorBrush(Colors.White),
            HorizontalAlignment = HorizontalAlignment.Left,
            VerticalAlignment = VerticalAlignment.Top,
            Padding = new Thickness(10),
        };
        host.Children.Add(child);
        root!.Children.Add(host);
        return host;
    }

    // ── WH-motion ───────────────────────────────────────────────────────────

    private async Task ScrollIntoViewPass()
    {
        StackPanel form = new() { Spacing = 8 };
        List<TextBox> fields = new();
        for (int i = 0; i < 20; i++)
        {
            TextBox field = new() { Header = $"Field {i}", Width = 200 };
            fields.Add(field);
            form.Children.Add(field);
        }
        ScrollViewer viewer = new()
        {
            Content = form,
            Height = 240,
            Width = 260,
            HorizontalAlignment = HorizontalAlignment.Left,
            VerticalAlignment = VerticalAlignment.Top,
        };
        root!.Children.Add(viewer);
        await Task.Delay(SettleMilliseconds);
        foreach ((string name, int target, Action<Pass> act) in new (string, int, Action<Pass>)[]
        {
            ("scroll_into_view_focus", 12, pass => { pass.Act("Focus(Keyboard) on field 12"); fields[12].Focus(FocusState.Keyboard); }),
            ("scroll_into_view_focus_next", 13, pass => { pass.Act("Focus(Keyboard) on field 13"); fields[13].Focus(FocusState.Keyboard); }),
            ("scroll_into_view_bring_into_view", 2, pass =>
            {
                pass.Act("StartBringIntoView(AnimationDesired) on field 2");
                fields[2].StartBringIntoView(new BringIntoViewOptions { AnimationDesired = true });
            }),
            ("scroll_into_view_change_view", 0, pass =>
            {
                pass.Act("ChangeView(vertical 600, animation)");
                viewer.ChangeView(null, 600, null, false);
            }),
        })
        {
            Pass pass = NewPass(name);
            List<string> changes = new();
            EventHandler<ScrollViewerViewChangingEventArgs> changing = (_, e) =>
                changes.Add($"{pass.Clock.Elapsed.TotalMilliseconds.ToString("F1", CultureInfo.InvariantCulture)} ms changing next {e.NextView.VerticalOffset.ToString("R", CultureInfo.InvariantCulture)} final {e.FinalView.VerticalOffset.ToString("R", CultureInfo.InvariantCulture)} inertial {e.IsInertial}");
            EventHandler<ScrollViewerViewChangedEventArgs> changed = (_, e) =>
                changes.Add($"{pass.Clock.Elapsed.TotalMilliseconds.ToString("F1", CultureInfo.InvariantCulture)} ms changed offset {viewer.VerticalOffset.ToString("R", CultureInfo.InvariantCulture)} intermediate {e.IsIntermediate}");
            List<string> rendered = new();
            EventHandler<object> rendering = (_, _) =>
                rendered.Add($"{pass.Clock.Elapsed.TotalMilliseconds.ToString("F1", CultureInfo.InvariantCulture)} {viewer.VerticalOffset.ToString("R", CultureInfo.InvariantCulture)}");
            viewer.ViewChanging += changing;
            viewer.ViewChanged += changed;
            CompositionTarget.Rendering += rendering;
            pass.Section.Num("offset_before", viewer.VerticalOffset);
            pass.Clock.Start();
            act(pass);
            await Task.Delay(1500);
            CompositionTarget.Rendering -= rendering;
            viewer.ViewChanging -= changing;
            viewer.ViewChanged -= changed;
            pass.Section.Num("offset_after", viewer.VerticalOffset);
            pass.Section.Int("target_field", (uint)target);
            pass.Section.Str("focused_element", FocusManager.GetFocusedElement(root.XamlRoot)?.GetType().FullName ?? "none");
            pass.Section.StrArray("view_events", changes);
            pass.Section.StrArray("rendering_offsets", rendered);
            pass.Section.StrArray("actions", pass.Actions);
            await Task.Delay(500);
        }
        root.Children.Remove(viewer);
        await Task.Delay(SettleMilliseconds);
    }

    // Brush transitions (1 s) between colours of different alpha over white:
    // the middle frames tell the interpolation's colour space and whether it
    // runs on premultiplied colours.
    private async Task ColorSpacePass()
    {
        foreach ((string name, Color from, Color to) in new (string, Color, Color)[]
        {
            ("color_space_red_to_translucent_blue", ColorHelper.FromArgb(255, 255, 0, 0), ColorHelper.FromArgb(64, 0, 0, 255)),
            ("color_space_translucent_green_to_red", ColorHelper.FromArgb(128, 0, 255, 0), ColorHelper.FromArgb(255, 255, 0, 0)),
            ("color_space_black_to_white", ColorHelper.FromArgb(255, 0, 0, 0), ColorHelper.FromArgb(255, 255, 255, 255)),
        })
        {
            Border border = TransitionBorder(from);
            Grid host = WhiteHost(border);
            await Task.Delay(SettleMilliseconds);
            Pass pass = NewPass(name);
            pass.Section.Str("from", $"#{from.A:x2}{from.R:x2}{from.G:x2}{from.B:x2}");
            pass.Section.Str("to", $"#{to.A:x2}{to.R:x2}{to.G:x2}{to.B:x2}");
            pass.Section.Str("transition", "Border.BackgroundTransition, BrushTransition 1000 ms, over white");
            RectInt32 rect = ScreenRect(border);
            pass.Clock.Start();
            Task<List<Frame>> recording = Record(pass, rect, 1600);
            await Task.Delay(200);
            pass.Act("Background set");
            border.Background = new SolidColorBrush(to);
            SaveFrames(pass, name, rect, await recording);
            root!.Children.Remove(host);
            await Task.Delay(500);
        }
    }

    // A theme-dependent brush (TextFillColorPrimaryBrush through
    // {ThemeResource}) with a brush transition: the theme switched while
    // idle, and switched again while an explicit transition runs.
    private async Task ThemeChangePass()
    {
        Border border = (Border)Microsoft.UI.Xaml.Markup.XamlReader.Load(
            "<Border xmlns=\"http://schemas.microsoft.com/winfx/2006/xaml/presentation\" Width=\"60\" Height=\"40\" " +
            "HorizontalAlignment=\"Left\" VerticalAlignment=\"Top\" Background=\"{ThemeResource TextFillColorPrimaryBrush}\">" +
            "<Border.BackgroundTransition><BrushTransition Duration=\"0:0:1\" /></Border.BackgroundTransition></Border>");
        Grid host = WhiteHost(border);
        await Task.Delay(SettleMilliseconds);
        RectInt32 rect = ScreenRect(border);

        Pass idle = NewPass("theme_change_idle");
        idle.Clock.Start();
        Task<List<Frame>> recording = Record(idle, rect, 1600);
        await Task.Delay(200);
        idle.Act("host.RequestedTheme = Dark");
        host.RequestedTheme = ElementTheme.Dark;
        SaveFrames(idle, "theme_change_idle", rect, await recording);
        await Task.Delay(500);

        Pass running = NewPass("theme_change_running");
        running.Clock.Start();
        recording = Record(running, rect, 2000);
        await Task.Delay(200);
        running.Act("Background = #ff0000ff");
        border.Background = new SolidColorBrush(ColorHelper.FromArgb(255, 0, 0, 255));
        await Task.Delay(400);
        running.Act("host.RequestedTheme = Light");
        host.RequestedTheme = ElementTheme.Light;
        SaveFrames(running, "theme_change_running", rect, await recording);
        root!.Children.Remove(host);
        await Task.Delay(500);
    }

    // An element collapsed and shown again during a brush transition; then
    // client-area animations turned off (SPI_SETCLIENTAREAANIMATION) during
    // one, and restored.
    private async Task PresencePass()
    {
        Border border = TransitionBorder(ColorHelper.FromArgb(255, 255, 0, 0));
        Grid host = WhiteHost(border);
        await Task.Delay(SettleMilliseconds);
        RectInt32 rect = ScreenRect(border);

        Pass collapse = NewPass("presence_collapse");
        collapse.Clock.Start();
        Task<List<Frame>> recording = Record(collapse, rect, 1800);
        await Task.Delay(200);
        collapse.Act("Background = #ff0000ff");
        border.Background = new SolidColorBrush(ColorHelper.FromArgb(255, 0, 0, 255));
        await Task.Delay(300);
        collapse.Act("Visibility = Collapsed");
        border.Visibility = Visibility.Collapsed;
        await Task.Delay(200);
        collapse.Act("Visibility = Visible");
        border.Visibility = Visibility.Visible;
        SaveFrames(collapse, "presence_collapse", rect, await recording);
        await Task.Delay(500);

        Pass animations = NewPass("presence_animations_off");
        bool before = NativeMotion.GetClientAreaAnimation(out bool enabled);
        animations.Section.Bool("client_area_animation_read", before);
        animations.Section.Bool("client_area_animation_before", enabled);
        bool uiBefore = new Windows.UI.ViewManagement.UISettings().AnimationsEnabled;
        animations.Section.Bool("ui_settings_animations_enabled_before", uiBefore);
        animations.Clock.Start();
        recording = Record(animations, rect, 1800);
        await Task.Delay(200);
        animations.Act("Background = #ffff0000");
        border.Background = new SolidColorBrush(ColorHelper.FromArgb(255, 255, 0, 0));
        await Task.Delay(300);
        animations.Act("SPI_SETCLIENTAREAANIMATION FALSE");
        animations.Section.Bool("set_off", NativeMotion.SetClientAreaAnimation(false));
        await Task.Delay(100);
        animations.Section.Bool("ui_settings_animations_enabled_after_off", new Windows.UI.ViewManagement.UISettings().AnimationsEnabled);
        animations.Act("Background = #ff00ff00");
        border.Background = new SolidColorBrush(ColorHelper.FromArgb(255, 0, 255, 0));
        SaveFrames(animations, "presence_animations_off", rect, await recording);
        animations.Section.Bool("restored", NativeMotion.SetClientAreaAnimation(enabled || !before));
        root!.Children.Remove(host);
        await Task.Delay(500);
    }

    // A ListView of three items: item 0 selected, then item 1, then item 0
    // again 120 ms later (during the pill's fade), the items' left edge
    // recorded.
    private async Task ListReselectPass()
    {
        ListView list = new()
        {
            Width = 200,
            SelectionMode = ListViewSelectionMode.Single,
            HorizontalAlignment = HorizontalAlignment.Left,
            VerticalAlignment = VerticalAlignment.Top,
        };
        foreach (string item in new[] { "Item 0", "Item 1", "Item 2" })
        {
            list.Items.Add(item);
        }
        root!.Children.Add(list);
        await Task.Delay(SettleMilliseconds);
        list.SelectedIndex = 0;
        await Task.Delay(SettleMilliseconds);
        RectInt32 full = ScreenRect(list);
        RectInt32 rect = new(full.X, full.Y, Math.Min(40, full.Width), full.Height);
        Pass pass = NewPass("list_reselect");
        pass.Clock.Start();
        Task<List<Frame>> recording = Record(pass, rect, 1500);
        await Task.Delay(200);
        pass.Act("SelectedIndex = 1");
        list.SelectedIndex = 1;
        await Task.Delay(120);
        pass.Act("SelectedIndex = 0");
        list.SelectedIndex = 0;
        SaveFrames(pass, "list_reselect", rect, await recording);
        root.Children.Remove(list);
        await Task.Delay(500);
    }

    private async Task CheckBoxFirstFramePass()
    {
        Grid host = WhiteHost(new Border { Width = 120, Height = 40 });
        await Task.Delay(SettleMilliseconds);
        RectInt32 rect = ScreenRect(host);
        Pass pass = NewPass("checkbox_first_frame");
        pass.Clock.Start();
        Task<List<Frame>> recording = Record(pass, rect, 1200);
        await Task.Delay(200);
        pass.Act("a new unchecked CheckBox added");
        host.Children.Clear();
        host.Children.Add(new CheckBox { Content = "Check", IsChecked = false });
        SaveFrames(pass, "checkbox_first_frame", rect, await recording);
        root!.Children.Remove(host);
        await Task.Delay(500);
    }

    // A checked RadioButton pressed by the pointer and held, then released;
    // the same for an unchecked one.
    private async Task RadioPressedPass()
    {
        foreach ((string name, bool isChecked) in new[] { ("radio_pressed_checked", true), ("radio_pressed_unchecked", false) })
        {
            RadioButton radio = new() { Content = "Radio", IsChecked = isChecked, GroupName = name };
            Grid host = WhiteHost(radio);
            await Task.Delay(SettleMilliseconds);
            RectInt32 rect = ScreenRect(radio);
            RectInt32 glyph = new(rect.X, rect.Y, Math.Min(32, rect.Width), rect.Height);
            (int x, int y) = (glyph.X + 10, glyph.Y + glyph.Height / 2);
            Pass pass = NewPass(name);
            pass.Section.Point("pointer", x, y);
            MovePointer(x, y);
            await Task.Delay(500);
            pass.Clock.Start();
            Task<List<Frame>> recording = Record(pass, glyph, 1600);
            await Task.Delay(200);
            pass.Act("left button down");
            NativeMotion.MouseButton(true);
            await Task.Delay(700);
            pass.Section.Bool("is_pressed_while_held", radio.IsPressed);
            pass.Act("left button up");
            NativeMotion.MouseButton(false);
            SaveFrames(pass, name, glyph, await recording);
            pass.Section.Str("is_checked_after", radio.IsChecked?.ToString() ?? "null");
            MovePointer(0, 0);
            root!.Children.Remove(host);
            await Task.Delay(500);
        }
    }

    // ── WH-captures ─────────────────────────────────────────────────────────

    // A focused TextBox's caret: at the end of left-to-right text, and in
    // mixed text at the boundary between the runs.
    private async Task CaretPass()
    {
        foreach ((string name, string text, int caret) in new[]
        {
            ("caret_ltr", "Wg", 2),
            ("caret_mixed", "abc אבג", 3),
            ("caret_mixed_end", "abc אבג", 7),
        })
        {
            TextBox box = new() { Text = text, Width = 160 };
            Grid host = WhiteHost(box);
            await Task.Delay(SettleMilliseconds);
            box.Focus(FocusState.Keyboard);
            box.Select(caret, 0);
            await Task.Delay(300);
            RectInt32 rect = ScreenRect(box);
            Pass pass = NewPass(name);
            pass.Section.Str("text", text);
            pass.Section.Int("selection_start", (uint)box.SelectionStart);
            pass.Section.Str("focus_state", box.FocusState.ToString());
            pass.Clock.Start();
            SaveFrames(pass, name, rect, await Record(pass, rect, 2400));
            root!.Children.Remove(host);
            await Task.Delay(500);
        }
    }

    // A MenuFlyout opened over a white and over a black window background,
    // its frame and the area around it captured.
    private async Task FlyoutShadowPass()
    {
        Brush? background = root!.Background;
        foreach ((string name, Color colour) in new[] { ("flyout_shadow_white", Colors.White), ("flyout_shadow_black", Colors.Black) })
        {
            root.Background = new SolidColorBrush(colour);
            MenuFlyout flyout = new();
            foreach (string item in new[] { "Cut", "Copy", "Paste" })
            {
                flyout.Items.Add(new MenuFlyoutItem { Text = item });
            }
            TaskCompletionSource opened = new();
            flyout.Opened += (_, _) => opened.TrySetResult();
            await Task.Delay(SettleMilliseconds);
            flyout.ShowAt(anchor);
            Pass pass = NewPass(name);
            pass.Section.Bool("opened_event", await WithTimeout(opened.Task, EventTimeoutMilliseconds));
            await Task.Delay(SettleMilliseconds);
            IReadOnlyList<Popup> popups = VisualTreeHelper.GetOpenPopupsForXamlRoot(root.XamlRoot);
            if (popups.Count > 0 && popups[0].Child is FrameworkElement child)
            {
                RectInt32 rect = ScreenRect(child, 48);
                pass.Section.Rect("child_screen_rect", ScreenRect(child));
                pass.Clock.Start();
                SaveFrames(pass, name, rect, await Record(pass, rect, 50));
            }
            else
            {
                pass.Section.Str("error", "no open popup");
            }
            flyout.Hide();
            await Task.Delay(SettleMilliseconds);
        }
        root.Background = background;
    }

    // A ListView item dragged by the pointer onto a ListView that accepts it
    // (AllowDrop, AcceptedOperation Move), held over it and captured.
    private async Task DropPass()
    {
        ListView source = new() { CanDragItems = true, Width = 160, SelectionMode = ListViewSelectionMode.None };
        source.Items.Add("Drag me");
        source.DragItemsStarting += (_, e) => e.Data.SetText("Drag me");
        ListView target = new() { AllowDrop = true, Width = 200, Height = 160 };
        target.Items.Add("Target");
        bool dragOver = false;
        target.DragOver += (_, e) =>
        {
            e.AcceptedOperation = Windows.ApplicationModel.DataTransfer.DataPackageOperation.Move;
            dragOver = true;
        };
        StackPanel panel = new() { Orientation = Orientation.Horizontal, Spacing = 40 };
        panel.Children.Add(source);
        panel.Children.Add(target);
        Grid host = WhiteHost(panel);
        await Task.Delay(SettleMilliseconds);
        RectInt32 from = ScreenRect(source);
        RectInt32 to = ScreenRect(target);
        (int fx, int fy) = (from.X + 30, from.Y + 20);
        (int tx, int ty) = Centre(to);
        Pass pass = NewPass("drop");
        MovePointer(fx, fy);
        await Task.Delay(300);
        pass.Clock.Start();
        pass.Act("left button down on the item");
        NativeMotion.MouseButton(true);
        await Task.Delay(200);
        for (int i = 1; i <= 20; i++)
        {
            MovePointer(fx + (tx - fx) * i / 20, fy + (ty - fy) * i / 20);
            await Task.Delay(30);
        }
        pass.Act("pointer over the target");
        await Task.Delay(800);
        pass.Section.Bool("drag_over_event", dragOver);
        RectInt32 rect = ScreenRect(target, 16);
        SaveFrames(pass, "drop", rect, await Record(pass, rect, pass.Clock.Elapsed.TotalMilliseconds + 300));
        CaptureScreen("screen-drop.png");
        NativeMotion.MouseButton(false);
        MovePointer(0, 0);
        await Task.Delay(SettleMilliseconds);
        root!.Children.Remove(host);
        await Task.Delay(500);
    }

    // Mouse Vanish (SPI_SETMOUSEVANISH) on, the pointer over a focused
    // TextBox, a key typed; the cursor's flags (GetCursorInfo) polled.
    private async Task MouseVanishPass()
    {
        TextBox box = new() { Width = 200 };
        Grid host = WhiteHost(box);
        await Task.Delay(SettleMilliseconds);
        box.Focus(FocusState.Keyboard);
        (int x, int y) = Centre(ScreenRect(box));
        MovePointer(x, y);
        Pass pass = NewPass("mouse_vanish");
        bool read = NativeMotion.GetMouseVanish(out bool before);
        pass.Section.Bool("mouse_vanish_read", read);
        pass.Section.Bool("mouse_vanish_before", before);
        pass.Section.Bool("set_on", NativeMotion.SetMouseVanish(true));
        await Task.Delay(500);
        List<string> flags = new();
        pass.Clock.Start();
        Task poll = Task.Run(() =>
        {
            string? last = null;
            while (pass.Clock.Elapsed.TotalMilliseconds < 2500)
            {
                string now = NativeMotion.CursorFlags();
                if (now != last)
                {
                    flags.Add($"{pass.Clock.Elapsed.TotalMilliseconds.ToString("F1", CultureInfo.InvariantCulture)} ms {now}");
                    last = now;
                }
                Thread.Sleep(2);
            }
        });
        await Task.Delay(300);
        pass.Act("key A typed");
        NativeMotion.TypeKey(0x41);
        await Task.Delay(1200);
        pass.Act("pointer moved");
        MovePointer(x + 5, y + 5);
        await poll;
        pass.Section.StrArray("cursor_flags", flags);
        pass.Section.Str("text_after", box.Text);
        pass.Section.StrArray("actions", pass.Actions);
        pass.Section.Bool("restored", NativeMotion.SetMouseVanish(before));
        MovePointer(0, 0);
        root!.Children.Remove(host);
        await Task.Delay(500);
    }

    // ── all-hosts ───────────────────────────────────────────────────────────

    // The window minimised during a brush transition and restored: the
    // rendering callbacks while minimised, the element's state, and the
    // frames after the restore.
    private async Task MinimisePass()
    {
        Border border = TransitionBorder(ColorHelper.FromArgb(255, 255, 0, 0));
        Grid host = WhiteHost(border);
        await Task.Delay(SettleMilliseconds);
        RectInt32 rect = ScreenRect(border);
        Pass pass = NewPass("minimise");
        int renders = 0;
        int sizeChanges = 0;
        EventHandler<object> rendering = (_, _) => renders++;
        SizeChangedEventHandler sized = (_, _) => sizeChanges++;
        border.SizeChanged += sized;
        pass.Clock.Start();
        border.Background = new SolidColorBrush(ColorHelper.FromArgb(255, 0, 0, 255));
        pass.Act("Background = #ff0000ff");
        await Task.Delay(300);
        CompositionTarget.Rendering += rendering;
        if (window!.AppWindow.Presenter is OverlappedPresenter presenter)
        {
            pass.Act("Minimize");
            presenter.Minimize();
            await Task.Delay(1000);
            pass.Section.Str("presenter_state_minimised", presenter.State.ToString());
            pass.Section.Bool("window_visible_minimised", window.Visible);
            pass.Section.Int("rendering_callbacks_minimised", (uint)renders);
            pass.Section.Size("border_actual_size_minimised", border.ActualWidth, border.ActualHeight);
            CompositionTarget.Rendering -= rendering;
            pass.Act("Restore");
            presenter.Restore();
            window.Activate();
            await Task.Delay(100);
            SaveFrames(pass, "minimise", rect, await Record(pass, rect, pass.Clock.Elapsed.TotalMilliseconds + 1200));
            pass.Section.Str("presenter_state_restored", presenter.State.ToString());
        }
        else
        {
            CompositionTarget.Rendering -= rendering;
            pass.Section.Str("error", "the presenter is not an OverlappedPresenter");
        }
        pass.Section.Int("size_changed_events", (uint)sizeChanges);
        border.SizeChanged -= sized;
        root!.Children.Remove(host);
        await Task.Delay(SettleMilliseconds);
    }

    // A button focused by the keyboard, the window deactivated (a second
    // window activated) and activated again: the focus state, the focused
    // element and the button's pixels each time; and the Mica backdrop's
    // pixels over the deactivation.
    private async Task DeactivatePass()
    {
        Pass pass = NewPass("deactivate");
        anchor!.Focus(FocusState.Keyboard);
        await Task.Delay(SettleMilliseconds);
        RectInt32 rect = ScreenRect(anchor, 8);
        pass.Section.Str("focus_state_active", anchor.FocusState.ToString());
        pass.Section.Str("focused_active", FocusManager.GetFocusedElement(root!.XamlRoot)?.GetType().FullName ?? "none");
        Brush? background = root.Background;
        bool micaSupported = Microsoft.UI.Composition.SystemBackdrops.MicaController.IsSupported();
        pass.Section.Bool("mica_controller_is_supported", micaSupported);
        root.Background = null;
        window!.SystemBackdrop = new MicaBackdrop();
        await Task.Delay(SettleMilliseconds);
        RectInt32 backdrop = new(rect.X - 120, rect.Y, 40, 40);
        pass.Section.Rect("backdrop_rect", backdrop);
        pass.Clock.Start();
        Task<List<Frame>> backdropFrames = Record(pass, backdrop, 2600);
        Task<List<Frame>> buttonFrames = Record(pass, rect, 2600);
        await Task.Delay(300);
        Window other = new() { Title = "native-theme WinUI host (other)" };
        other.AppWindow.MoveAndResize(new RectInt32(900, 600, 120, 120));
        pass.Act("another window activated");
        other.Activate();
        await Task.Delay(1200);
        pass.Section.Str("focus_state_inactive", anchor.FocusState.ToString());
        pass.Section.Str("focused_inactive", FocusManager.GetFocusedElement(root.XamlRoot)?.GetType().FullName ?? "none");
        pass.Act("the host's window activated");
        window.Activate();
        window.AppWindow.MoveInZOrderAtTop();
        List<Frame> button = await buttonFrames;
        List<Frame> mica = await backdropFrames;
        pass.Section.Str("focus_state_reactivated", anchor.FocusState.ToString());
        SaveFrames(pass, "deactivate_button", rect, button);
        Pass micaPass = NewPass("deactivate_mica");
        micaPass.Section.Bool("mica_controller_is_supported", micaSupported);
        foreach (string action in pass.Actions)
        {
            micaPass.Actions.Add(action);
        }
        SaveFrames(micaPass, "deactivate_mica", backdrop, mica);
        other.Close();
        window.SystemBackdrop = null;
        root.Background = background;
        await Task.Delay(SettleMilliseconds);
    }
}

internal static class NativeMotion
{
    private const uint SPI_GETCLIENTAREAANIMATION = 0x1042;
    private const uint SPI_SETCLIENTAREAANIMATION = 0x1043;
    private const uint SPI_GETMOUSEVANISH = 0x1020;
    private const uint SPI_SETMOUSEVANISH = 0x1021;
    private const uint SPIF_SENDCHANGE = 0x2;
    private const uint MOUSEEVENTF_LEFTDOWN = 0x2;
    private const uint MOUSEEVENTF_LEFTUP = 0x4;
    private const uint KEYEVENTF_KEYUP = 0x2;
    private const uint CURSOR_SHOWING = 0x1;
    private const uint CURSOR_SUPPRESSED = 0x2;

    [StructLayout(LayoutKind.Sequential)]
    private struct CURSORINFO
    {
        public uint cbSize;
        public uint flags;
        public IntPtr hCursor;
        public Native.POINT ptScreenPos;
    }

    [DllImport("user32.dll", EntryPoint = "SystemParametersInfoW")]
    private static extern bool SystemParametersInfoGet(uint action, uint param, out int value, uint winIni);

    [DllImport("user32.dll", EntryPoint = "SystemParametersInfoW")]
    private static extern bool SystemParametersInfoSet(uint action, uint param, IntPtr value, uint winIni);

    [DllImport("user32.dll")]
    private static extern void mouse_event(uint flags, int dx, int dy, uint data, UIntPtr extra);

    [DllImport("user32.dll")]
    private static extern void keybd_event(byte vk, byte scan, uint flags, UIntPtr extra);

    [DllImport("user32.dll")]
    private static extern bool GetCursorInfo(ref CURSORINFO info);

    public static bool GetClientAreaAnimation(out bool enabled)
    {
        bool ok = SystemParametersInfoGet(SPI_GETCLIENTAREAANIMATION, 0, out int value, 0);
        enabled = value != 0;
        return ok;
    }

    public static bool SetClientAreaAnimation(bool enabled) =>
        SystemParametersInfoSet(SPI_SETCLIENTAREAANIMATION, 0, enabled ? (IntPtr)1 : IntPtr.Zero, SPIF_SENDCHANGE);

    public static bool GetMouseVanish(out bool enabled)
    {
        bool ok = SystemParametersInfoGet(SPI_GETMOUSEVANISH, 0, out int value, 0);
        enabled = value != 0;
        return ok;
    }

    public static bool SetMouseVanish(bool enabled) =>
        SystemParametersInfoSet(SPI_SETMOUSEVANISH, 0, enabled ? (IntPtr)1 : IntPtr.Zero, SPIF_SENDCHANGE);

    public static void MouseButton(bool down) =>
        mouse_event(down ? MOUSEEVENTF_LEFTDOWN : MOUSEEVENTF_LEFTUP, 0, 0, 0, UIntPtr.Zero);

    public static void TypeKey(byte vk)
    {
        keybd_event(vk, 0, 0, UIntPtr.Zero);
        keybd_event(vk, 0, KEYEVENTF_KEYUP, UIntPtr.Zero);
    }

    public static string CursorFlags()
    {
        CURSORINFO info = new() { cbSize = (uint)Marshal.SizeOf<CURSORINFO>() };
        if (!GetCursorInfo(ref info))
        {
            return $"GetCursorInfo failed ({Marshal.GetLastPInvokeError()})";
        }
        List<string> names = new();
        if ((info.flags & CURSOR_SHOWING) != 0)
        {
            names.Add("showing");
        }
        if ((info.flags & CURSOR_SUPPRESSED) != 0)
        {
            names.Add("suppressed");
        }
        return names.Count == 0 ? "hidden (flags 0)" : string.Join("+", names);
    }
}
