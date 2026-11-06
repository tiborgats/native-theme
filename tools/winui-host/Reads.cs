// The measurement passes of the 0.8.0 measurement plan (§2) the WinUI host
// runs after spike W's passes:
//   WP-reads     [system]: GetCaretBlinkTime, GetDoubleClickTime, UISettings'
//                TextScaleFactor; [directwrite]: the rendering parameters
//                DirectWrite gives by default and for the window's monitor;
//                [[font_fallback]]: DirectWrite's system font fallback
//                (IDWriteFontFallback::MapCharacters) per sample text and
//                locale; [[text]]: TextBlocks at the type ramp's sizes, their
//                measured sizes and their drawn pixels (text-<pass>.png, the
//                count of grey and of coloured pixels: ClearType's fringes
//                are coloured, greyscale antialiasing is not)
//   WH-defaults  [dialog] and [[dialog.button]]: a ContentDialog's buttons with
//                DefaultButton Primary (their style, padding, border, focus
//                visual and label font); [[app_bar]] and [[part]]: an
//                AppBarButton and an AppBarToggleButton standalone and in a
//                CommandBar (labels below and to the right), each named
//                template part's bounds and margins; the accent button's
//                focus visual and label font in [defaults]
//   WH-motion    [gesture_recognizer_*]: every readable property of a new
//                Microsoft.UI.Input and Windows.UI.Input GestureRecognizer
// XAML coordinates are effective pixels; screen captures physical pixels.
using System.Globalization;
using System.Reflection;
using System.Runtime.InteropServices;
using Microsoft.UI;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;
using Microsoft.UI.Xaml.Controls.Primitives;
using Microsoft.UI.Xaml.Media;
using Windows.Foundation;

namespace WinUiHost;

public partial class App
{
    // The type ramp's sizes (Caption 12, Body 14, Body Large 18, Subtitle 20,
    // Title 28, Title Large 40, Display 68) and 10 and 16 between them.
    private static readonly double[] TextSizes = { 10, 12, 14, 16, 18, 20, 28, 40, 68 };

    // Each sample's text and the locale it is mapped under: the scripts of the
    // model's per-language fallback list, Han under each CJK locale (the same
    // characters, so only the locale differs), emoji and symbols.
    private static readonly (string Locale, string Text)[] FallbackSamples =
    {
        ("en-US", "Aa"),
        ("el-GR", "Ωω"),
        ("ru-RU", "Жж"),
        ("ar-SA", "عربي"),
        ("fa-IR", "فارسی"),
        ("he-IL", "עברית"),
        ("hi-IN", "हिन्दी"),
        ("bn-IN", "বাংলা"),
        ("ta-IN", "தமிழ்"),
        ("th-TH", "ไทย"),
        ("lo-LA", "ລາວ"),
        ("km-KH", "ខ្មែរ"),
        ("my-MM", "မြန်မာ"),
        ("si-LK", "සිංහල"),
        ("hy-AM", "Հայ"),
        ("ka-GE", "ქართ"),
        ("am-ET", "አማርኛ"),
        ("ja-JP", "直あア"),
        ("zh-CN", "直汉"),
        ("zh-TW", "直漢"),
        ("zh-HK", "直漢"),
        ("ko-KR", "直한"),
        ("en-US", "😀"),
        ("en-US", "☃→✓"),
    };

    private async Task ReadPasses()
    {
        RecordSystem();
        RecordDirectWrite();
        RecordGestureRecognizers();
        await TextPass("default");
        await DialogPass();
        await AppBarPass();
    }

    private void RecordSystem()
    {
        Toml.Section section = toml.Add("[system]");
        section.Int("GetCaretBlinkTime", NativeReads.GetCaretBlinkTime());
        section.Int("GetDoubleClickTime", NativeReads.GetDoubleClickTime());
        try
        {
            section.Num("ui_settings_text_scale_factor", new Windows.UI.ViewManagement.UISettings().TextScaleFactor);
        }
        catch (Exception e)
        {
            section.Str("ui_settings_text_scale_factor_error", e.Message);
        }
    }

    private void RecordDirectWrite()
    {
        Toml.Section section = toml.Add("[directwrite]");
        IntPtr hwnd = WinRT.Interop.WindowNative.GetWindowHandle(window);
        IntPtr monitor = NativeReads.MonitorFromWindow(hwnd, NativeReads.MONITOR_DEFAULTTOPRIMARY);
        try
        {
            DWrite.Record(section, monitor, (locale, text, family, runs) =>
            {
                Toml.Section run = toml.Add("[[font_fallback]]");
                run.Str("locale", locale);
                run.Str("text", text);
                run.Str("base_family", family ?? "");
                run.StrArray("runs", runs);
            }, FallbackSamples);
        }
        catch (Exception e)
        {
            section.Str("error", e.ToString());
            errors.Add("directwrite: " + e.Message);
        }
    }

    private void RecordGestureRecognizers()
    {
        DumpProperties("gesture_recognizer_microsoft_ui_input", () => new Microsoft.UI.Input.GestureRecognizer());
        DumpProperties("gesture_recognizer_windows_ui_input", () => new Windows.UI.Input.GestureRecognizer());
    }

    private void DumpProperties(string header, Func<object> create)
    {
        Toml.Section section = toml.Add($"[{header}]");
        object instance;
        try
        {
            instance = create();
        }
        catch (Exception e)
        {
            section.Str("error", e.Message);
            return;
        }
        section.Str("type", instance.GetType().FullName ?? "");
        foreach (PropertyInfo property in instance.GetType().GetProperties(BindingFlags.Public | BindingFlags.Instance).OrderBy(p => p.Name, StringComparer.Ordinal))
        {
            if (!property.CanRead || property.GetIndexParameters().Length > 0)
            {
                continue;
            }
            try
            {
                object? value = property.GetValue(instance);
                section.Str(property.Name, Convert.ToString(value, CultureInfo.InvariantCulture) ?? "null");
            }
            catch (Exception e)
            {
                section.Str(property.Name + "_error", e.InnerException?.Message ?? e.Message);
            }
        }
    }

    // TextBlocks of the default style in black on white; the capture is the
    // screen's, the pixels the compositor drew.
    private async Task TextPass(string pass)
    {
        StackPanel panel = new()
        {
            Background = new SolidColorBrush(Colors.White),
            HorizontalAlignment = HorizontalAlignment.Left,
            VerticalAlignment = VerticalAlignment.Top,
            Padding = new Thickness(8),
        };
        List<TextBlock> blocks = new();
        foreach (double size in TextSizes)
        {
            TextBlock block = new()
            {
                Text = "Ag Typography 0123",
                FontSize = size,
                Foreground = new SolidColorBrush(Colors.Black),
            };
            blocks.Add(block);
            panel.Children.Add(block);
        }
        root!.Children.Add(panel);
        await Task.Delay(SettleMilliseconds);
        foreach (TextBlock block in blocks)
        {
            Toml.Section section = toml.Add("[[text]]");
            section.Str("pass", pass);
            section.Num("font_size", block.FontSize);
            section.Str("font_family", block.FontFamily.Source);
            section.Str("font_weight", block.FontWeight.Weight.ToString(CultureInfo.InvariantCulture));
            section.Bool("is_text_scale_factor_enabled", block.IsTextScaleFactorEnabled);
            section.Num("line_height", block.LineHeight);
            section.Str("line_stacking_strategy", block.LineStackingStrategy.ToString());
            section.Str("optical_margin_alignment", block.OpticalMarginAlignment.ToString());
            section.Str("text_line_bounds", block.TextLineBounds.ToString());
            section.Num("baseline_offset", block.BaselineOffset);
            section.Size("actual_size", block.ActualWidth, block.ActualHeight);
            section.Size("desired_size", block.DesiredSize.Width, block.DesiredSize.Height);
            RecordBounds(section, "bounds", block, root);
        }
        Toml.Section capture = toml.Add("[[text_capture]]");
        capture.Str("pass", pass);
        RecordBounds(capture, "panel_bounds", panel, root);
        CaptureElement(panel, $"text-{pass}.png", capture);
        root.Children.Remove(panel);
        await Task.Delay(SettleMilliseconds);
    }

    // The element's part of the screen, saved as PNG, with the count of its
    // pixels that are not white: grey (R = G = B) and coloured.
    private void CaptureElement(FrameworkElement element, string name, Toml.Section section)
    {
        try
        {
            IntPtr hwnd = WinRT.Interop.WindowNative.GetWindowHandle(window);
            Native.POINT origin = new();
            Native.ClientToScreen(hwnd, ref origin);
            Rect bounds = element.TransformToVisual(root).TransformBounds(new Rect(0, 0, element.ActualWidth, element.ActualHeight));
            double scale = root!.XamlRoot.RasterizationScale;
            int x = origin.X + (int)Math.Round(bounds.X * scale);
            int y = origin.Y + (int)Math.Round(bounds.Y * scale);
            int width = Math.Max(1, (int)Math.Round(bounds.Width * scale));
            int height = Math.Max(1, (int)Math.Round(bounds.Height * scale));
            using System.Drawing.Bitmap bitmap = new(width, height);
            using (System.Drawing.Graphics graphics = System.Drawing.Graphics.FromImage(bitmap))
            {
                graphics.CopyFromScreen(x, y, 0, 0, new System.Drawing.Size(width, height));
            }
            bitmap.Save(Path.Combine(outDir, name), System.Drawing.Imaging.ImageFormat.Png);
            uint grey = 0;
            uint coloured = 0;
            int spread = 0;
            for (int py = 0; py < height; py++)
            {
                for (int px = 0; px < width; px++)
                {
                    System.Drawing.Color c = bitmap.GetPixel(px, py);
                    if (c.R == 255 && c.G == 255 && c.B == 255)
                    {
                        continue;
                    }
                    if (c.R == c.G && c.G == c.B)
                    {
                        grey++;
                    }
                    else
                    {
                        coloured++;
                        spread = Math.Max(spread, Math.Max(c.R, Math.Max(c.G, c.B)) - Math.Min(c.R, Math.Min(c.G, c.B)));
                    }
                }
            }
            section.Str("png", name);
            section.Rect("screen_rect", new Windows.Graphics.RectInt32(x, y, width, height));
            section.Int("grey_pixels", grey);
            section.Int("coloured_pixels", coloured);
            section.Int("largest_channel_spread", (uint)spread);
        }
        catch (Exception e)
        {
            section.Str("capture_error", e.Message);
            errors.Add($"{name}: {e.Message}");
        }
    }

    // A ContentDialog with three buttons and DefaultButton Primary, shown in
    // the window; its buttons are found by their template names.
    private async Task DialogPass()
    {
        Toml.Section section = toml.Add("[dialog]");
        ContentDialog dialog = new()
        {
            XamlRoot = root!.XamlRoot,
            Title = "Dialog",
            Content = "Content",
            PrimaryButtonText = "Primary",
            SecondaryButtonText = "Secondary",
            CloseButtonText = "Close",
            DefaultButton = ContentDialogButton.Primary,
        };
        TaskCompletionSource opened = new();
        TaskCompletionSource closed = new();
        dialog.Opened += (_, _) => opened.TrySetResult();
        dialog.Closed += (_, _) => closed.TrySetResult();
        _ = dialog.ShowAsync();
        section.Bool("opened_event", await WithTimeout(opened.Task, EventTimeoutMilliseconds));
        await Task.Delay(SettleMilliseconds);
        section.Str("default_button", dialog.DefaultButton.ToString());
        Style? accentStyle = Resources.TryGetValue("AccentButtonStyle", out object? style) ? style as Style : null;
        Style? defaultStyle = Resources.TryGetValue("DefaultButtonStyle", out object? plain) ? plain as Style : null;
        section.Bool("accent_button_style_found", accentStyle is not null);
        section.Bool("default_button_style_found", defaultStyle is not null);
        List<DependencyObject> trees = new() { dialog };
        foreach (Popup popup in VisualTreeHelper.GetOpenPopupsForXamlRoot(root.XamlRoot))
        {
            if (popup.Child is not null)
            {
                trees.Add(popup.Child);
            }
        }
        foreach (string name in new[] { "PrimaryButton", "SecondaryButton", "CloseButton" })
        {
            Toml.Section buttonSection = toml.Add("[[dialog.button]]");
            buttonSection.Str("name", name);
            Button? button = trees.Select(tree => FindNamed(tree, name)).OfType<Button>().FirstOrDefault();
            if (button is null)
            {
                buttonSection.Str("error", "not found in the dialog's tree or its popups");
                continue;
            }
            buttonSection.Bool("style_is_accent_button_style", accentStyle is not null && ReferenceEquals(button.Style, accentStyle));
            buttonSection.Bool("style_is_default_button_style", defaultStyle is not null && ReferenceEquals(button.Style, defaultStyle));
            buttonSection.Bool("style_is_null", button.Style is null);
            RecordControl(buttonSection, "button", button);
            RecordFocusVisual(buttonSection, "button", button);
            RecordLabel(buttonSection, "button", button);
            RecordBounds(buttonSection, "bounds", button, null);
        }
        CaptureScreen("screen-dialog.png");
        dialog.Hide();
        section.Bool("closed_event", await WithTimeout(closed.Task, EventTimeoutMilliseconds));
        await Task.Delay(SettleMilliseconds);
    }

    // AppBarButton and AppBarToggleButton (a SymbolIcon and a label), standalone
    // and in a CommandBar with its labels below (the default) and to the right.
    private async Task AppBarPass()
    {
        foreach ((string pass, CommandBarDefaultLabelPosition? position) in new (string, CommandBarDefaultLabelPosition?)[]
        {
            ("standalone", null),
            ("command_bar", CommandBarDefaultLabelPosition.Bottom),
            ("command_bar_right", CommandBarDefaultLabelPosition.Right),
        })
        {
            AppBarButton button = new() { Icon = new SymbolIcon(Symbol.Add), Label = "Add" };
            AppBarToggleButton toggle = new() { Icon = new SymbolIcon(Symbol.Pin), Label = "Pin" };
            FrameworkElement container;
            CommandBar? bar = null;
            if (position is CommandBarDefaultLabelPosition labels)
            {
                bar = new CommandBar { DefaultLabelPosition = labels, IsDynamicOverflowEnabled = false };
                bar.PrimaryCommands.Add(button);
                bar.PrimaryCommands.Add(toggle);
                container = bar;
            }
            else
            {
                StackPanel panel = new() { Orientation = Orientation.Horizontal };
                panel.Children.Add(button);
                panel.Children.Add(toggle);
                container = panel;
            }
            container.HorizontalAlignment = HorizontalAlignment.Left;
            container.VerticalAlignment = VerticalAlignment.Top;
            root!.Children.Add(container);
            await Task.Delay(SettleMilliseconds);
            foreach ((string kind, Control control) in new (string, Control)[] { ("app_bar_button", button), ("app_bar_toggle_button", toggle) })
            {
                Toml.Section section = toml.Add("[[app_bar]]");
                section.Str("pass", pass);
                section.Str("kind", kind);
                if (bar is not null)
                {
                    section.Str("default_label_position", bar.DefaultLabelPosition.ToString());
                }
                RecordControl(section, "control", control);
                RecordFocusVisual(section, "control", control);
                section.Bool("control_style_null", control.Style is null);
                RecordBounds(section, "bounds", control, root);
                RecordParts($"{pass}.{kind}", control);
            }
            CaptureScreen($"screen-app_bar-{pass}.png");
            root.Children.Remove(container);
            await Task.Delay(SettleMilliseconds);
        }
    }

    private static void RecordFocusVisual(Toml.Section section, string name, Control control)
    {
        section.Str(name + "_focus_visual_margin", control.FocusVisualMargin.ToString());
        section.Str(name + "_focus_visual_primary_thickness", control.FocusVisualPrimaryThickness.ToString());
        section.Str(name + "_focus_visual_secondary_thickness", control.FocusVisualSecondaryThickness.ToString());
        section.Str(name + "_focus_visual_primary_brush", BrushText(control.FocusVisualPrimaryBrush));
        section.Str(name + "_focus_visual_secondary_brush", BrushText(control.FocusVisualSecondaryBrush));
        section.Bool(name + "_use_system_focus_visuals", control.UseSystemFocusVisuals);
        section.Num(name + "_min_width", control.MinWidth);
        section.Num(name + "_min_height", control.MinHeight);
    }

    // The first TextBlock inside the control: the label the content presenter
    // or the template draws.
    private void RecordLabel(Toml.Section section, string name, DependencyObject control)
    {
        TextBlock? label = Descendants(control).OfType<TextBlock>().FirstOrDefault(t => !string.IsNullOrEmpty(t.Text));
        if (label is null)
        {
            section.Str(name + "_label_error", "no TextBlock with text");
            return;
        }
        section.Str(name + "_label_text", label.Text);
        section.Str(name + "_label_font_family", label.FontFamily.Source);
        section.Num(name + "_label_font_size", label.FontSize);
        section.Str(name + "_label_font_weight", label.FontWeight.Weight.ToString(CultureInfo.InvariantCulture));
        section.Str(name + "_label_font_style", label.FontStyle.ToString());
        section.Str(name + "_label_font_stretch", label.FontStretch.ToString());
        section.Size(name + "_label_actual_size", label.ActualWidth, label.ActualHeight);
        if (control is UIElement element)
        {
            RecordBounds(section, name + "_label_bounds", label, element);
        }
    }

    private void RecordParts(string owner, Control control)
    {
        foreach (FrameworkElement part in Descendants(control).OfType<FrameworkElement>())
        {
            if (string.IsNullOrEmpty(part.Name))
            {
                continue;
            }
            Toml.Section section = toml.Add("[[part]]");
            section.Str("owner", owner);
            section.Str("name", part.Name);
            section.Str("type", part.GetType().FullName ?? "");
            section.Str("visibility", part.Visibility.ToString());
            section.Str("margin", part.Margin.ToString());
            section.Str("horizontal_alignment", part.HorizontalAlignment.ToString());
            section.Str("vertical_alignment", part.VerticalAlignment.ToString());
            section.Num("width", part.Width);
            section.Num("height", part.Height);
            switch (part)
            {
                case Border border:
                    section.Str("padding", border.Padding.ToString());
                    section.Str("border_thickness", border.BorderThickness.ToString());
                    break;
                case ContentPresenter presenter:
                    section.Str("padding", presenter.Padding.ToString());
                    break;
                case Control inner:
                    section.Str("padding", inner.Padding.ToString());
                    break;
                case TextBlock text:
                    section.Str("text", text.Text);
                    section.Str("font_family", text.FontFamily.Source);
                    section.Num("font_size", text.FontSize);
                    section.Str("font_weight", text.FontWeight.Weight.ToString(CultureInfo.InvariantCulture));
                    break;
            }
            section.Size("actual_size", part.ActualWidth, part.ActualHeight);
            RecordBounds(section, "bounds", part, control);
        }
    }

    private static IEnumerable<DependencyObject> Descendants(DependencyObject element)
    {
        int count = VisualTreeHelper.GetChildrenCount(element);
        for (int i = 0; i < count; i++)
        {
            DependencyObject child = VisualTreeHelper.GetChild(element, i);
            yield return child;
            foreach (DependencyObject descendant in Descendants(child))
            {
                yield return descendant;
            }
        }
    }

    private static FrameworkElement? FindNamed(DependencyObject tree, string name)
    {
        if (tree is FrameworkElement self && self.Name == name)
        {
            return self;
        }
        return Descendants(tree).OfType<FrameworkElement>().FirstOrDefault(element => element.Name == name);
    }
}

internal static class NativeReads
{
    public const uint MONITOR_DEFAULTTOPRIMARY = 1;

    [DllImport("user32.dll")]
    public static extern uint GetCaretBlinkTime();

    [DllImport("user32.dll")]
    public static extern uint GetDoubleClickTime();

    [DllImport("user32.dll")]
    public static extern IntPtr MonitorFromWindow(IntPtr hwnd, uint flags);
}

// DirectWrite through its COM vtables (slot numbers from dwrite.h's
// declaration order, IUnknown's three first): the shared factory's rendering
// parameters and its system font fallback. The text analysis source the
// fallback reads is a small native object whose methods are this class's
// UnmanagedCallersOnly functions; it serves one text and one locale at a time.
internal static unsafe class DWrite
{
    private const int S_OK = 0;
    private const int E_NOINTERFACE = unchecked((int)0x80004002);
    private const int DWRITE_FACTORY_TYPE_SHARED = 0;
    private const int DWRITE_FONT_WEIGHT_NORMAL = 400;
    private const int DWRITE_FONT_STYLE_NORMAL = 0;
    private const int DWRITE_FONT_STRETCH_NORMAL = 5;

    private static readonly Guid UnknownIid = new("00000000-0000-0000-c000-000000000046");
    private static readonly Guid Factory2Iid = new("0439fc60-ca44-4994-8dee-3a9af7b732ec");
    private static readonly Guid RenderingParams1Iid = new("94413cf4-a6fc-4248-8b50-6674348fcad3");
    private static readonly Guid RenderingParams2Iid = new("f9d711c3-9777-40ae-87e8-3e5af9bf0948");
    private static readonly Guid RenderingParams3Iid = new("b7924baa-391b-412a-8c5c-e44cc2d867dc");
    private static readonly Guid SourceIid = new("688e1a58-5094-47c8-adc8-fbcea60ae92b");

    private static IntPtr sourceText;
    private static uint sourceLength;
    private static IntPtr sourceLocale;

    [DllImport("dwrite.dll")]
    private static extern int DWriteCreateFactory(int factoryType, ref Guid iid, out IntPtr factory);

    private static void** Vtbl(IntPtr instance) => *(void***)instance;

    private static void Release(IntPtr instance)
    {
        if (instance != IntPtr.Zero)
        {
            ((delegate* unmanaged<IntPtr, uint>)Vtbl(instance)[2])(instance);
        }
    }

    private static IntPtr QueryInterface(IntPtr instance, Guid iid)
    {
        IntPtr result = IntPtr.Zero;
        int hr = ((delegate* unmanaged<IntPtr, Guid*, IntPtr*, int>)Vtbl(instance)[0])(instance, &iid, &result);
        return hr == S_OK ? result : IntPtr.Zero;
    }

    public static void Record(Toml.Section section, IntPtr monitor, Action<string, string, string?, List<string>> fallbackRun, (string Locale, string Text)[] samples)
    {
        Guid iid = Factory2Iid;
        int hr = DWriteCreateFactory(DWRITE_FACTORY_TYPE_SHARED, ref iid, out IntPtr factory);
        if (hr != S_OK || factory == IntPtr.Zero)
        {
            section.Str("factory_error", $"DWriteCreateFactory(IDWriteFactory2): 0x{hr:X8}");
            return;
        }
        try
        {
            IntPtr parameters = IntPtr.Zero;
            hr = ((delegate* unmanaged<IntPtr, IntPtr*, int>)Vtbl(factory)[10])(factory, &parameters);
            if (hr == S_OK)
            {
                RecordParams(section, "default_", parameters);
                Release(parameters);
            }
            else
            {
                section.Str("default_error", $"CreateRenderingParams: 0x{hr:X8}");
            }
            parameters = IntPtr.Zero;
            hr = ((delegate* unmanaged<IntPtr, IntPtr, IntPtr*, int>)Vtbl(factory)[11])(factory, monitor, &parameters);
            if (hr == S_OK)
            {
                RecordParams(section, "monitor_", parameters);
                Release(parameters);
            }
            else
            {
                section.Str("monitor_error", $"CreateMonitorRenderingParams: 0x{hr:X8}");
            }

            IntPtr fallback = IntPtr.Zero;
            hr = ((delegate* unmanaged<IntPtr, IntPtr*, int>)Vtbl(factory)[26])(factory, &fallback);
            if (hr != S_OK || fallback == IntPtr.Zero)
            {
                section.Str("fallback_error", $"GetSystemFontFallback: 0x{hr:X8}");
                return;
            }
            IntPtr collection = IntPtr.Zero;
            hr = ((delegate* unmanaged<IntPtr, IntPtr*, int, int>)Vtbl(factory)[3])(factory, &collection, 0);
            if (hr != S_OK)
            {
                section.Str("collection_error", $"GetSystemFontCollection: 0x{hr:X8}");
                collection = IntPtr.Zero;
            }
            IntPtr source = CreateSource();
            foreach (string? family in new string?[] { null, "Segoe UI", "Segoe UI Variable", "Segoe UI Variable Text" })
            {
                foreach ((string locale, string text) in samples)
                {
                    fallbackRun(locale, text, family, MapAll(fallback, source, collection, locale, text, family));
                }
            }
            Release(collection);
            Release(fallback);
        }
        finally
        {
            Release(factory);
        }
    }

    private static void RecordParams(Toml.Section section, string prefix, IntPtr parameters)
    {
        void** v = Vtbl(parameters);
        Float(section, prefix + "gamma", ((delegate* unmanaged<IntPtr, float>)v[3])(parameters));
        Float(section, prefix + "enhanced_contrast", ((delegate* unmanaged<IntPtr, float>)v[4])(parameters));
        Float(section, prefix + "cleartype_level", ((delegate* unmanaged<IntPtr, float>)v[5])(parameters));
        int geometry = ((delegate* unmanaged<IntPtr, int>)v[6])(parameters);
        section.Str(prefix + "pixel_geometry", geometry switch { 0 => "FLAT", 1 => "RGB", 2 => "BGR", _ => geometry.ToString(CultureInfo.InvariantCulture) });
        int mode = ((delegate* unmanaged<IntPtr, int>)v[7])(parameters);
        section.Str(prefix + "rendering_mode", RenderingMode(mode));
        IntPtr p1 = QueryInterface(parameters, RenderingParams1Iid);
        if (p1 != IntPtr.Zero)
        {
            Float(section, prefix + "grayscale_enhanced_contrast", ((delegate* unmanaged<IntPtr, float>)Vtbl(p1)[8])(p1));
            Release(p1);
        }
        IntPtr p2 = QueryInterface(parameters, RenderingParams2Iid);
        if (p2 != IntPtr.Zero)
        {
            int grid = ((delegate* unmanaged<IntPtr, int>)Vtbl(p2)[9])(p2);
            section.Str(prefix + "grid_fit_mode", grid switch { 0 => "DEFAULT", 1 => "DISABLED", 2 => "ENABLED", _ => grid.ToString(CultureInfo.InvariantCulture) });
            Release(p2);
        }
        IntPtr p3 = QueryInterface(parameters, RenderingParams3Iid);
        if (p3 != IntPtr.Zero)
        {
            int mode1 = ((delegate* unmanaged<IntPtr, int>)Vtbl(p3)[10])(p3);
            section.Str(prefix + "rendering_mode1", RenderingMode1(mode1));
            Release(p3);
        }
    }

    private static string RenderingMode(int mode) => mode switch
    {
        0 => "DEFAULT",
        1 => "ALIASED",
        2 => "GDI_CLASSIC",
        3 => "GDI_NATURAL",
        4 => "NATURAL",
        5 => "NATURAL_SYMMETRIC",
        6 => "OUTLINE",
        _ => mode.ToString(CultureInfo.InvariantCulture),
    };

    private static string RenderingMode1(int mode) => mode switch
    {
        7 => "NATURAL_SYMMETRIC_DOWNSAMPLED",
        _ => RenderingMode(mode),
    };

    private static void Float(Toml.Section section, string key, float value)
    {
        string text = value.ToString("R", CultureInfo.InvariantCulture);
        if (!text.Contains('.') && !text.Contains('E') && !text.Contains('N') && !text.Contains('I'))
        {
            text += ".0";
        }
        section.Lines.Add($"{key} = {text}");
    }

    // Maps the whole text, run by run: each run's characters, its font's
    // family, face, weight and scale, or "none" where no font has them.
    private static List<string> MapAll(IntPtr fallback, IntPtr source, IntPtr collection, string locale, string text, string? family)
    {
        List<string> runs = new();
        sourceText = Marshal.StringToHGlobalUni(text);
        sourceLength = (uint)text.Length;
        sourceLocale = Marshal.StringToHGlobalUni(locale);
        IntPtr familyName = family is null ? IntPtr.Zero : Marshal.StringToHGlobalUni(family);
        try
        {
            uint position = 0;
            while (position < sourceLength)
            {
                uint mapped = 0;
                IntPtr font = IntPtr.Zero;
                float scale = 0;
                int hr = ((delegate* unmanaged<IntPtr, IntPtr, uint, uint, IntPtr, IntPtr, int, int, int, uint*, IntPtr*, float*, int>)Vtbl(fallback)[3])(
                    fallback, source, position, sourceLength - position, collection, familyName,
                    DWRITE_FONT_WEIGHT_NORMAL, DWRITE_FONT_STYLE_NORMAL, DWRITE_FONT_STRETCH_NORMAL, &mapped, &font, &scale);
                if (hr != S_OK)
                {
                    runs.Add($"error MapCharacters 0x{hr:X8} at {position}");
                    break;
                }
                if (mapped == 0)
                {
                    runs.Add($"error MapCharacters mapped nothing at {position}");
                    Release(font);
                    break;
                }
                string characters = text.Substring((int)position, (int)Math.Min(mapped, sourceLength - position));
                string describe = font == IntPtr.Zero ? "none" : DescribeFont(font);
                runs.Add($"{characters} -> {describe}; scale {scale.ToString("R", CultureInfo.InvariantCulture)}");
                Release(font);
                position += mapped;
            }
        }
        finally
        {
            Marshal.FreeHGlobal(sourceText);
            Marshal.FreeHGlobal(sourceLocale);
            if (familyName != IntPtr.Zero)
            {
                Marshal.FreeHGlobal(familyName);
            }
            sourceText = IntPtr.Zero;
            sourceLocale = IntPtr.Zero;
            sourceLength = 0;
        }
        return runs;
    }

    private static string DescribeFont(IntPtr font)
    {
        void** v = Vtbl(font);
        string familyName = "?";
        IntPtr fontFamily = IntPtr.Zero;
        if (((delegate* unmanaged<IntPtr, IntPtr*, int>)v[3])(font, &fontFamily) == S_OK && fontFamily != IntPtr.Zero)
        {
            IntPtr names = IntPtr.Zero;
            if (((delegate* unmanaged<IntPtr, IntPtr*, int>)Vtbl(fontFamily)[6])(fontFamily, &names) == S_OK && names != IntPtr.Zero)
            {
                familyName = LocalizedString(names);
                Release(names);
            }
            Release(fontFamily);
        }
        string faceName = "?";
        IntPtr faces = IntPtr.Zero;
        if (((delegate* unmanaged<IntPtr, IntPtr*, int>)v[8])(font, &faces) == S_OK && faces != IntPtr.Zero)
        {
            faceName = LocalizedString(faces);
            Release(faces);
        }
        int weight = ((delegate* unmanaged<IntPtr, int>)v[4])(font);
        return $"{familyName} {faceName} (weight {weight.ToString(CultureInfo.InvariantCulture)})";
    }

    private static string LocalizedString(IntPtr strings)
    {
        void** v = Vtbl(strings);
        uint index = 0;
        int exists = 0;
        fixed (char* english = "en-us")
        {
            if (((delegate* unmanaged<IntPtr, char*, uint*, int*, int>)v[4])(strings, english, &index, &exists) != S_OK || exists == 0)
            {
                index = 0;
            }
        }
        uint length = 0;
        if (((delegate* unmanaged<IntPtr, uint, uint*, int>)v[7])(strings, index, &length) != S_OK)
        {
            return "?";
        }
        char[] buffer = new char[length + 1];
        fixed (char* text = buffer)
        {
            if (((delegate* unmanaged<IntPtr, uint, char*, uint, int>)v[8])(strings, index, text, length + 1) != S_OK)
            {
                return "?";
            }
        }
        return new string(buffer, 0, (int)length);
    }

    private static IntPtr CreateSource()
    {
        IntPtr* vtbl = (IntPtr*)NativeMemory.Alloc(8, (nuint)sizeof(IntPtr));
        vtbl[0] = (IntPtr)(void*)(delegate* unmanaged<IntPtr, Guid*, IntPtr*, int>)&SourceQueryInterface;
        vtbl[1] = (IntPtr)(void*)(delegate* unmanaged<IntPtr, uint>)&SourceAddRef;
        vtbl[2] = (IntPtr)(void*)(delegate* unmanaged<IntPtr, uint>)&SourceRelease;
        vtbl[3] = (IntPtr)(void*)(delegate* unmanaged<IntPtr, uint, char**, uint*, int>)&GetTextAtPosition;
        vtbl[4] = (IntPtr)(void*)(delegate* unmanaged<IntPtr, uint, char**, uint*, int>)&GetTextBeforePosition;
        vtbl[5] = (IntPtr)(void*)(delegate* unmanaged<IntPtr, int>)&GetParagraphReadingDirection;
        vtbl[6] = (IntPtr)(void*)(delegate* unmanaged<IntPtr, uint, uint*, char**, int>)&GetLocaleName;
        vtbl[7] = (IntPtr)(void*)(delegate* unmanaged<IntPtr, uint, uint*, IntPtr*, int>)&GetNumberSubstitution;
        IntPtr* instance = (IntPtr*)NativeMemory.Alloc(1, (nuint)sizeof(IntPtr));
        instance[0] = (IntPtr)vtbl;
        return (IntPtr)instance;
    }

    [UnmanagedCallersOnly]
    private static int SourceQueryInterface(IntPtr self, Guid* iid, IntPtr* result)
    {
        if (*iid == UnknownIid || *iid == SourceIid)
        {
            *result = self;
            return S_OK;
        }
        *result = IntPtr.Zero;
        return E_NOINTERFACE;
    }

    // The source lives until the process ends: its count is not kept.
    [UnmanagedCallersOnly]
    private static uint SourceAddRef(IntPtr self) => 1;

    [UnmanagedCallersOnly]
    private static uint SourceRelease(IntPtr self) => 1;

    [UnmanagedCallersOnly]
    private static int GetTextAtPosition(IntPtr self, uint position, char** text, uint* length)
    {
        if (position >= sourceLength)
        {
            *text = null;
            *length = 0;
        }
        else
        {
            *text = (char*)sourceText + position;
            *length = sourceLength - position;
        }
        return S_OK;
    }

    [UnmanagedCallersOnly]
    private static int GetTextBeforePosition(IntPtr self, uint position, char** text, uint* length)
    {
        if (position == 0 || position > sourceLength)
        {
            *text = null;
            *length = 0;
        }
        else
        {
            *text = (char*)sourceText;
            *length = position;
        }
        return S_OK;
    }

    // DWRITE_READING_DIRECTION_LEFT_TO_RIGHT.
    [UnmanagedCallersOnly]
    private static int GetParagraphReadingDirection(IntPtr self) => 0;

    [UnmanagedCallersOnly]
    private static int GetLocaleName(IntPtr self, uint position, uint* length, char** locale)
    {
        *length = position < sourceLength ? sourceLength - position : 0;
        *locale = (char*)sourceLocale;
        return S_OK;
    }

    [UnmanagedCallersOnly]
    private static int GetNumberSubstitution(IntPtr self, uint position, uint* length, IntPtr* substitution)
    {
        *length = position < sourceLength ? sourceLength - position : 0;
        *substitution = IntPtr.Zero;
        return S_OK;
    }
}
