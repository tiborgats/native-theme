// The iOS host of the UIKit measurements (the 0.8.0 plan, D4; spike I).
//
// A UIButton in each UIButton.Configuration style the SDK offers (iOS 26 adds
// the four Liquid Glass styles), at each `buttonSize`, in each state, under the
// light and the dark appearance (the window's `overrideUserInterfaceStyle`).
// One page per appearance, style and size: five cells of the system
// background, one per state, each holding a button at its intrinsic size,
// centred. The states: normal; highlighted (`isHighlighted = true`, no touch);
// disabled; toggle_off (`changesSelectionAsPrimaryAction`, not selected);
// selected (the same, `isSelected = true`). UIKit's animations are disabled
// and each page settles for a second before it is drawn.
//
// Each cell is drawn twice: by the app (`drawHierarchy(in:afterScreenUpdates:)`
// into a UIGraphicsImageRenderer of the screen's scale and the standard range)
// to draw/, and cropped from the simulator's own screenshot of the page to
// screen/. The screenshot is taken by tools/ios-host/run.sh: the app writes
// sync/ready-<page>, the script writes sync/screen-<page>.png. index.toml
// records the simulator, the build (the DT keys Xcode writes into the
// bundle's Info.plist), the screen and each cell's button frame.
//
// Launched by run.sh as `simctl launch <device> <bundle> <run>`; writes into
// Documents/<run>/ of its data container, then exits (1 where a step failed).

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

let sizes: [(name: String, size: UIButton.Configuration.Size)] = [
    ("mini", .mini), ("small", .small), ("medium", .medium), ("large", .large),
]

let appearances: [(name: String, style: UIUserInterfaceStyle)] = [("light", .light), ("dark", .dark)]

enum ButtonState: String, CaseIterable {
    case normal
    case highlighted
    case disabled
    case toggleOff = "toggle_off"
    case selected
}

struct Page {
    let appearance: (name: String, style: UIUserInterfaceStyle)
    let style: Style
    let size: (name: String, size: UIButton.Configuration.Size)

    var path: String { "\(appearance.name)/\(style.name)/\(size.name)" }
}

final class Cell {
    let state: ButtonState
    let view: UIView
    let button: UIButton

    init(state: ButtonState, view: UIView, button: UIButton) {
        self.state = state
        self.view = view
        self.button = button
    }
}

func quoted(_ text: String) -> String {
    let escaped = text.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "\"", with: "\\\"")
    return "\"\(escaped)\""
}

func number(_ value: CGFloat) -> String { String(format: "%.3f", Double(value)) }

func rectangle(_ rect: CGRect) -> String {
    "[\(number(rect.minX)), \(number(rect.minY)), \(number(rect.width)), \(number(rect.height))]"
}

final class HostViewController: UIViewController {
    private let pages: [Page]
    private let run: String
    private let out: URL
    private var pageIndex = 0
    private var cells: [Cell] = []
    private var entries: [String] = []
    private var failures: [String] = []
    private var started = false

    init() {
        var pages: [Page] = []
        for appearance in appearances {
            for style in configurationStyles() {
                for size in sizes {
                    pages.append(Page(appearance: appearance, style: style, size: size))
                }
            }
        }
        self.pages = pages
        let argument = CommandLine.arguments.dropFirst().first ?? "run"
        run = argument.allSatisfy { $0.isLetter || $0.isNumber } ? argument : "run"
        let documents = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first
            ?? URL(fileURLWithPath: NSTemporaryDirectory())
        out = documents.appendingPathComponent(run, isDirectory: true)
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) { nil }

    override var prefersStatusBarHidden: Bool { true }
    override var prefersHomeIndicatorAutoHidden: Bool { true }

    override func viewDidLoad() {
        super.viewDidLoad()
        view.backgroundColor = .systemBackground
    }

    override func viewDidAppear(_ animated: Bool) {
        super.viewDidAppear(animated)
        guard !started else { return }
        started = true
        UIView.setAnimationsEnabled(false)
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
        show(page: 0)
    }

    private func show(page index: Int) {
        pageIndex = index
        let page = pages[index]
        for cell in cells { cell.view.removeFromSuperview() }
        cells = []
        view.window?.overrideUserInterfaceStyle = page.appearance.style

        let margin: CGFloat = 16
        let height: CGFloat = 96
        let gap: CGFloat = 8
        var top = view.safeAreaInsets.top + margin
        for state in ButtonState.allCases {
            let cellView = UIView(frame: CGRect(x: margin, y: top, width: view.bounds.width - 2 * margin, height: height))
            cellView.backgroundColor = .systemBackground
            var configuration = page.style.make()
            configuration.title = "Button"
            configuration.buttonSize = page.size.size
            let button = UIButton(configuration: configuration)
            button.translatesAutoresizingMaskIntoConstraints = false
            cellView.addSubview(button)
            NSLayoutConstraint.activate([
                button.centerXAnchor.constraint(equalTo: cellView.centerXAnchor),
                button.centerYAnchor.constraint(equalTo: cellView.centerYAnchor),
            ])
            view.addSubview(cellView)
            switch state {
            case .normal: break
            case .highlighted: button.isHighlighted = true
            case .disabled: button.isEnabled = false
            case .toggleOff: button.changesSelectionAsPrimaryAction = true
            case .selected:
                button.changesSelectionAsPrimaryAction = true
                button.isSelected = true
            }
            cells.append(Cell(state: state, view: cellView, button: button))
            top += height + gap
        }
        view.layoutIfNeeded()
        DispatchQueue.main.asyncAfter(deadline: .now() + 1.0) { [weak self] in self?.draw() }
    }

    private func draw() {
        let page = pages[pageIndex]
        let scale = view.window?.windowScene?.screen.scale ?? 1
        for cell in cells {
            let format = UIGraphicsImageRendererFormat(for: cell.view.traitCollection)
            format.scale = scale
            format.opaque = true
            format.preferredRange = .standard
            let renderer = UIGraphicsImageRenderer(bounds: cell.view.bounds, format: format)
            var drawn = false
            let image = renderer.image { _ in
                drawn = cell.view.drawHierarchy(in: cell.view.bounds, afterScreenUpdates: true)
            }
            if !drawn { failures.append("\(page.path)/\(cell.state.rawValue): drawHierarchy returned false") }
            write(image, to: "draw/\(page.path)/\(cell.state.rawValue).png")
        }
        let ready = out.appendingPathComponent("sync/ready-\(pageIndex)")
        do {
            try page.path.write(to: ready, atomically: true, encoding: .utf8)
        } catch {
            failures.append("write \(ready.lastPathComponent): \(error)")
        }
        waitForScreenshot(since: Date())
    }

    private func waitForScreenshot(since start: Date) {
        let screenshot = out.appendingPathComponent("sync/screen-\(pageIndex).png")
        if FileManager.default.fileExists(atPath: screenshot.path) {
            crop(screenshot)
        } else if Date().timeIntervalSince(start) > 60 {
            failures.append("\(pages[pageIndex].path): no screenshot within 60 s")
            finish()
        } else {
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) { [weak self] in
                self?.waitForScreenshot(since: start)
            }
        }
    }

    private func crop(_ screenshot: URL) {
        let page = pages[pageIndex]
        let screen = view.window?.windowScene?.screen
        let scale = screen?.scale ?? 1
        guard let image = UIImage(contentsOfFile: screenshot.path), let source = image.cgImage else {
            failures.append("\(page.path): the screenshot does not load")
            finish()
            return
        }
        let manager = FileManager.default
        do {
            try manager.moveItem(at: screenshot, to: out.appendingPathComponent("pages/\(pageIndex).png"))
        } catch {
            failures.append("\(page.path): keep the screenshot: \(error)")
        }
        let native = screen?.nativeBounds.size ?? .zero
        if CGFloat(source.width) != native.width || CGFloat(source.height) != native.height {
            failures.append("\(page.path): screenshot \(source.width)x\(source.height), screen \(native)")
        }
        for cell in cells {
            let frame = cell.view.convert(cell.view.bounds, to: nil)
            let pixels = CGRect(
                x: (frame.minX * scale).rounded(), y: (frame.minY * scale).rounded(),
                width: (frame.width * scale).rounded(), height: (frame.height * scale).rounded())
            let name = "screen/\(page.path)/\(cell.state.rawValue).png"
            if let cropped = source.cropping(to: pixels) {
                write(UIImage(cgImage: cropped, scale: scale, orientation: .up), to: name)
            } else {
                failures.append("\(name): crop \(pixels) outside the screenshot")
            }
            let button = cell.button
            entries.append(
                """
                [[render]]
                appearance = \(quoted(page.appearance.name))
                style = \(quoted(page.style.name))
                size = \(quoted(page.size.name))
                state = \(quoted(cell.state.rawValue))
                draw = \(quoted("draw/\(page.path)/\(cell.state.rawValue).png"))
                screen = \(quoted(name))
                page = \(quoted("pages/\(pageIndex).png"))
                cell_px = \(rectangle(pixels))
                button_frame_pt = \(rectangle(button.frame))
                intrinsic_size_pt = [\(number(button.intrinsicContentSize.width)), \(number(button.intrinsicContentSize.height))]
                is_highlighted = \(button.isHighlighted)
                is_enabled = \(button.isEnabled)
                is_selected = \(button.isSelected)
                user_interface_style = \(button.traitCollection.userInterfaceStyle.rawValue)

                """)
        }
        if pageIndex == 0 {
            entries.insert(screenEntry(source: source), at: 0)
        }
        if pageIndex + 1 < pages.count {
            show(page: pageIndex + 1)
        } else {
            finish()
        }
    }

    private func screenEntry(source: CGImage) -> String {
        let screen = view.window?.windowScene?.screen
        let traits = view.traitCollection
        let colorSpace = (source.colorSpace?.name as String?) ?? "none"
        return """
            [screen]
            scale = \(number(screen?.scale ?? 0))
            bounds_pt = \(rectangle(screen?.bounds ?? .zero))
            native_bounds_px = \(rectangle(screen?.nativeBounds ?? .zero))
            safe_area_top_pt = \(number(view.safeAreaInsets.top))
            screenshot_color_space = \(quoted(colorSpace))
            display_gamut = \(traits.displayGamut.rawValue)
            preferred_content_size_category = \(quoted(traits.preferredContentSizeCategory.rawValue))

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

    private func environmentEntry() -> String {
        let environment = ProcessInfo.processInfo.environment
        let info = Bundle.main.infoDictionary ?? [:]
        func value(_ key: String) -> String { quoted(environment[key] ?? "absent") }
        func plist(_ key: String) -> String { quoted((info[key] as? String) ?? "absent") }
        let styles = configurationStyles().map { quoted($0.name) }.joined(separator: ", ")
        return """
            [environment]
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
            sizes = [\(sizes.map { quoted($0.name) }.joined(separator: ", "))]
            states = [\(ButtonState.allCases.map { quoted($0.rawValue) }.joined(separator: ", "))]
            title = "Button"
            render_draw = "drawHierarchy(in:afterScreenUpdates: true), UIGraphicsImageRenderer, standard range"
            render_screen = "xcrun simctl io screenshot, cropped to the cell"
            settle_seconds = 1.0

            """
    }

    private func finish() {
        let index = ([environmentEntry()] + entries).joined(separator: "\n")
        do {
            try index.write(to: out.appendingPathComponent("index.toml"), atomically: true, encoding: .utf8)
        } catch {
            failures.append("index.toml: \(error)")
        }
        let status = failures.isEmpty ? "ok\n" : failures.joined(separator: "\n") + "\n"
        try? status.write(to: out.appendingPathComponent("sync/done"), atomically: true, encoding: .utf8)
        exit(failures.isEmpty ? 0 : 1)
    }
}
