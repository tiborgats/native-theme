// The iOS host of the UIKit measurements (the 0.8.0 plan, D4; spike I; the iOS
// lane's second round).
//
// Two modes, chosen by the first launch argument:
//
// `run<N>` (tools/ios-host/run.sh, `simctl launch`): one page per appearance
// (the window's `overrideUserInterfaceStyle`, light and dark), specimen and
// size. The specimens: a UIButton in each UIButton.Configuration style the SDK
// offers (iOS 26 adds the four Liquid Glass styles) at each `buttonSize`; at
// `medium`, the styles with each variant the catalogue names for ios-* (a role,
// a corner style, an image and its placement, a subtitle, the activity
// indicator, a menu and its indicator); `UIButton(type:)` of the system types;
// bar button items in a UIToolbar; a navigation bar's back button in each
// display mode; UIPasteControl in each display mode; AVRoutePickerView. A page
// is a grid of cells, a row per state and a column per backdrop. The states:
// normal; highlighted (`isHighlighted = true`, no touch); disabled; toggle_off
// (`changesSelectionAsPrimaryAction`, not selected); selected (the same,
// `isSelected = true`); selected_highlighted (both); focused (a focus update to
// the system column's specimen is requested, and whether UIKit focused it is
// recorded). A specimen lists the states it can take. The backdrops: the
// system background, opaque black and opaque white, all else equal, for a
// per-pixel solve of the layer's colour and alpha (tools/ios-host/solve.swift);
// a glass style has a second page over a known pattern, with a column of the
// pattern alone. UIKit's animations are disabled and each page settles for two
// seconds. Before the pages, the app writes UIKit's own values to index.toml:
// every UIColor class colour run.sh found in the SDK's headers, resolved under
// each appearance, contrast and level; the text styles' fonts and
// UIFontMetrics' scaling at every content size category; symbol image sizes;
// the caret; the motion and behaviour values UIKit exposes. Each render of the
// system column adds the button's configuration as `updated(for:)` resolves it
// for the state, its title font and its view tree.
//
// `press<N>` (launched by the UI test, IosHostUITests/PressTests.swift): the
// glass styles at each size and, at medium, the other styles and the system
// type, pressed by a real touch (`XCUIElement.press(forDuration:)`). The app
// arms one button at a time (accessibilityIdentifier `press-<k>`); once the
// touch has been down for `pressSettleSeconds` it requests the screenshot, and
// keeps the capture only if the touch was still down when the screenshot
// arrived. UIKit's animations stay enabled: the press is one.
//
// Each cell is drawn by the app (`drawHierarchy(in:afterScreenUpdates:)` into a
// UIGraphicsImageRenderer of the screen's scale and the standard range) to
// draw/, and cropped from the simulator's own screenshot of the page to
// screen/. The screenshot is taken by run.sh: the app writes
// sync/ready-<n>-<attempt>, the script writes sync/screen-<n>-<attempt>.png. A
// screenshot whose page margins are not the plain system background, or whose
// page code (a row of red and blue squares under the grid, the page's number
// in binary) is not the page's, was covered by a system banner or is stale: it
// is taken again (`maxRetakes`). A page is kept once two screenshots agree
// (`maxSettleShots`). The retakes are listed in sync/retakes. index.toml
// records the simulator, the build (the DT keys Xcode writes into the bundle's
// Info.plist), the screen, UIKit's values and each cell.
//
// Writes into Documents/<run>/ of its data container. The `run` mode exits (1
// where a step failed); the `press` mode shows `press-done` for the UI test.

import AVKit
import UIKit

@main
final class AppDelegate: UIResponder, UIApplicationDelegate {
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?
    ) -> Bool {
        true
    }
}

final class SceneDelegate: UIResponder, UIWindowSceneDelegate {
    var window: UIWindow?

    func scene(
        _ scene: UIScene, willConnectTo session: UISceneSession,
        options connectionOptions: UIScene.ConnectionOptions
    ) {
        guard let windowScene = scene as? UIWindowScene else { return }
        let window = UIWindow(windowScene: windowScene)
        window.rootViewController = HostViewController()
        window.makeKeyAndVisible()
        self.window = window
    }
}

struct Style {
    let name: String
    let make: () -> UIButton.Configuration
}

func configurationStyles() -> [Style] {
    var styles = [
        Style(name: "plain", make: { .plain() }),
        Style(name: "gray", make: { .gray() }),
        Style(name: "tinted", make: { .tinted() }),
        Style(name: "filled", make: { .filled() }),
        Style(name: "borderless", make: { .borderless() }),
        Style(name: "bordered", make: { .bordered() }),
        Style(name: "bordered_tinted", make: { .borderedTinted() }),
        Style(name: "bordered_prominent", make: { .borderedProminent() }),
    ]
    #if compiler(>=6.2)
    if #available(iOS 26.0, *) {
        styles += [
            Style(name: "glass", make: { .glass() }),
            Style(name: "prominent_glass", make: { .prominentGlass() }),
            Style(name: "clear_glass", make: { .clearGlass() }),
            Style(name: "prominent_clear_glass", make: { .prominentClearGlass() }),
        ]
    }
    #endif
    return styles
}

let glassStyles: Set<String> = ["glass", "prominent_glass", "clear_glass", "prominent_clear_glass"]

typealias SizeChoice = (name: String, size: UIButton.Configuration.Size)

let sizes: [SizeChoice] = [("mini", .mini), ("small", .small), ("medium", .medium), ("large", .large)]
let medium: SizeChoice = ("medium", .medium)
// A specimen that has no UIButton.Configuration size.
let noSize: SizeChoice = ("default", .medium)

let settleSeconds = 2.0
let pressSettleSeconds = 1.5
let pressRearmSeconds = 0.5

// A screenshot whose margins hold more than `overlayPixels` pixels differing
// from the system background by more than `overlayLevels` in a channel, or
// whose page code does not read as the page's number, is taken again, up to
// `maxRetakes` times, `retakeDelaySeconds` apart: a system banner (iOS 26.5's
// "Ready for Apple Intelligence") can slide over the page.
let overlayLevels = 32
let overlayPixels = 50
let maxRetakes = 5
let retakeDelaySeconds = 6.0

// A page is kept when two screenshots `settleShotSeconds` apart agree within
// `settledLevels` in every cell, at most `maxSettleShots` screenshots after the
// first: Liquid Glass adapts to the luminance behind it, and that transition
// can outlast `settleSeconds` (iOS 26.5, dark glass over white). The activity
// indicator turns without end, so its pages keep the first screenshot.
let settledLevels = 1
let settleShotSeconds = 1.0
let maxSettleShots = 10

let margin: CGFloat = 16
let cellHeight: CGFloat = 92
let gap: CGFloat = 8
let codeBits = 12
let codeSquare: CGFloat = 6
let patternSquare: CGFloat = 6
let patternColors: [(name: String, color: UIColor)] = [
    ("#000000", UIColor(red: 0, green: 0, blue: 0, alpha: 1)),
    ("#ffffff", UIColor(red: 1, green: 1, blue: 1, alpha: 1)),
    ("#ff0000", UIColor(red: 1, green: 0, blue: 0, alpha: 1)),
    ("#0000ff", UIColor(red: 0, green: 0, blue: 1, alpha: 1)),
]

func samples(_ image: CGImage) -> [UInt8]? {
    guard let space = CGColorSpace(name: CGColorSpace.sRGB),
        let context = CGContext(
            data: nil, width: image.width, height: image.height, bitsPerComponent: 8,
            bytesPerRow: image.width * 4, space: space,
            bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)
    else { return nil }
    context.draw(image, in: CGRect(x: 0, y: 0, width: image.width, height: image.height))
    guard let data = context.makeImage()?.dataProvider?.data as Data? else { return nil }
    return [UInt8](data)
}

typealias Appearance = (name: String, style: UIUserInterfaceStyle)

let appearances: [Appearance] = [("light", .light), ("dark", .dark)]

enum ButtonState: String {
    case normal
    case highlighted
    case disabled
    case toggleOff = "toggle_off"
    case selected
    case selectedHighlighted = "selected_highlighted"
    case focused
    case pressed
    case pressedSelected = "pressed_selected"
}

let allStates: [ButtonState] = [.normal, .highlighted, .disabled, .toggleOff, .selected, .selectedHighlighted, .focused]
let untoggledStates: [ButtonState] = [.normal, .highlighted, .disabled, .focused]

enum Fill: String {
    case system
    case black
    case white
    case pattern
}

struct Backdrop {
    let fill: Fill
    let holdsSpecimen: Bool

    var name: String { holdsSpecimen ? fill.rawValue : "\(fill.rawValue)_alone" }
}

let solveBackdrops = [Backdrop(fill: .system, holdsSpecimen: true), Backdrop(fill: .black, holdsSpecimen: true),
                      Backdrop(fill: .white, holdsSpecimen: true)]
let patternBackdrops = [Backdrop(fill: .pattern, holdsSpecimen: true), Backdrop(fill: .pattern, holdsSpecimen: false)]

/// What a specimen puts in a cell: the view placed there, and the control,
/// button or bar item its states are set on.
final class Sample {
    let view: UIView
    let control: UIControl?
    let button: UIButton?
    let barItem: UIBarButtonItem?
    /// The size the harness gives a view that has no intrinsic size.
    let harnessSize: CGSize?
    /// Whether the view spans the cell's width, inset by `barInset`.
    let spansWidth: Bool

    init(
        view: UIView, control: UIControl? = nil, button: UIButton? = nil, barItem: UIBarButtonItem? = nil,
        harnessSize: CGSize? = nil, spansWidth: Bool = false
    ) {
        self.view = view
        self.control = control
        self.button = button
        self.barItem = barItem
        self.harnessSize = harnessSize
        self.spansWidth = spansWidth
    }
}

let barInset: CGFloat = 12

struct Specimen {
    let name: String
    let kind: String
    let style: String
    let variant: [(key: String, value: String)]
    let sizes: [SizeChoice]
    let states: [ButtonState]
    let make: @MainActor (UIButton.Configuration.Size) -> Sample

    /// Whether the specimen moves without end (the activity indicator).
    var animated: Bool { variant.contains { $0.key == "activity_indicator" } }
}

@MainActor
func configured(
    _ style: Style, variant: [(key: String, value: String)] = [], sizes: [SizeChoice] = [medium],
    states: [ButtonState] = allStates, configure: @escaping (inout UIButton.Configuration) -> Void = { _ in },
    adjust: @escaping @MainActor (UIButton) -> Void = { _ in }
) -> Specimen {
    let suffix = variant.map { "\($0.key)_\($0.value)" }.joined(separator: "+")
    return Specimen(
        name: suffix.isEmpty ? style.name : "\(style.name)+\(suffix)", kind: "configuration", style: style.name,
        variant: variant, sizes: sizes, states: states,
        make: { size in
            var configuration = style.make()
            configuration.title = "Button"
            configuration.buttonSize = size
            configure(&configuration)
            let button = UIButton(configuration: configuration)
            adjust(button)
            return Sample(view: button, control: button, button: button)
        })
}

@MainActor
func typed(_ name: String, _ type: UIButton.ButtonType, title: String?) -> Specimen {
    Specimen(
        name: name, kind: "button_type", style: name, variant: [], sizes: [noSize], states: allStates,
        make: { _ in
            let button = UIButton(type: type)
            if let title { button.setTitle(title, for: .normal) }
            return Sample(view: button, control: button, button: button)
        })
}

@MainActor
func barItem(_ name: String, variant: [(key: String, value: String)] = [], _ make: @escaping @MainActor () -> UIBarButtonItem)
    -> Specimen
{
    Specimen(
        name: name, kind: "bar_button_item", style: name, variant: variant, sizes: [noSize],
        states: [.normal, .disabled, .toggleOff, .selected],
        make: { _ in
            let item = make()
            let toolbar = UIToolbar(frame: CGRect(x: 0, y: 0, width: 200, height: 44))
            toolbar.items = [.flexibleSpace(), item, .flexibleSpace()]
            return Sample(view: toolbar, barItem: item, spansWidth: true)
        })
}

@MainActor
func backButton(_ name: String, mode: UINavigationItem.BackButtonDisplayMode, modeName: String) -> Specimen {
    Specimen(
        name: name, kind: "navigation_bar", style: "back", variant: [("back_button_display_mode", modeName)],
        sizes: [noSize], states: [.normal],
        make: { _ in
            let bar = UINavigationBar(frame: CGRect(x: 0, y: 0, width: 200, height: 44))
            let previous = UINavigationItem(title: "Previous")
            let top = UINavigationItem(title: "Title")
            top.backButtonDisplayMode = mode
            bar.setItems([previous, top], animated: false)
            return Sample(view: bar, spansWidth: true)
        })
}

/// The view a paste control pastes into; UIPasteControl enables itself only
/// for a target that accepts the pasteboard's content.
@MainActor
let pasteTarget: UITextView = {
    let view = UITextView(frame: CGRect(x: 0, y: 0, width: 1, height: 1))
    view.pasteConfiguration = UIPasteConfiguration(forAccepting: NSString.self)
    view.backgroundColor = .clear
    view.alpha = 0
    return view
}()

@MainActor
func pasteControl(_ name: String, mode: UIPasteControl.DisplayMode?, modeName: String) -> Specimen {
    Specimen(
        name: name, kind: "paste_control", style: "paste_control",
        variant: mode == nil ? [] : [("display_mode", modeName)], sizes: [noSize],
        states: [.normal, .highlighted, .disabled, .focused],
        make: { _ in
            let configuration = UIPasteControl.Configuration()
            if let mode { configuration.displayMode = mode }
            let control = UIPasteControl(configuration: configuration)
            control.target = pasteTarget
            return Sample(view: control, control: control)
        })
}

@MainActor
func specimens(quick: Bool) -> [Specimen] {
    let styles = configurationStyles()
    var list = styles.map { configured($0, sizes: sizes) }
    list += [
        typed("system", .system, title: "Button"),
        typed("info_light", .infoLight, title: nil),
        typed("info_dark", .infoDark, title: nil),
        typed("detail_disclosure", .detailDisclosure, title: nil),
        typed("contact_add", .contactAdd, title: nil),
        typed("close", .close, title: nil),
    ]
    let roles: [(String, UIButton.Role)] = [("destructive", .destructive), ("cancel", .cancel), ("primary", .primary)]
    for style in styles {
        for (name, role) in roles {
            list.append(configured(style, variant: [("role", name)], adjust: { $0.role = role }))
        }
    }
    let corners: [(String, UIButton.Configuration.CornerStyle)] = [
        ("fixed", .fixed), ("dynamic", .dynamic), ("small", .small), ("medium", .medium), ("large", .large),
        ("capsule", .capsule),
    ]
    for style in styles where ["filled", "gray", "glass"].contains(style.name) {
        for (name, corner) in corners {
            list.append(configured(style, variant: [("corner_style", name)], configure: { $0.cornerStyle = corner }))
        }
    }
    for style in styles {
        list.append(
            configured(style, variant: [("image", "star")], configure: { $0.image = UIImage(systemName: "star") }))
        list.append(
            configured(
                style, variant: [("image", "star"), ("title", "none")],
                configure: {
                    $0.image = UIImage(systemName: "star")
                    $0.title = nil
                }))
        list.append(configured(style, variant: [("subtitle", "Subtitle")], configure: { $0.subtitle = "Subtitle" }))
        list.append(
            configured(style, variant: [("activity_indicator", "shown")], configure: { $0.showsActivityIndicator = true }))
    }
    let placements: [(String, NSDirectionalRectEdge)] = [("trailing", .trailing), ("top", .top), ("bottom", .bottom)]
    for style in styles where style.name == "filled" {
        for (name, placement) in placements {
            list.append(
                configured(
                    style, variant: [("image", "star"), ("image_placement", name)],
                    configure: {
                        $0.image = UIImage(systemName: "star")
                        $0.imagePlacement = placement
                    }))
        }
    }
    let menu = UIMenu(children: [UIAction(title: "Item") { _ in }])
    let indicators: [(String, UIButton.Configuration.Indicator)] = [("popup", .popup), ("none", .none)]
    for style in styles where ["filled", "gray", "plain", "glass"].contains(style.name) {
        list.append(
            configured(
                style, variant: [("menu", "primary_action")], states: untoggledStates,
                adjust: {
                    $0.menu = menu
                    $0.showsMenuAsPrimaryAction = true
                }))
        for (name, indicator) in indicators {
            list.append(
                configured(
                    style, variant: [("menu", "primary_action"), ("indicator", name)], states: untoggledStates,
                    configure: { $0.indicator = indicator },
                    adjust: {
                        $0.menu = menu
                        $0.showsMenuAsPrimaryAction = true
                    }))
        }
        list.append(
            configured(style, variant: [("menu", "on_hold")], states: untoggledStates, adjust: { $0.menu = menu }))
    }
    list += [
        barItem("bar_plain") { UIBarButtonItem(primaryAction: UIAction(title: "Button") { _ in }) },
        barItem("bar_done", variant: [("style", "done")]) {
            let item = UIBarButtonItem(primaryAction: UIAction(title: "Button") { _ in })
            item.style = .done
            return item
        },
        barItem("bar_system_done") { UIBarButtonItem(systemItem: .done, primaryAction: UIAction { _ in }) },
        barItem("bar_system_cancel") { UIBarButtonItem(systemItem: .cancel, primaryAction: UIAction { _ in }) },
        barItem("bar_system_close") { UIBarButtonItem(systemItem: .close, primaryAction: UIAction { _ in }) },
        barItem("bar_system_action") { UIBarButtonItem(systemItem: .action, primaryAction: UIAction { _ in }) },
        barItem("bar_system_add") { UIBarButtonItem(systemItem: .add, primaryAction: UIAction { _ in }) },
        barItem("bar_image") { UIBarButtonItem(primaryAction: UIAction(image: UIImage(systemName: "star")) { _ in }) },
    ]
    #if compiler(>=6.2)
    if #available(iOS 26.0, *) {
        list += [
            barItem("bar_prominent", variant: [("style", "prominent")]) {
                let item = UIBarButtonItem(primaryAction: UIAction(title: "Button") { _ in })
                item.style = .prominent
                return item
            },
            barItem("bar_hides_shared_background", variant: [("hides_shared_background", "true")]) {
                let item = UIBarButtonItem(primaryAction: UIAction(title: "Button") { _ in })
                item.hidesSharedBackground = true
                return item
            },
        ]
    }
    #endif
    list += [
        backButton("back_default", mode: .default, modeName: "default"),
        backButton("back_generic", mode: .generic, modeName: "generic"),
        backButton("back_minimal", mode: .minimal, modeName: "minimal"),
        pasteControl("paste_control", mode: nil, modeName: "default"),
        pasteControl("paste_control+icon_and_label", mode: .iconAndLabel, modeName: "icon_and_label"),
        pasteControl("paste_control+icon_only", mode: .iconOnly, modeName: "icon_only"),
        pasteControl("paste_control+label_only", mode: .labelOnly, modeName: "label_only"),
        Specimen(
            name: "route_picker", kind: "route_picker", style: "route_picker", variant: [], sizes: [noSize],
            states: [.normal],
            make: { _ in
                let picker = AVRoutePickerView()
                let intrinsic = picker.intrinsicContentSize
                let sized = intrinsic.width <= 0 || intrinsic.height <= 0
                return Sample(view: picker, harnessSize: sized ? CGSize(width: 44, height: 44) : nil)
            }),
    ]
    guard quick else { return list }
    let quickNames: Set<String> = [
        "filled", "gray", "glass", "clear_glass", "system", "close", "filled+role_destructive",
        "filled+menu_primary_action", "filled+subtitle_Subtitle", "filled+activity_indicator_shown",
        "bar_plain", "bar_hides_shared_background", "back_default", "paste_control", "route_picker",
    ]
    return list.filter { quickNames.contains($0.name) }.map { specimen in
        Specimen(
            name: specimen.name, kind: specimen.kind, style: specimen.style, variant: specimen.variant,
            sizes: specimen.sizes.filter { $0.name == "medium" || $0.name == "default" }, states: specimen.states,
            make: specimen.make)
    }
}

struct Page {
    let appearance: Appearance
    let specimen: Specimen
    let size: SizeChoice
    let states: [ButtonState]
    let backdrops: [Backdrop]

    var path: String { "\(appearance.name)/\(specimen.name)/\(size.name)" }

    /// The page's screenshot, named for the page and its backdrops.
    var screenshot: String { "pages/\(path)/\(backdrops.first?.fill == .pattern ? "pattern" : "solid").png" }
}

final class Cell {
    let state: ButtonState
    let backdrop: Backdrop
    let view: UIView
    let sample: Sample?
    let applied: Bool

    init(state: ButtonState, backdrop: Backdrop, view: UIView, sample: Sample?, applied: Bool) {
        self.state = state
        self.backdrop = backdrop
        self.view = view
        self.sample = sample
        self.applied = applied
    }

    var name: String { "\(backdrop.name)/\(state.rawValue)" }
}

/// The known pattern behind glass: squares of `patternSquare` points, the
/// colours of `patternColors` in a 2 x 2 tile, anchored at the cell's corner.
final class PatternView: UIView {
    override func draw(_ rect: CGRect) {
        guard let context = UIGraphicsGetCurrentContext() else { return }
        let columns = Int((bounds.width / patternSquare).rounded(.up))
        let rows = Int((bounds.height / patternSquare).rounded(.up))
        for row in 0..<rows {
            for column in 0..<columns {
                context.setFillColor(patternColors[(column % 2) + 2 * (row % 2)].color.cgColor)
                context.fill(
                    CGRect(
                        x: CGFloat(column) * patternSquare, y: CGFloat(row) * patternSquare, width: patternSquare,
                        height: patternSquare))
            }
        }
    }
}

func quoted(_ text: String) -> String {
    var escaped = ""
    for scalar in text.unicodeScalars {
        switch scalar {
        case "\\": escaped += "\\\\"
        case "\"": escaped += "\\\""
        case "\n": escaped += "\\n"
        case "\t": escaped += "\\t"
        default:
            if scalar.value < 0x20 || scalar.value == 0x7f {
                escaped += String(format: "\\u%04x", scalar.value)
            } else {
                escaped.unicodeScalars.append(scalar)
            }
        }
    }
    return "\"\(escaped)\""
}

func number(_ value: CGFloat) -> String { String(format: "%.3f", Double(value)) }

func precise(_ value: Double) -> String { String(format: "%.6f", value) }

func rectangle(_ rect: CGRect) -> String {
    "[\(number(rect.minX)), \(number(rect.minY)), \(number(rect.width)), \(number(rect.height))]"
}

func pair(_ size: CGSize) -> String { "[\(number(size.width)), \(number(size.height))]" }

func insets(_ insets: NSDirectionalEdgeInsets) -> String {
    "[\(number(insets.top)), \(number(insets.leading)), \(number(insets.bottom)), \(number(insets.trailing))]"
}

func edgeInsets(_ insets: UIEdgeInsets) -> String {
    "[\(number(insets.top)), \(number(insets.left)), \(number(insets.bottom)), \(number(insets.right))]"
}

let absent = quoted("absent")

let extendedSRGB = CGColorSpace(name: CGColorSpace.extendedSRGB)

/// A colour as extended sRGB components (0...1 inside sRGB), its 8-bit hex
/// where it is inside sRGB, and the colour space it was stated in.
func colorValue(_ color: CGColor?) -> String {
    guard let color else { return absent }
    let source = (color.colorSpace?.name as String?) ?? "none"
    guard let space = extendedSRGB,
        let converted = color.converted(to: space, intent: .defaultIntent, options: nil),
        let components = converted.components, components.count == 4
    else { return "{ space = \(quoted(source)), srgb = \(quoted("unconvertible")) }" }
    let values = components.map { String(format: "%.4f", Double($0)) }.joined(separator: ", ")
    var hex = ""
    // A conversion can land a hair outside 0...1 (white as 1.0000001).
    if components.allSatisfy({ $0 >= -0.0005 && $0 <= 1.0005 }) {
        hex = ", hex = " + quoted(
            "#" + components.map { String(format: "%02x", Int((min(max($0, 0), 1) * 255).rounded())) }.joined())
    }
    return "{ srgb = [\(values)]\(hex), space = \(quoted(source)) }"
}

func colorValue(_ color: UIColor?, _ traits: UITraitCollection) -> String {
    guard let color else { return absent }
    return colorValue(color.resolvedColor(with: traits).cgColor)
}

func fontValue(_ font: UIFont?) -> String {
    guard let font else { return absent }
    let descriptor = font.fontDescriptor
    let traits = descriptor.object(forKey: .traits) as? [UIFontDescriptor.TraitKey: Any]
    func trait(_ key: UIFontDescriptor.TraitKey) -> String {
        guard let value = traits?[key] as? NSNumber else { return absent }
        return String(format: "%.4f", value.doubleValue)
    }
    let textStyle = (descriptor.object(forKey: .textStyle) as? String).map(quoted) ?? absent
    return "{ name = \(quoted(font.fontName)), family = \(quoted(font.familyName)), size_pt = \(number(font.pointSize)), "
        + "weight = \(trait(.weight)), width = \(trait(.width)), slant = \(trait(.slant)), "
        + "symbolic_traits = \(descriptor.symbolicTraits.rawValue), text_style = \(textStyle), "
        + "line_height_pt = \(number(font.lineHeight)), ascender_pt = \(number(font.ascender)), "
        + "descender_pt = \(number(font.descender)), leading_pt = \(number(font.leading)), "
        + "cap_height_pt = \(number(font.capHeight)), x_height_pt = \(number(font.xHeight)) }"
}

func cornerStyleName(_ style: UIButton.Configuration.CornerStyle) -> String {
    switch style {
    case .fixed: return "fixed"
    case .dynamic: return "dynamic"
    case .small: return "small"
    case .medium: return "medium"
    case .large: return "large"
    case .capsule: return "capsule"
    @unknown default: return "unknown"
    }
}

func sizeName(_ size: UIButton.Configuration.Size) -> String {
    switch size {
    case .mini: return "mini"
    case .small: return "small"
    case .medium: return "medium"
    case .large: return "large"
    @unknown default: return "unknown"
    }
}

func indicatorName(_ indicator: UIButton.Configuration.Indicator) -> String {
    switch indicator {
    case .automatic: return "automatic"
    case .none: return "none"
    case .popup: return "popup"
    @unknown default: return "unknown"
    }
}

func titleAlignmentName(_ alignment: UIButton.Configuration.TitleAlignment) -> String {
    switch alignment {
    case .automatic: return "automatic"
    case .leading: return "leading"
    case .center: return "center"
    case .trailing: return "trailing"
    @unknown default: return "unknown"
    }
}

func edgeName(_ edge: NSDirectionalRectEdge) -> String {
    let names: [(NSDirectionalRectEdge, String)] = [
        (.leading, "leading"), (.trailing, "trailing"), (.top, "top"), (.bottom, "bottom"), (.all, "all"),
        (NSDirectionalRectEdge(), "none"),
    ]
    return names.first { $0.0 == edge }?.1 ?? "raw \(edge.rawValue)"
}

/// A description with its object addresses removed, so two runs write the same.
func described(_ value: Any) -> String {
    quoted(String(describing: value).replacingOccurrences(of: "0x[0-9a-fA-F]+", with: "0x", options: .regularExpression))
}

let textStyles: [(name: String, style: UIFont.TextStyle)] = [
    ("large_title", .largeTitle), ("title1", .title1), ("title2", .title2), ("title3", .title3),
    ("headline", .headline), ("subheadline", .subheadline), ("body", .body), ("callout", .callout),
    ("footnote", .footnote), ("caption1", .caption1), ("caption2", .caption2),
]

let categories: [(name: String, category: UIContentSizeCategory)] = [
    ("extra_small", .extraSmall), ("small", .small), ("medium", .medium), ("large", .large),
    ("extra_large", .extraLarge), ("extra_extra_large", .extraExtraLarge),
    ("extra_extra_extra_large", .extraExtraExtraLarge), ("accessibility_medium", .accessibilityMedium),
    ("accessibility_large", .accessibilityLarge), ("accessibility_extra_large", .accessibilityExtraLarge),
    ("accessibility_extra_extra_large", .accessibilityExtraExtraLarge),
    ("accessibility_extra_extra_extra_large", .accessibilityExtraExtraExtraLarge),
]

let symbolScales: [(name: String, scale: UIImage.SymbolScale)] = [
    ("default", .default), ("unspecified", .unspecified), ("small", .small), ("medium", .medium), ("large", .large),
]

let symbolNames = ["star", "circle", "square.and.arrow.up", "chevron.backward"]

final class HostViewController: UIViewController {
    private let pressMode: Bool
    private let quick: Bool
    private let colorNames: [String]
    private let pages: [Page]
    private let run: String
    private let out: URL
    private var pageIndex = 0
    private var cells: [Cell] = []
    private var code: [UIView] = []
    private var cellsBottom: CGFloat = 0
    private var sequence = 0
    private var attempt = 0
    private var retakes: [String] = []
    private var reads: [String] = []
    private var entries: [String] = []
    private var failures: [String] = []
    private var started = false
    private var focusTarget: UIView?
    private var doneLabel: UILabel?
    private var finished = false
    private var screenWritten = false
    private var overlayRetakes = 0
    private var settleShots = 0
    private var previousCrops: [[UInt8]?] = []

    // The press mode's state: the cells left to press on this page, the armed
    // cell, its identifier's number, and whether its touch is down.
    private var pressQueue: [Cell] = []
    private var armed: Cell?
    private var pressNumber = 0
    private var armedNumber = 0
    private var shots = 0
    private var touchDown = false
    private var captured = false
    private var rearm = false
    private var pressRetakes = 0
    private var aloneWritten = false
    private var highlightedAtCapture = false
    private var trackingAtCapture = false

    init() {
        let arguments = Array(CommandLine.arguments.dropFirst())
        let argument = arguments.first ?? "run"
        let runName = argument.allSatisfy { $0.isLetter || $0.isNumber } ? argument : "run"
        let press = runName.hasPrefix("press")
        let quickRun = arguments.contains("quick")
        run = runName
        pressMode = press
        quick = quickRun
        colorNames = arguments.first { $0.hasPrefix("colors=") }.map {
            $0.dropFirst("colors=".count).split(separator: ",").map(String.init)
        } ?? []
        let documents = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first
            ?? URL(fileURLWithPath: NSTemporaryDirectory())
        out = documents.appendingPathComponent(runName, isDirectory: true)
        var pages: [Page] = []
        let list = specimens(quick: quickRun)
        for appearance in appearances {
            for specimen in list {
                if press {
                    guard specimen.kind == "configuration" && specimen.variant.isEmpty || specimen.name == "system"
                    else { continue }
                    if glassStyles.contains(specimen.name) {
                        for size in specimen.sizes {
                            pages.append(
                                Page(
                                    appearance: appearance, specimen: specimen, size: size,
                                    states: [.pressed, .pressedSelected], backdrops: solveBackdrops))
                            pages.append(
                                Page(
                                    appearance: appearance, specimen: specimen, size: size, states: [.pressed],
                                    backdrops: patternBackdrops))
                        }
                    } else if let size = specimen.sizes.first(where: { $0.name == "medium" || $0.name == "default" }) {
                        if quickRun && specimen.name != "filled" { continue }
                        pages.append(
                            Page(
                                appearance: appearance, specimen: specimen, size: size,
                                states: [.pressed, .pressedSelected], backdrops: [solveBackdrops[0]]))
                    }
                } else {
                    for size in specimen.sizes {
                        pages.append(
                            Page(
                                appearance: appearance, specimen: specimen, size: size, states: specimen.states,
                                backdrops: solveBackdrops))
                        if specimen.kind == "configuration" && specimen.variant.isEmpty
                            && glassStyles.contains(specimen.name)
                        {
                            pages.append(
                                Page(
                                    appearance: appearance, specimen: specimen, size: size, states: specimen.states,
                                    backdrops: patternBackdrops))
                        }
                    }
                }
            }
        }
        self.pages = pages
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) { nil }

    override var prefersStatusBarHidden: Bool { true }

    override var preferredFocusEnvironments: [any UIFocusEnvironment] {
        if let focusTarget { return [focusTarget] }
        return super.preferredFocusEnvironments
    }

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .systemBackground
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        guard !started else { return }
        started = true
        let manager = FileManager.default
        try? manager.removeItem(at: out)
        for directory in ["sync", "draw", "screen", "pages"] {
            do {
                try manager.createDirectory(
                    at: out.appendingPathComponent(directory, isDirectory: true), withIntermediateDirectories: true)
            } catch {
                failures.append("create \(directory): \(error)")
            }
        }
        if !pressMode {
            readMotion()
            UIView.setAnimationsEnabled(false)
            UIPasteboard.general.string = "Text"
            view.addSubview(pasteTarget)
            readColors()
            readTextStyles()
            readSymbols()
            readCaret()
            readBehavior()
        }
        guard !pages.isEmpty else {
            failures.append("no pages")
            finish()
            return
        }
        show(page: 0)
    }

    // MARK: the pages

    private func show(page index: Int) {
        guard !finished else { return }
        pageIndex = index
        let page = pages[index]
        for cell in cells { cell.view.removeFromSuperview() }
        for square in code { square.removeFromSuperview() }
        cells = []
        code = []
        focusTarget = nil
        overlayRetakes = 0
        settleShots = 0
        previousCrops = []
        view.window?.overrideUserInterfaceStyle = page.appearance.style

        let columns = CGFloat(page.backdrops.count)
        let width = ((view.bounds.width - 2 * margin - (columns - 1) * gap) / columns).rounded(.down)
        var top = view.safeAreaInsets.top + margin
        for state in page.states {
            for (column, backdrop) in page.backdrops.enumerated() {
                let frame = CGRect(
                    x: margin + CGFloat(column) * (width + gap), y: top, width: width, height: cellHeight)
                let cellView: UIView
                switch backdrop.fill {
                case .system:
                    cellView = UIView(frame: frame)
                    cellView.backgroundColor = .systemBackground
                case .black:
                    cellView = UIView(frame: frame)
                    cellView.backgroundColor = UIColor(red: 0, green: 0, blue: 0, alpha: 1)
                case .white:
                    cellView = UIView(frame: frame)
                    cellView.backgroundColor = UIColor(red: 1, green: 1, blue: 1, alpha: 1)
                case .pattern:
                    cellView = PatternView(frame: frame)
                }
                view.addSubview(cellView)
                var sample: Sample?
                var applied = true
                if backdrop.holdsSpecimen {
                    let made = page.specimen.make(page.size.size)
                    place(made, in: cellView)
                    applied = apply(state, to: made)
                    if state == .focused && backdrop.fill == .system { focusTarget = made.view }
                    sample = made
                }
                cells.append(Cell(state: state, backdrop: backdrop, view: cellView, sample: sample, applied: applied))
            }
            top += cellHeight + gap
        }
        cellsBottom = top - gap
        for bit in 0..<codeBits {
            let square = UIView(
                frame: CGRect(
                    x: margin + CGFloat(bit) * codeSquare, y: cellsBottom + codeSquare, width: codeSquare,
                    height: codeSquare))
            square.backgroundColor =
                (index >> bit) & 1 == 1
                ? UIColor(red: 1, green: 0, blue: 0, alpha: 1) : UIColor(red: 0, green: 0, blue: 1, alpha: 1)
            view.addSubview(square)
            code.append(square)
        }
        view.layoutIfNeeded()
        if focusTarget != nil {
            setNeedsFocusUpdate()
            updateFocusIfNeeded()
        }
        DispatchQueue.main.asyncAfter(deadline: .now() + settleSeconds) { [weak self] in
            guard let self, !self.finished else { return }
            if self.pressMode {
                self.pressQueue = self.cells.filter { $0.sample?.button != nil }
                self.aloneWritten = false
                self.armNext()
            } else {
                self.draw()
            }
        }
    }

    private func place(_ sample: Sample, in cellView: UIView) {
        let view = sample.view
        view.translatesAutoresizingMaskIntoConstraints = false
        cellView.addSubview(view)
        var constraints = [view.centerYAnchor.constraint(equalTo: cellView.centerYAnchor)]
        if sample.spansWidth {
            constraints += [
                view.leadingAnchor.constraint(equalTo: cellView.leadingAnchor, constant: barInset),
                view.trailingAnchor.constraint(equalTo: cellView.trailingAnchor, constant: -barInset),
            ]
        } else {
            constraints.append(view.centerXAnchor.constraint(equalTo: cellView.centerXAnchor))
        }
        if let size = sample.harnessSize {
            constraints += [
                view.widthAnchor.constraint(equalToConstant: size.width),
                view.heightAnchor.constraint(equalToConstant: size.height),
            ]
        }
        NSLayoutConstraint.activate(constraints)
    }

    /// Sets a state on a sample; false where the sample cannot take it.
    private func apply(_ state: ButtonState, to sample: Sample) -> Bool {
        if let item = sample.barItem {
            switch state {
            case .normal: return true
            case .disabled: item.isEnabled = false
            case .toggleOff: item.changesSelectionAsPrimaryAction = true
            case .selected:
                item.changesSelectionAsPrimaryAction = true
                item.isSelected = true
            default: return false
            }
            return true
        }
        guard let control = sample.control else { return state == .normal }
        switch state {
        case .normal, .focused, .pressed: return true
        case .highlighted: control.isHighlighted = true
        case .disabled: control.isEnabled = false
        case .toggleOff:
            guard let button = sample.button else { return false }
            button.changesSelectionAsPrimaryAction = true
        case .selected, .pressedSelected:
            guard let button = sample.button else { return false }
            button.changesSelectionAsPrimaryAction = true
            button.isSelected = true
        case .selectedHighlighted:
            guard let button = sample.button else { return false }
            button.changesSelectionAsPrimaryAction = true
            button.isSelected = true
            button.isHighlighted = true
        }
        return true
    }

    private func drawCell(_ cell: Cell, to name: String) -> CGImage? {
        let scale = view.window?.windowScene?.screen.scale ?? 1
        let format = UIGraphicsImageRendererFormat(for: cell.view.traitCollection)
        format.scale = scale
        format.opaque = true
        format.preferredRange = .standard
        let renderer = UIGraphicsImageRenderer(bounds: cell.view.bounds, format: format)
        var succeeded = false
        let image = renderer.image { _ in
            succeeded = cell.view.drawHierarchy(in: cell.view.bounds, afterScreenUpdates: true)
        }
        if !succeeded { failures.append("\(name): drawHierarchy returned false") }
        write(image, to: name)
        return image.cgImage
    }

    private func draw() {
        let page = pages[pageIndex]
        for cell in cells {
            _ = drawCell(cell, to: "draw/\(page.path)/\(cell.name).png")
        }
        requestScreenshot(sequence: pageIndex, attempt: 0)
    }

    // MARK: the screenshots

    private func requestScreenshot(sequence: Int, attempt: Int) {
        self.sequence = sequence
        self.attempt = attempt
        let ready = out.appendingPathComponent("sync/ready-\(sequence)-\(attempt)")
        do {
            try pages[pageIndex].path.write(to: ready, atomically: true, encoding: .utf8)
        } catch {
            failures.append("write \(ready.lastPathComponent): \(error)")
        }
        waitForScreenshot(since: Date())
    }

    private func waitForScreenshot(since start: Date) {
        guard !finished else { return }
        let screenshot = out.appendingPathComponent("sync/screen-\(sequence)-\(attempt).png")
        if FileManager.default.fileExists(atPath: screenshot.path) {
            if pressMode { pressScreenshot(screenshot) } else { crop(screenshot) }
        } else if Date().timeIntervalSince(start) > 300 {
            failures.append("\(pages[pageIndex].path): no screenshot within 300 s")
            finish()
        } else {
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) { [weak self] in
                self?.waitForScreenshot(since: start)
            }
        }
    }

    private var pixelScale: CGFloat { view.window?.windowScene?.screen.scale ?? 1 }

    private func pixels(_ cell: Cell) -> CGRect {
        let scale = pixelScale
        let frame = cell.view.convert(cell.view.bounds, to: nil)
        return CGRect(
            x: (frame.minX * scale).rounded(), y: (frame.minY * scale).rounded(),
            width: (frame.width * scale).rounded(), height: (frame.height * scale).rounded())
    }

    /// Why the screenshot is not of this page as drawn (a banner over its
    /// margins, a stale page code), or nil.
    private func overlay(_ source: CGImage) -> String? {
        guard let bytes = samples(source) else { return "the screenshot does not decode" }
        let scale = pixelScale
        let width = source.width
        let height = source.height
        let background = UIColor.systemBackground.resolvedColor(with: view.traitCollection)
        var red: CGFloat = 0
        var green: CGFloat = 0
        var blue: CGFloat = 0
        var alpha: CGFloat = 0
        _ = background.getRed(&red, green: &green, blue: &blue, alpha: &alpha)
        let expected = [red, green, blue].map { Int(($0 * 255).rounded()) }
        func px(_ points: CGFloat) -> Int { Int((points * scale).rounded()) }
        let top = view.safeAreaInsets.top + 2
        let regions = [
            (px(2), px(margin - 2), px(top), px(cellsBottom)),
            (px(view.bounds.width - margin + 2), px(view.bounds.width - 2), px(top), px(cellsBottom)),
            (px(2), px(view.bounds.width - 2), px(top), px(top + margin - 4)),
        ]
        var differing = 0
        for (x0, x1, y0, y1) in regions {
            for y in max(0, y0)..<min(height, y1) {
                for x in max(0, x0)..<min(width, x1) {
                    let offset = (y * width + x) * 4
                    for channel in 0..<3 where abs(Int(bytes[offset + channel]) - expected[channel]) > overlayLevels {
                        differing += 1
                        break
                    }
                }
            }
        }
        if differing > overlayPixels { return "\(differing) margin pixels differ from the system background" }
        var read = 0
        for bit in 0..<codeBits {
            let x = px(margin + (CGFloat(bit) + 0.5) * codeSquare)
            let y = px(cellsBottom + 1.5 * codeSquare)
            guard x < width, y < height else { return "the page code is outside the screenshot" }
            let offset = (y * width + x) * 4
            let r = Int(bytes[offset])
            let b = Int(bytes[offset + 2])
            if r > 200 && b < 60 {
                read |= 1 << bit
            } else if !(b > 200 && r < 60) {
                return "page code bit \(bit) reads as neither red nor blue"
            }
        }
        if read != pageIndex & ((1 << codeBits) - 1) { return "the page code reads \(read), not \(pageIndex)" }
        return nil
    }

    private func loadScreenshot(_ screenshot: URL) -> CGImage? {
        guard let image = UIImage(contentsOfFile: screenshot.path), let source = image.cgImage else {
            failures.append("\(pages[pageIndex].path): the screenshot does not load")
            return nil
        }
        let native = view.window?.windowScene?.screen.nativeBounds.size ?? .zero
        if CGFloat(source.width) != native.width || CGFloat(source.height) != native.height {
            failures.append("\(pages[pageIndex].path): screenshot \(source.width)x\(source.height), screen \(native)")
        }
        return source
    }

    private func crop(_ screenshot: URL) {
        let page = pages[pageIndex]
        guard let source = loadScreenshot(screenshot) else {
            finish()
            return
        }
        let manager = FileManager.default
        if let reason = overlay(source) {
            if overlayRetakes < maxRetakes {
                overlayRetakes += 1
                try? manager.removeItem(at: screenshot)
                retakes.append("\(page.path) attempt \(attempt): \(reason)")
                DispatchQueue.main.asyncAfter(deadline: .now() + retakeDelaySeconds) { [weak self] in
                    guard let self else { return }
                    self.requestScreenshot(sequence: self.pageIndex, attempt: self.attempt + 1)
                }
                return
            }
            failures.append("\(page.path): \(reason) after \(maxRetakes) retakes")
        }
        if !page.specimen.animated {
            let crops = cells.map { cell in source.cropping(to: pixels(cell)).flatMap { samples($0) } }
            let settled = previousCrops.count == crops.count && zip(previousCrops, crops).allSatisfy { pair in
                guard let before = pair.0, let after = pair.1, before.count == after.count else { return false }
                return zip(before, after).allSatisfy { abs(Int($0.0) - Int($0.1)) <= settledLevels }
            }
            if !settled {
                if settleShots < maxSettleShots {
                    if !previousCrops.isEmpty { retakes.append("\(page.path) attempt \(attempt): not settled") }
                    settleShots += 1
                    previousCrops = crops
                    try? manager.removeItem(at: screenshot)
                    DispatchQueue.main.asyncAfter(deadline: .now() + settleShotSeconds) { [weak self] in
                        guard let self else { return }
                        self.requestScreenshot(sequence: self.pageIndex, attempt: self.attempt + 1)
                    }
                    return
                }
                failures.append("\(page.path): not settled after \(maxSettleShots) screenshots")
            }
        }
        let kept = out.appendingPathComponent(page.screenshot)
        do {
            try manager.createDirectory(at: kept.deletingLastPathComponent(), withIntermediateDirectories: true)
            try manager.moveItem(at: screenshot, to: kept)
        } catch {
            failures.append("\(page.path): keep the screenshot: \(error)")
        }
        for cell in cells {
            let frame = pixels(cell)
            let name = "screen/\(page.path)/\(cell.name).png"
            if let cropped = source.cropping(to: frame) {
                write(UIImage(cgImage: cropped, scale: pixelScale, orientation: .up), to: name)
            } else {
                failures.append("\(name): crop \(frame) outside the screenshot")
            }
            entries.append(
                renderEntry(
                    page: page, cell: cell, draw: "draw/\(page.path)/\(cell.name).png", screen: name,
                    screenshot: page.screenshot, frame: frame))
        }
        if !screenWritten {
            screenWritten = true
            entries.insert(screenEntry(source: source), at: 0)
        }
        if pageIndex + 1 < pages.count {
            show(page: pageIndex + 1)
        } else {
            finish()
        }
    }

    // MARK: the press mode

    private func armNext() {
        guard !finished else { return }
        armed?.sample?.button?.accessibilityIdentifier = nil
        armed = nil
        guard let cell = pressQueue.first, let button = cell.sample?.button else {
            if pageIndex + 1 < pages.count {
                show(page: pageIndex + 1)
            } else {
                finish()
            }
            return
        }
        armed = cell
        touchDown = false
        captured = false
        rearm = false
        button.removeTarget(self, action: nil, for: .allEvents)
        button.addTarget(self, action: #selector(pressBegan(_:)), for: .touchDown)
        button.addTarget(self, action: #selector(pressEnded(_:)), for: [.touchUpInside, .touchUpOutside, .touchCancel])
        armedNumber = pressNumber
        button.accessibilityIdentifier = "press-\(armedNumber)"
        pressNumber += 1
    }

    @objc private func pressBegan(_ sender: UIButton) {
        guard let cell = armed, sender === cell.sample?.button, !touchDown else { return }
        touchDown = true
        DispatchQueue.main.asyncAfter(deadline: .now() + pressSettleSeconds) { [weak self] in
            self?.capturePress()
        }
    }

    @objc private func pressEnded(_ sender: UIButton) {
        guard let cell = armed, sender === cell.sample?.button, touchDown else { return }
        touchDown = false
        if captured {
            if !pressQueue.isEmpty { pressQueue.removeFirst() }
            pressRetakes = 0
            scheduleArm()
        } else if rearm {
            scheduleArm()
        }
    }

    private func scheduleArm() {
        armed?.sample?.button?.accessibilityIdentifier = nil
        DispatchQueue.main.asyncAfter(deadline: .now() + pressRearmSeconds) { [weak self] in self?.armNext() }
    }

    private func capturePress() {
        guard !finished, let cell = armed, let button = cell.sample?.button else { return }
        guard touchDown else {
            retakePress("the touch ended before the capture")
            return
        }
        highlightedAtCapture = button.isHighlighted
        trackingAtCapture = button.isTracking
        _ = drawCell(cell, to: "draw/\(pages[pageIndex].path)/\(cell.name).png")
        requestScreenshot(sequence: shots, attempt: 0)
        shots += 1
    }

    private func retakePress(_ reason: String) {
        guard let cell = armed else { return }
        let page = pages[pageIndex]
        if pressRetakes < maxRetakes {
            pressRetakes += 1
            retakes.append("\(page.path)/\(cell.name) press \(armedNumber): \(reason)")
        } else {
            failures.append("\(page.path)/\(cell.name): \(reason) after \(maxRetakes) retakes")
            if !pressQueue.isEmpty { pressQueue.removeFirst() }
            pressRetakes = 0
        }
        if touchDown {
            rearm = true
        } else {
            scheduleArm()
        }
    }

    private func pressScreenshot(_ screenshot: URL) {
        let page = pages[pageIndex]
        guard let cell = armed, let button = cell.sample?.button else { return }
        let heldThrough = touchDown
        guard let source = loadScreenshot(screenshot) else {
            finish()
            return
        }
        let manager = FileManager.default
        let kept = out.appendingPathComponent("pages/\(sequence).png")
        do {
            try manager.moveItem(at: screenshot, to: kept)
        } catch {
            failures.append("\(page.path): keep the screenshot: \(error)")
        }
        if let reason = overlay(source) {
            retakePress(reason)
            return
        }
        guard heldThrough else {
            retakePress("the touch ended before the screenshot arrived")
            return
        }
        let name = "screen/\(page.path)/\(cell.name).png"
        let frame = pixels(cell)
        if let cropped = source.cropping(to: frame) {
            write(UIImage(cgImage: cropped, scale: pixelScale, orientation: .up), to: name)
        } else {
            failures.append("\(name): crop \(frame) outside the screenshot")
        }
        entries.append(
            renderEntry(
                page: page, cell: cell, draw: "draw/\(page.path)/\(cell.name).png", screen: name,
                screenshot: "pages/\(sequence).png", frame: frame,
                press: "press_number = \(armedNumber)\nhighlighted_at_capture = \(highlightedAtCapture)\n"
                    + "tracking_at_capture = \(trackingAtCapture)\nheld_until_screenshot = \(heldThrough)\n"
                    + "selected_at_screenshot = \(button.isSelected)\n"))
        if !aloneWritten {
            aloneWritten = true
            for alone in cells where !alone.backdrop.holdsSpecimen {
                let aloneName = "screen/\(page.path)/\(alone.name).png"
                let aloneFrame = pixels(alone)
                if let cropped = source.cropping(to: aloneFrame) {
                    write(UIImage(cgImage: cropped, scale: pixelScale, orientation: .up), to: aloneName)
                }
                entries.append(
                    renderEntry(
                        page: page, cell: alone, draw: absent, screen: aloneName,
                        screenshot: "pages/\(sequence).png", frame: aloneFrame))
            }
        }
        if !screenWritten {
            screenWritten = true
            entries.insert(screenEntry(source: source), at: 0)
        }
        captured = true
    }

    // MARK: the index

    private func renderEntry(
        page: Page, cell: Cell, draw: String, screen: String, screenshot: String, frame: CGRect, press: String = ""
    ) -> String {
        let variant = page.specimen.variant.map { "\($0.key) = \(quoted($0.value))" }.joined(separator: ", ")
        var text = """
            [[render]]
            appearance = \(quoted(page.appearance.name))
            specimen = \(quoted(page.specimen.name))
            kind = \(quoted(page.specimen.kind))
            style = \(quoted(page.specimen.style))
            variant = { \(variant) }
            size = \(quoted(page.size.name))
            backdrop = \(quoted(cell.backdrop.name))
            state = \(quoted(cell.state.rawValue))
            state_applied = \(cell.applied)
            draw = \(draw.hasPrefix("\"") ? draw : quoted(draw))
            screen = \(quoted(screen))
            page = \(quoted(screenshot))
            cell_px = \(rectangle(frame))
            cell_pt = \(rectangle(cell.view.frame))

            """
        text += press
        guard let sample = cell.sample else { return text + "\n" }
        let specimen = sample.view
        text += """
            frame_pt = \(rectangle(specimen.frame))
            intrinsic_size_pt = \(pair(specimen.intrinsicContentSize))
            harness_size_pt = \(sample.harnessSize.map(pair) ?? absent)
            user_interface_style = \(specimen.traitCollection.userInterfaceStyle.rawValue)
            is_focused = \(specimen.isFocused)
            can_become_focused = \(specimen.canBecomeFocused)

            """
        if let control = sample.control {
            text += """
                is_highlighted = \(control.isHighlighted)
                is_enabled = \(control.isEnabled)
                is_selected = \(control.isSelected)

                """
        }
        if let item = sample.barItem {
            text += """
                item_is_enabled = \(item.isEnabled)
                item_is_selected = \(item.isSelected)
                item_style = \(item.style.rawValue)

                """
        }
        guard cell.backdrop.fill == .system else { return text + "\n" }
        let traits = specimen.traitCollection
        text += "tint = \(colorValue(specimen.tintColor, traits))\n"
        if let button = sample.button {
            text += """
                title_font = \(fontValue(button.titleLabel?.font))
                title_color = \(colorValue(button.titleLabel?.textColor, traits))
                button_type = \(button.buttonType.rawValue)
                role = \(button.role.rawValue)

                """
            if let configuration = button.configuration {
                text += configurationTable(configuration.updated(for: button), tint: button.tintColor, traits: traits)
            }
        }
        if let picker = specimen as? AVRoutePickerView {
            text += """
                route_tint = \(colorValue(picker.tintColor, traits))
                route_active_tint = \(colorValue(picker.activeTintColor, traits))

                """
        }
        if let paste = specimen as? UIPasteControl {
            let configuration = paste.configuration
            text += """
                [render.paste_configuration]
                display_mode = \(configuration.displayMode.rawValue)
                corner_style = \(quoted(cornerStyleName(configuration.cornerStyle)))
                corner_radius_pt = \(number(configuration.cornerRadius))
                base_foreground_color = \(colorValue(configuration.baseForegroundColor, traits))
                base_background_color = \(colorValue(configuration.baseBackgroundColor, traits))

                """
        }
        text += "\n" + viewTree(specimen).joined()
        return text
    }

    private func configurationTable(
        _ configuration: UIButton.Configuration, tint: UIColor, traits: UITraitCollection
    ) -> String {
        let background = configuration.background
        let resolvedTint = tint.resolvedColor(with: traits)
        var title = absent
        if let transformer = configuration.titleTextAttributesTransformer {
            let attributes = transformer(AttributeContainer())
            title = "{ font = \(fontValue(attributes[AttributeScopes.UIKitAttributes.FontAttribute.self])), "
                + "foreground = \(colorValue(attributes[AttributeScopes.UIKitAttributes.ForegroundColorAttribute.self], traits)) }"
        }
        var subtitle = absent
        if let transformer = configuration.subtitleTextAttributesTransformer {
            let attributes = transformer(AttributeContainer())
            subtitle = "{ font = \(fontValue(attributes[AttributeScopes.UIKitAttributes.FontAttribute.self])), "
                + "foreground = \(colorValue(attributes[AttributeScopes.UIKitAttributes.ForegroundColorAttribute.self], traits)) }"
        }
        let symbol = configuration.preferredSymbolConfigurationForImage.map { described($0) } ?? absent
        return """
            [render.configuration]
            source = "UIButton.Configuration.updated(for:) of the button in this state"
            button_size = \(quoted(sizeName(configuration.buttonSize)))
            corner_style = \(quoted(cornerStyleName(configuration.cornerStyle)))
            content_insets_pt = \(insets(configuration.contentInsets))
            image_padding_pt = \(number(configuration.imagePadding))
            title_padding_pt = \(number(configuration.titlePadding))
            image_placement = \(quoted(edgeName(configuration.imagePlacement)))
            title_alignment = \(quoted(titleAlignmentName(configuration.titleAlignment)))
            indicator = \(quoted(indicatorName(configuration.indicator)))
            shows_activity_indicator = \(configuration.showsActivityIndicator)
            automatically_update_for_selection = \(configuration.automaticallyUpdateForSelection)
            base_foreground_color = \(colorValue(configuration.baseForegroundColor, traits))
            base_background_color = \(colorValue(configuration.baseBackgroundColor, traits))
            title_text_attributes = \(title)
            subtitle_text_attributes = \(subtitle)
            preferred_symbol_configuration = \(symbol)
            background_color = \(colorValue(background.backgroundColor, traits))
            resolved_background_color = \(colorValue(background.resolvedBackgroundColor(for: resolvedTint), traits))
            stroke_color = \(colorValue(background.strokeColor, traits))
            resolved_stroke_color = \(colorValue(background.resolvedStrokeColor(for: resolvedTint), traits))
            stroke_width_pt = \(number(background.strokeWidth))
            stroke_outset_pt = \(number(background.strokeOutset))
            background_corner_radius_pt = \(number(background.cornerRadius))
            background_insets_pt = \(insets(background.backgroundInsets))
            visual_effect = \(background.visualEffect.map { quoted(String(describing: type(of: $0))) } ?? absent)
            resolved_against_tint = \(colorValue(resolvedTint.cgColor))

            """
    }

    private func filterName(_ filter: Any?) -> String {
        if let name = filter as? String { return quoted(name) }
        guard let filter else { return absent }
        return quoted(String(describing: type(of: filter)))
    }

    /// The specimen's views, depth first, at most 80 of them and 8 deep: what
    /// UIKit draws the specimen with.
    private func viewTree(_ root: UIView) -> [String] {
        var lines: [String] = []
        var count = 0
        func visit(_ view: UIView, path: String, depth: Int) {
            guard count < 80, depth <= 8 else { return }
            count += 1
            let layer = view.layer
            let traits = view.traitCollection
            var fields = [
                "[[render.view]]",
                "path = \(quoted(path))",
                "class = \(quoted(String(describing: type(of: view))))",
                "frame_pt = \(rectangle(view.convert(view.bounds, to: root)))",
                "hidden = \(view.isHidden)",
                "alpha = \(number(view.alpha))",
                "clips = \(view.clipsToBounds)",
                "background = \(colorValue(view.backgroundColor, traits))",
                "tint = \(colorValue(view.tintColor, traits))",
                "layer_class = \(quoted(String(describing: type(of: layer))))",
                "layer_opacity = \(number(CGFloat(layer.opacity)))",
                "layer_background = \(colorValue(layer.backgroundColor))",
                "corner_radius_pt = \(number(layer.cornerRadius))",
                "corner_curve = \(quoted(layer.cornerCurve.rawValue))",
                "masked_corners = \(layer.maskedCorners.rawValue)",
                "border_width_pt = \(number(layer.borderWidth))",
                "border_color = \(colorValue(layer.borderColor))",
                "shadow_opacity = \(number(CGFloat(layer.shadowOpacity)))",
                "shadow_radius_pt = \(number(layer.shadowRadius))",
                "shadow_offset_pt = \(pair(layer.shadowOffset))",
                "shadow_color = \(colorValue(layer.shadowColor))",
                "has_mask = \(layer.mask != nil || view.mask != nil)",
                "compositing_filter = \(filterName(layer.compositingFilter))",
                "filters = [\((layer.filters ?? []).map { quoted(String(describing: type(of: $0))) }.joined(separator: ", "))]",
            ]
            if let label = view as? UILabel {
                fields += [
                    "text = \(label.text.map(quoted) ?? absent)",
                    "font = \(fontValue(label.font))",
                    "text_color = \(colorValue(label.textColor, traits))",
                ]
            }
            if let imageView = view as? UIImageView {
                fields += [
                    "image_size_pt = \(imageView.image.map { pair($0.size) } ?? absent)",
                    "image_is_symbol = \(imageView.image?.isSymbolImage ?? false)",
                    "image_alignment_insets_pt = \(imageView.image.map { edgeInsets($0.alignmentRectInsets) } ?? absent)",
                    "image_symbol_configuration = \(imageView.preferredSymbolConfiguration.map { described($0) } ?? absent)",
                ]
            }
            if let effectView = view as? UIVisualEffectView {
                fields.append(
                    "effect = \(effectView.effect.map { quoted(String(describing: type(of: $0))) } ?? absent)")
            }
            if let spinner = view as? UIActivityIndicatorView {
                fields += [
                    "spinner_style = \(spinner.style.rawValue)",
                    "spinner_color = \(colorValue(spinner.color, traits))",
                    "spinner_animating = \(spinner.isAnimating)",
                ]
            }
            lines.append(fields.joined(separator: "\n") + "\n\n")
            for (index, child) in view.subviews.enumerated() {
                visit(child, path: "\(path).\(index)", depth: depth + 1)
            }
        }
        visit(root, path: "0", depth: 0)
        return lines
    }

    private func screenEntry(source: CGImage) -> String {
        let screen = view.window?.windowScene?.screen
        let traits = view.traitCollection
        let colorSpace = (source.colorSpace?.name as String?) ?? "none"
        let pattern = patternColors.map { quoted($0.name) }.joined(separator: ", ")
        return """
            [screen]
            scale = \(number(screen?.scale ?? 0))
            bounds_pt = \(rectangle(screen?.bounds ?? .zero))
            native_bounds_px = \(rectangle(screen?.nativeBounds ?? .zero))
            safe_area_top_pt = \(number(view.safeAreaInsets.top))
            safe_area_bottom_pt = \(number(view.safeAreaInsets.bottom))
            screenshot_color_space = \(quoted(colorSpace))
            display_gamut = \(traits.displayGamut.rawValue)
            preferred_content_size_category = \(quoted(traits.preferredContentSizeCategory.rawValue))
            focus_system = \(UIFocusSystem.focusSystem(for: self) != nil)
            interface_idiom = \(traits.userInterfaceIdiom.rawValue)

            [layout]
            margin_pt = \(number(margin))
            cell_height_pt = \(number(cellHeight))
            gap_pt = \(number(gap))
            bar_inset_pt = \(number(barInset))
            backdrops = { system = "UIColor.systemBackground", black = "#000000", white = "#ffffff", pattern = "squares of pattern.square_pt, pattern.colors in a 2 x 2 tile (column % 2 + 2 * (row % 2)), anchored at the cell's top left", pattern_alone = "the pattern, no specimen" }
            page_code = "\(codeBits) squares of \(number(codeSquare)) pt under the grid, red #ff0000 for a set bit, blue #0000ff for a clear one, the page's number from bit 0 at the left"

            [pattern]
            square_pt = \(number(patternSquare))
            colors = [\(pattern)]

            """
    }

    private func write(_ image: UIImage, to name: String) {
        let url = out.appendingPathComponent(name)
        do {
            try FileManager.default.createDirectory(
                at: url.deletingLastPathComponent(), withIntermediateDirectories: true)
            guard let data = image.pngData() else {
                failures.append("\(name): no PNG data")
                return
            }
            try data.write(to: url)
        } catch {
            failures.append("\(name): \(error)")
        }
    }

    // MARK: UIKit's values

    private func readMotion() {
        var springDuration: TimeInterval = -1
        UIView.animate(bounce: 0, animations: { springDuration = UIView.inheritedAnimationDuration })
        let longPress = UILongPressGestureRecognizer()
        reads.append(
            """
            [motion]
            ca_transaction_animation_duration_s = \(precise(CATransaction.animationDuration()))
            spring_animation_default_duration_s = \(precise(springDuration))
            spring_animation_default_source = "UIView.inheritedAnimationDuration inside UIView.animate(springDuration:bounce:...) called with its default duration"
            navigation_bar_hide_show_duration_s = \(precise(Double(UINavigationController.hideShowBarDuration)))
            scroll_deceleration_rate_normal = \(precise(Double(UIScrollView.DecelerationRate.normal.rawValue)))
            scroll_deceleration_rate_fast = \(precise(Double(UIScrollView.DecelerationRate.fast.rawValue)))
            scroll_view_default_deceleration_rate = \(precise(Double(UIScrollView().decelerationRate.rawValue)))
            long_press_minimum_duration_s = \(precise(longPress.minimumPressDuration))
            long_press_allowable_movement_pt = \(precise(Double(longPress.allowableMovement)))
            maximum_frames_per_second = \(view.window?.windowScene?.screen.maximumFramesPerSecond ?? 0)
            reduce_motion = \(UIAccessibility.isReduceMotionEnabled)
            darker_system_colors = \(UIAccessibility.isDarkerSystemColorsEnabled)
            bold_text = \(UIAccessibility.isBoldTextEnabled)
            button_shapes = \(UIAccessibility.buttonShapesEnabled)
            unexposed = [
              "the key repeat delay and interval: UIKit has no API that reports them",
              "the double-tap interval: UITapGestureRecognizer has no API that reports it",
              "the caret blink: UIKit has no API that reports its timing",
              "UIView.animate(withDuration:) has no default duration: every call states its own",
              "the pointer effects (hover): XCUITest has no hover on iOS and the screenshot shows no pointer",
            ]

            """)
    }

    private func readColors() {
        let contrasts: [(name: String, contrast: UIAccessibilityContrast)] = [("normal", .normal), ("high", .high)]
        let levels: [(name: String, level: UIUserInterfaceLevel)] = [("base", .base), ("elevated", .elevated)]
        if colorNames.isEmpty { failures.append("no colour names: run.sh passes them as colors=<names>") }
        let colorClass = UIColor.self as AnyObject
        for name in colorNames {
            guard colorClass.responds(to: NSSelectorFromString(name)) == true,
                let color = colorClass.value(forKey: name) as? UIColor
            else {
                reads.append("[[color]]\nname = \(quoted(name))\nresponds = false\n\n")
                continue
            }
            for appearance in appearances {
                for contrast in contrasts {
                    for level in levels {
                        let traits = UITraitCollection { traits in
                            traits.userInterfaceStyle = appearance.style
                            traits.accessibilityContrast = contrast.contrast
                            traits.userInterfaceLevel = level.level
                        }
                        reads.append(
                            """
                            [[color]]
                            name = \(quoted(name))
                            responds = true
                            appearance = \(quoted(appearance.name))
                            contrast = \(quoted(contrast.name))
                            level = \(quoted(level.name))
                            value = \(colorValue(color, traits))

                            """)
                    }
                }
            }
        }
    }

    private func readTextStyles() {
        for (styleName, style) in textStyles {
            let metrics = UIFontMetrics(forTextStyle: style)
            let large = UITraitCollection(preferredContentSizeCategory: .large)
            let baseSize = UIFont.preferredFont(forTextStyle: style, compatibleWith: large).pointSize
            for (categoryName, category) in categories {
                for bold in [false, true] {
                    let traits = UITraitCollection { traits in
                        traits.preferredContentSizeCategory = category
                        traits.legibilityWeight = bold ? .bold : .regular
                    }
                    let font = UIFont.preferredFont(forTextStyle: style, compatibleWith: traits)
                    let scaled = metrics.scaledFont(for: UIFont.systemFont(ofSize: baseSize), compatibleWith: traits)
                    reads.append(
                        """
                        [[text_style]]
                        style = \(quoted(styleName))
                        text_style = \(quoted(style.rawValue))
                        category = \(quoted(categoryName))
                        content_size_category = \(quoted(category.rawValue))
                        legibility_weight = \(quoted(bold ? "bold" : "regular"))
                        font = \(fontValue(font))
                        metrics_scaled_value_100 = \(precise(Double(metrics.scaledValue(for: 100, compatibleWith: traits))))
                        metrics_scaled_value_large_size = \(precise(Double(metrics.scaledValue(for: baseSize, compatibleWith: traits))))
                        metrics_scaled_system_font = \(fontValue(scaled))
                        large_size_pt = \(number(baseSize))

                        """)
                }
            }
        }
    }

    private func readSymbols() {
        for symbol in symbolNames {
            var configurations: [(String, String, UIImage.SymbolConfiguration?)] = [("none", "none", nil)]
            for (styleName, style) in textStyles {
                for (scaleName, scale) in symbolScales {
                    configurations.append((styleName, scaleName, UIImage.SymbolConfiguration(textStyle: style, scale: scale)))
                }
            }
            for (styleName, scaleName, configuration) in configurations {
                let image = configuration.map { UIImage(systemName: symbol, withConfiguration: $0) }
                    ?? UIImage(systemName: symbol)
                guard let image else {
                    reads.append("[[symbol]]\nname = \(quoted(symbol))\nexists = false\n\n")
                    continue
                }
                reads.append(
                    """
                    [[symbol]]
                    name = \(quoted(symbol))
                    exists = true
                    text_style = \(quoted(styleName))
                    scale = \(quoted(scaleName))
                    size_pt = \(pair(image.size))
                    alignment_insets_pt = \(edgeInsets(image.alignmentRectInsets))
                    baseline_offset_pt = \(image.baselineOffsetFromBottom.map(number) ?? absent)
                    configuration = \(image.symbolConfiguration.map { described($0) } ?? absent)

                    """)
            }
        }
    }

    private func readCaret() {
        let frame = CGRect(x: 0, y: 0, width: 300, height: 200)
        let textView = UITextView(frame: frame)
        let textField = UITextField(frame: CGRect(x: 0, y: 0, width: 300, height: 44))
        textView.text = "Text"
        textField.text = "Text"
        view.addSubview(textView)
        view.addSubview(textField)
        reads.append(
            """
            [caret]
            harness_text_view_frame_pt = \(rectangle(frame))
            text_view_default_font = \(fontValue(textView.font))
            text_view_tint_light = \(colorValue(textView.tintColor, UITraitCollection(userInterfaceStyle: .light)))
            text_view_tint_dark = \(colorValue(textView.tintColor, UITraitCollection(userInterfaceStyle: .dark)))

            """)
        var fonts: [(String, UIFont)] = textStyles.map { ($0.name, UIFont.preferredFont(forTextStyle: $0.style)) }
        for size in [12, 17, 22, 28, 34, 48, 64] {
            fonts.append(("system_\(size)", UIFont.systemFont(ofSize: CGFloat(size))))
        }
        for (name, font) in fonts {
            textView.font = font
            textField.font = font
            textView.layoutIfNeeded()
            textField.layoutIfNeeded()
            let viewRect = textView.caretRect(for: textView.endOfDocument)
            let fieldRect = textField.caretRect(for: textField.endOfDocument)
            reads.append(
                """
                [[caret.rect]]
                font_source = \(quoted(name))
                font = \(fontValue(font))
                text_view_caret_rect_pt = \(rectangle(viewRect))
                text_field_caret_rect_pt = \(rectangle(fieldRect))

                """)
        }
        textView.removeFromSuperview()
        textField.removeFromSuperview()
    }

    private func readBehavior() {
        let scrollView = UIScrollView()
        let tap = UITapGestureRecognizer()
        reads.append(
            """
            [behavior]
            scroll_view_bounces = \(scrollView.bounces)
            scroll_view_delays_content_touches = \(scrollView.delaysContentTouches)
            scroll_view_index_display_mode = \(scrollView.indexDisplayMode.rawValue)
            tap_number_of_taps_required = \(tap.numberOfTapsRequired)
            pan_minimum_number_of_touches = \(UIPanGestureRecognizer().minimumNumberOfTouches)

            """)
    }

    private func environmentEntry() -> String {
        let environment = ProcessInfo.processInfo.environment
        let info = Bundle.main.infoDictionary ?? [:]
        func value(_ key: String) -> String { quoted(environment[key] ?? "absent") }
        func plist(_ key: String) -> String { quoted((info[key] as? String) ?? "absent") }
        let styles = configurationStyles().map { quoted($0.name) }.joined(separator: ", ")
        let names = specimens(quick: quick).map { quoted($0.name) }.joined(separator: ", ")
        return """
            [environment]
            mode = \(quoted(pressMode ? "press" : "render"))
            quick = \(quick)
            simulator_runtime_version = \(value("SIMULATOR_RUNTIME_VERSION"))
            simulator_runtime_build = \(value("SIMULATOR_RUNTIME_BUILD_VERSION"))
            simulator_device_name = \(value("SIMULATOR_DEVICE_NAME"))
            simulator_model_identifier = \(value("SIMULATOR_MODEL_IDENTIFIER"))
            system_version = \(quoted(UIDevice.current.systemVersion))
            operating_system_version = \(quoted(ProcessInfo.processInfo.operatingSystemVersionString))
            xcode = \(plist("DTXcode"))
            xcode_build = \(plist("DTXcodeBuild"))
            sdk = \(plist("DTSDKName"))
            sdk_build = \(plist("DTSDKBuild"))
            platform_version = \(plist("DTPlatformVersion"))
            platform_build = \(plist("DTPlatformBuild"))
            compiler = \(plist("DTCompiler"))
            styles = [\(styles)]
            specimens = [\(names)]
            sizes = [\(sizes.map { quoted($0.name) }.joined(separator: ", "))]
            pages = \(pages.count)
            title = "Button"
            render_draw = "drawHierarchy(in:afterScreenUpdates: true), UIGraphicsImageRenderer, standard range"
            render_screen = "xcrun simctl io screenshot, cropped to the cell"
            settle_seconds = \(settleSeconds)
            press_settle_seconds = \(pressSettleSeconds)
            animations = \(quoted(pressMode ? "enabled" : "disabled (UIView.setAnimationsEnabled(false))"))

            """
    }

    private func finish() {
        guard !finished else { return }
        finished = true
        let index = ([environmentEntry()] + reads + entries).joined(separator: "\n")
        do {
            try index.write(to: out.appendingPathComponent("index.toml"), atomically: true, encoding: .utf8)
        } catch {
            failures.append("index.toml: \(error)")
        }
        let retakeLog = retakes.map { $0 + "\n" }.joined()
        try? retakeLog.write(to: out.appendingPathComponent("sync/retakes"), atomically: true, encoding: .utf8)
        let status = failures.isEmpty ? "ok\n" : failures.joined(separator: "\n") + "\n"
        try? status.write(to: out.appendingPathComponent("sync/done"), atomically: true, encoding: .utf8)
        guard pressMode else { exit(failures.isEmpty ? 0 : 1) }
        for cell in cells { cell.view.removeFromSuperview() }
        let label = UILabel(frame: CGRect(x: margin, y: view.safeAreaInsets.top + margin, width: 200, height: 40))
        label.text = failures.isEmpty ? "press-done" : "press-failed"
        label.accessibilityIdentifier = "press-done"
        view.addSubview(label)
        doneLabel = label
    }
}
