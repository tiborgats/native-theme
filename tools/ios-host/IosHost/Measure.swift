// The iOS host's `measure<N>` mode (tools/ios-host/run.sh, `simctl launch`, the
// measurement campaign's passes of docs/todo_v0.7.1_model-completeness-ledger.md
// *Unmeasured register*). The launch arguments: `measure<N>`, `passes=<list>`
// (comma-separated, below) and `colors=<names>` (the UIColor class colours
// run.sh finds in the SDK's headers).
//
// Each pass reads what UIKit reports (written to index.toml before the scenes)
// and draws scenes, each captured by the simulator's own screenshot, as the
// `run` mode does (sync/ready-<n>-<attempt>, sync/screen-<n>-<attempt>.png).
// A scene is drawn in light and in dark (the window's
// `overrideUserInterfaceStyle`); where what UIKit draws may be translucent it
// is drawn over the system background, opaque black and opaque white, all
// else equal, for tools/ios-host/solve.swift. The passes:
//
// - `roles`: the colour roles the facts left pending: the tint dimmed
//   (`tintAdjustmentMode = .dimmed`, and the views behind an alert, which UIKit
//   dims itself), the scene inactive (`traitOverrides.activeAppearance =
//   .inactive`, and every UIColor class colour resolved with that trait),
//   disabled controls (a label, a text field, a segmented control, a switch, a
//   slider, a stepper), list selections (UICollectionView lists in each
//   appearance, a cell normal, selected, highlighted, disabled, selected and
//   disabled; the list configurations resolved per state), selected text in a
//   text view that is first responder and one that is not, UIFindInteraction's
//   match highlights, an alert, an action sheet, a popover and a sheet with
//   the dimming behind them (each scene's view trees record the classes,
//   colours and shadows UIKit drew them with).
// - `fonts`: each text style's font descriptor (its attributes, the font's
//   variation axes and values, feature settings and features), the tight,
//   loose and bold (emphasized) variants, the extra large titles, the fonts
//   UIKit's own controls and class methods give (system, monospaced,
//   monospaced digits, widths, designs), the default cascade list per language
//   and the font CoreText picks per script; a scene of text over the three
//   backdrops (each capture's pixels counted by chroma, which subpixel
//   antialiasing would leave); and the configuration buttons' title and
//   subtitle labels (line break mode, number of lines, the attributed text's
//   attributes) with an overlong word and a long title in a narrow button.
// - `symbols`: each configuration style's default values (`buttonSize`, the
//   corner style, insets, paddings) as a new configuration states them; each
//   style's image at every `buttonSize`, the system types' images and the
//   pop-up indicator's image (the image's description, which names the
//   symbol, its size and symbol configuration, the image view's preferred
//   configuration and frame); a scene of the controls that place a symbol of
//   their own (a toolbar, a navigation bar, a tab bar, a segmented control, a
//   search bar, list cells) with their image views read.
//
// Writes into Documents/<run>/ of its data container: index.toml (the
// environment, the reads, a [[scene_read]] per scene and a [[capture]] per
// crop), screen/<appearance>/<scene>/<part>/<backdrop>/<region>.png (the
// crops), pages/<appearance>/<scene>/<part>-<backdrop>.png (the screenshots)
// and sync/ (the handshake; done holds ok or the failures). Exits 1 where a
// step failed.

import CoreText
import UIKit

/// A region of a scene's screenshot kept as a crop.
struct MeasureRegion {
    let name: String
    let backdrop: String
    /// The region in the window's points, read when the screenshot arrives.
    let rect: @MainActor () -> CGRect
}

/// A scene: what it draws, when it is captured and how it is undone.
struct MeasureScene {
    let name: String
    let part: String
    let backdrop: String
    let appearance: Appearance
    /// Seconds from the setup to the reads and the first screenshot.
    let settle: Double
    /// Whether two screenshots must agree before one is kept.
    let settles: Bool
    let setup: @MainActor (MeasureController) -> [MeasureRegion]
    let read: (@MainActor (MeasureController, SceneRead) -> Void)?
    /// Undoes what the setup presented; calls its argument when done.
    let teardown: (@MainActor (MeasureController, @escaping () -> Void) -> Void)?

    var page: String { "pages/\(appearance.name)/\(name)/\(part)-\(backdrop).png" }
}

/// One [[scene_read]] table: its keys, then its [[scene_read.view]] tables.
final class SceneRead {
    var fields: [String] = []
    var views: [String] = []

    func set(_ key: String, _ value: String) { fields.append("\(key) = \(value)") }
}

let measureFills: [(name: String, color: UIColor)] = [
    ("system", .systemBackground), ("black", UIColor(red: 0, green: 0, blue: 0, alpha: 1)),
    ("white", UIColor(red: 1, green: 1, blue: 1, alpha: 1)),
]

func fillColor(_ name: String) -> UIColor { measureFills.first { $0.name == name }?.color ?? .systemBackground }

/// A value as TOML: dictionaries as inline tables (keys sorted), arrays,
/// numbers, booleans, strings, fonts, colours; anything else described.
func plist(_ value: Any?) -> String {
    guard let value else { return absent }
    if let number = value as? NSNumber {
        if CFGetTypeID(number) == CFBooleanGetTypeID() { return number.boolValue ? "true" : "false" }
        return String(format: "%.6g", number.doubleValue)
    }
    if let string = value as? String { return quoted(string) }
    if let font = value as? UIFont { return fontValue(font) }
    if let color = value as? UIColor { return colorValue(color.cgColor) }
    if let dictionary = value as? [AnyHashable: Any] {
        let entries = dictionary.map { (key: keyName($0.key), value: $0.value) }.sorted { $0.key < $1.key }
        return "{ " + entries.map { "\(quoted($0.key)) = \(plist($0.value))" }.joined(separator: ", ") + " }"
    }
    if let array = value as? [Any] { return "[" + array.map { plist($0) }.joined(separator: ", ") + "]" }
    return described(value)
}

/// A dictionary key's name: a string, a string-valued raw value (an
/// attribute name, a trait key), or its description.
func keyName(_ key: AnyHashable) -> String {
    if let string = key.base as? String { return string }
    if let raw = key.base as? any RawRepresentable, let string = raw.rawValue as? String { return string }
    return String(describing: key.base)
}

/// The attributes of an attributed string, run by run.
func attributesValue(_ text: NSAttributedString?) -> String {
    guard let text, text.length > 0 else { return absent }
    var runs: [String] = []
    text.enumerateAttributes(in: NSRange(location: 0, length: text.length)) { attributes, range, _ in
        var parts = ["range = [\(range.location), \(range.length)]"]
        for (key, value) in attributes.sorted(by: { $0.key.rawValue < $1.key.rawValue }) {
            parts.append("\(quoted(key.rawValue)) = \(plist(value))")
        }
        runs.append("{ " + parts.joined(separator: ", ") + " }")
    }
    return "[" + runs.joined(separator: ", ") + "]"
}

func lineBreakName(_ mode: NSLineBreakMode) -> String {
    switch mode {
    case .byWordWrapping: return "by_word_wrapping"
    case .byCharWrapping: return "by_char_wrapping"
    case .byClipping: return "by_clipping"
    case .byTruncatingHead: return "by_truncating_head"
    case .byTruncatingTail: return "by_truncating_tail"
    case .byTruncatingMiddle: return "by_truncating_middle"
    @unknown default: return "raw \(mode.rawValue)"
    }
}

/// A font's CoreText details: its descriptor's attributes, variation axes and
/// values, feature settings and (with `features`) the features it offers.
func fontDetail(_ font: UIFont, features: Bool) -> String {
    let ctFont = font as CTFont
    var parts = [
        "font = \(fontValue(font))",
        "descriptor_attributes = \(plist(Dictionary(uniqueKeysWithValues: font.fontDescriptor.fontAttributes.map { ($0.key.rawValue, $0.value) })))",
        "postscript_name = \(quoted(CTFontCopyPostScriptName(ctFont) as String))",
        "family_name = \(quoted(CTFontCopyFamilyName(ctFont) as String))",
        "variation_axes = \(plist(CTFontCopyVariationAxes(ctFont) as? [Any]))",
        "variation = \(plist(CTFontCopyVariation(ctFont) as? [AnyHashable: Any]))",
        "feature_settings = \(plist(CTFontCopyFeatureSettings(ctFont) as? [Any]))",
        "optical_size_attribute = \(plist(CTFontCopyAttribute(ctFont, kCTFontOpticalSizeAttribute)))",
        "symbolic_traits_ct = \(CTFontGetSymbolicTraits(ctFont).rawValue)",
    ]
    if features { parts.append("features = \(plist(CTFontCopyFeatures(ctFont) as? [Any]))") }
    return parts.joined(separator: "\n")
}

/// A list cell whose configuration state can be forced disabled.
final class MeasureListCell: UICollectionViewListCell {
    var forcedDisabled = false

    override var configurationState: UICellConfigurationState {
        var state = super.configurationState
        if forcedDisabled { state.isDisabled = true }
        return state
    }
}

final class MeasureController: UIViewController, UIPopoverPresentationControllerDelegate {
    private let run: String
    private let out: URL
    private let passes: [String]
    private let colorNames: [String]
    private var scenes: [MeasureScene] = []
    private var sceneIndex = 0
    private var regions: [MeasureRegion] = []
    private var attempt = 0
    private var codeRetakes = 0
    private var settleShots = 0
    private var previousCrops: [[UInt8]?] = []
    private var reads: [String] = []
    private var entries: [String] = []
    private var failures: [String] = []
    private var retakes: [String] = []
    private var started = false
    private var finished = false
    private var screenWritten = false
    private var codeWindow: UIWindow?
    private var codeSquares: [UIView] = []
    /// The views a scene names for its reads, and the objects it keeps alive.
    var named: [String: UIView] = [:]
    var kept: [AnyObject] = []
    var presentedForScene: UIViewController?

    init() {
        let arguments = Array(CommandLine.arguments.dropFirst())
        let argument = arguments.first ?? "measure"
        run = argument.allSatisfy { $0.isLetter || $0.isNumber } ? argument : "measure"
        passes = arguments.first { $0.hasPrefix("passes=") }.map {
            $0.dropFirst("passes=".count).split(separator: ",").map(String.init)
        } ?? []
        colorNames = arguments.first { $0.hasPrefix("colors=") }.map {
            $0.dropFirst("colors=".count).split(separator: ",").map(String.init)
        } ?? []
        let documents = FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first
            ?? URL(fileURLWithPath: NSTemporaryDirectory())
        out = documents.appendingPathComponent(run, isDirectory: true)
        super.init(nibName: nil, bundle: nil)
    }

    required init?(coder: NSCoder) { nil }

    override var prefersStatusBarHidden: Bool { true }

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
        for directory in ["sync", "screen", "pages"] {
            do {
                try manager.createDirectory(
                    at: out.appendingPathComponent(directory, isDirectory: true), withIntermediateDirectories: true)
            } catch {
                failures.append("create \(directory): \(error)")
            }
        }
        UIView.setAnimationsEnabled(false)
        installCode()
        if passes.isEmpty { failures.append("no passes: run.sh passes them as passes=<list>") }
        for pass in passes {
            switch pass {
            case "roles":
                readInactiveColors()
                readListConfigurations()
                scenes += roleScenes()
            case "fonts":
                readFonts()
                scenes += fontScenes()
            case "symbols":
                readConfigurationDefaults()
                readButtonSymbols()
                scenes += symbolScenes()
            default:
                failures.append("unknown pass \(pass)")
            }
        }
        guard !scenes.isEmpty else {
            finish()
            return
        }
        show(0)
    }

    // MARK: the page code

    /// The page code (the scene's number in binary, red for a set bit, blue
    /// for a clear one) in a window above every other, alerts included, under
    /// the top safe area: a screenshot whose code does not read as the scene's
    /// number is stale or covered, and is taken again.
    private func installCode() {
        guard let scene = view.window?.windowScene else {
            failures.append("no window scene for the page code")
            return
        }
        let window = UIWindow(windowScene: scene)
        window.windowLevel = UIWindow.Level(rawValue: UIWindow.Level.alert.rawValue + 100)
        window.backgroundColor = .clear
        window.isUserInteractionEnabled = false
        let root = UIViewController()
        root.view.backgroundColor = .clear
        root.view.isUserInteractionEnabled = false
        window.rootViewController = root
        window.isHidden = false
        let top = view.safeAreaInsets.top + 2
        for bit in 0..<codeBits {
            let square = UIView(
                frame: CGRect(x: margin + CGFloat(bit) * codeSquare, y: top, width: codeSquare, height: codeSquare))
            root.view.addSubview(square)
            codeSquares.append(square)
        }
        codeWindow = window
    }

    private func setCode(_ number: Int) {
        for (bit, square) in codeSquares.enumerated() {
            square.backgroundColor =
                (number >> bit) & 1 == 1
                ? UIColor(red: 1, green: 0, blue: 0, alpha: 1) : UIColor(red: 0, green: 0, blue: 1, alpha: 1)
        }
    }

    private var pixelScale: CGFloat { view.window?.windowScene?.screen.scale ?? 1 }

    private func codeProblem(_ bytes: [UInt8], width: Int, height: Int) -> String? {
        let scale = pixelScale
        var read = 0
        for bit in 0..<codeBits {
            guard let square = codeSquares.dropFirst(bit).first else { return "no page code" }
            let center = square.convert(CGPoint(x: square.bounds.midX, y: square.bounds.midY), to: nil)
            let x = Int((center.x * scale).rounded(.down))
            let y = Int((center.y * scale).rounded(.down))
            guard x >= 0, y >= 0, x < width, y < height else { return "the page code is outside the screenshot" }
            let offset = (y * width + x) * 4
            let r = Int(bytes[offset])
            let b = Int(bytes[offset + 2])
            if r > 200 && b < 60 {
                read |= 1 << bit
            } else if !(b > 200 && r < 60) {
                return "page code bit \(bit) reads as neither red nor blue"
            }
        }
        if read != sceneIndex & ((1 << codeBits) - 1) { return "the page code reads \(read), not \(sceneIndex)" }
        return nil
    }

    // MARK: the scenes

    /// The stage's top: under the page code.
    var stageTop: CGFloat { view.safeAreaInsets.top + margin }

    var stageWidth: CGFloat { view.bounds.width - 2 * margin }

    /// A cell of the stage filled with a backdrop.
    func cell(x: CGFloat, y: CGFloat, width: CGFloat, height: CGFloat, fill: String) -> UIView {
        let cellView = UIView(frame: CGRect(x: x, y: y, width: width, height: height))
        cellView.backgroundColor = fillColor(fill)
        view.addSubview(cellView)
        return cellView
    }

    /// Rows of the three backdrops (columns), each cell holding `make(row,
    /// backdrop)` centred, or spanning the cell less `barInset` where `spans`;
    /// a region per cell, named by its row.
    func grid(
        rows: [String], height: CGFloat, spans: Bool = false, columns: [String] = measureFills.map { $0.name },
        make: (String, String) -> UIView
    ) -> [MeasureRegion] {
        var regions: [MeasureRegion] = []
        let count = CGFloat(columns.count)
        let width = ((stageWidth - (count - 1) * gap) / count).rounded(.down)
        for (rowIndex, row) in rows.enumerated() {
            for (columnIndex, column) in columns.enumerated() {
                let cellView = cell(
                    x: margin + CGFloat(columnIndex) * (width + gap), y: stageTop + CGFloat(rowIndex) * (height + gap),
                    width: width, height: height, fill: column)
                let content = make(row, column)
                content.translatesAutoresizingMaskIntoConstraints = false
                cellView.addSubview(content)
                var constraints = [content.centerYAnchor.constraint(equalTo: cellView.centerYAnchor)]
                if spans {
                    constraints += [
                        content.leadingAnchor.constraint(equalTo: cellView.leadingAnchor, constant: barInset),
                        content.trailingAnchor.constraint(equalTo: cellView.trailingAnchor, constant: -barInset),
                    ]
                } else {
                    constraints.append(content.centerXAnchor.constraint(equalTo: cellView.centerXAnchor))
                    constraints.append(content.widthAnchor.constraint(lessThanOrEqualTo: cellView.widthAnchor))
                }
                NSLayoutConstraint.activate(constraints)
                named["\(row)/\(column)"] = content
                regions.append(MeasureRegion(name: row, backdrop: column, rect: { [weak cellView] in
                    guard let cellView else { return .zero }
                    return cellView.convert(cellView.bounds, to: nil)
                }))
            }
        }
        return regions
    }

    /// The row height that fits `count` rows on the stage, at most `limit`.
    func rowHeight(_ count: Int, limit: CGFloat) -> CGFloat {
        let available = view.bounds.height - view.safeAreaInsets.bottom - margin - stageTop
        return min(limit, ((available - CGFloat(count - 1) * gap) / CGFloat(max(count, 1))).rounded(.down))
    }

    private func show(_ index: Int) {
        guard !finished else { return }
        sceneIndex = index
        let scene = scenes[index]
        for subview in view.subviews { subview.removeFromSuperview() }
        named = [:]
        kept = []
        presentedForScene = nil
        codeRetakes = 0
        settleShots = 0
        previousCrops = []
        view.backgroundColor = .systemBackground
        view.window?.overrideUserInterfaceStyle = scene.appearance.style
        setCode(index)
        regions = scene.setup(self)
        view.layoutIfNeeded()
        DispatchQueue.main.asyncAfter(deadline: .now() + scene.settle) { [weak self] in
            guard let self, !self.finished else { return }
            let read = SceneRead()
            read.set("scene", quoted(scene.name))
            read.set("part", quoted(scene.part))
            read.set("backdrop", quoted(scene.backdrop))
            read.set("appearance", quoted(scene.appearance.name))
            read.set("page", quoted(scene.page))
            scene.read?(self, read)
            self.reads.append(
                "[[scene_read]]\n" + read.fields.joined(separator: "\n") + "\n\n" + read.views.joined())
            self.requestScreenshot(attempt: 0)
        }
    }

    private func requestScreenshot(attempt: Int) {
        self.attempt = attempt
        let ready = out.appendingPathComponent("sync/ready-\(sceneIndex)-\(attempt)")
        do {
            try scenes[sceneIndex].page.write(to: ready, atomically: true, encoding: .utf8)
        } catch {
            failures.append("write \(ready.lastPathComponent): \(error)")
        }
        waitForScreenshot(since: Date())
    }

    private func waitForScreenshot(since start: Date) {
        guard !finished else { return }
        let screenshot = out.appendingPathComponent("sync/screen-\(sceneIndex)-\(attempt).png")
        if FileManager.default.fileExists(atPath: screenshot.path) {
            received(screenshot)
        } else if Date().timeIntervalSince(start) > 300 {
            failures.append("\(scenes[sceneIndex].page): no screenshot within 300 s")
            finish()
        } else {
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.05) { [weak self] in
                self?.waitForScreenshot(since: start)
            }
        }
    }

    private func pixels(_ rect: CGRect, width: Int, height: Int) -> CGRect {
        let scale = pixelScale
        let frame = CGRect(
            x: (rect.minX * scale).rounded(), y: (rect.minY * scale).rounded(), width: (rect.width * scale).rounded(),
            height: (rect.height * scale).rounded())
        return frame.intersection(CGRect(x: 0, y: 0, width: width, height: height))
    }

    private func retake(after delay: Double) {
        DispatchQueue.main.asyncAfter(deadline: .now() + delay) { [weak self] in
            guard let self else { return }
            self.requestScreenshot(attempt: self.attempt + 1)
        }
    }

    private func received(_ screenshot: URL) {
        let scene = scenes[sceneIndex]
        let manager = FileManager.default
        guard let image = UIImage(contentsOfFile: screenshot.path), let source = image.cgImage,
            let bytes = samples(source)
        else {
            failures.append("\(scene.page): the screenshot does not load")
            finish()
            return
        }
        if let problem = codeProblem(bytes, width: source.width, height: source.height) {
            if codeRetakes < maxRetakes {
                codeRetakes += 1
                try? manager.removeItem(at: screenshot)
                retakes.append("\(scene.page) attempt \(attempt): \(problem)")
                retake(after: retakeDelaySeconds)
                return
            }
            failures.append("\(scene.page): \(problem) after \(maxRetakes) retakes")
        }
        let frames = regions.map { pixels($0.rect(), width: source.width, height: source.height) }
        var settled = true
        if scene.settles {
            let crops = frames.map { frame in source.cropping(to: frame).flatMap { samples($0) } }
            settled = previousCrops.count == crops.count && zip(previousCrops, crops).allSatisfy { pair in
                guard let before = pair.0, let after = pair.1, before.count == after.count else { return false }
                return zip(before, after).allSatisfy { abs(Int($0.0) - Int($0.1)) <= settledLevels }
            }
            if !settled && settleShots < maxSettleShots {
                if !previousCrops.isEmpty { retakes.append("\(scene.page) attempt \(attempt): not settled") }
                settleShots += 1
                previousCrops = crops
                try? manager.removeItem(at: screenshot)
                retake(after: settleShotSeconds)
                return
            }
            if !settled { retakes.append("\(scene.page): not settled after \(maxSettleShots) screenshots, kept") }
        }
        let keptPage = out.appendingPathComponent(scene.page)
        do {
            try manager.createDirectory(at: keptPage.deletingLastPathComponent(), withIntermediateDirectories: true)
            try? manager.removeItem(at: keptPage)
            try manager.moveItem(at: screenshot, to: keptPage)
        } catch {
            failures.append("\(scene.page): keep the screenshot: \(error)")
        }
        for (region, frame) in zip(regions, frames) {
            let name = "screen/\(scene.appearance.name)/\(scene.name)/\(scene.part)/\(region.backdrop)/\(region.name).png"
            var stats = "absent"
            if !frame.isEmpty, let cropped = source.cropping(to: frame) {
                write(UIImage(cgImage: cropped, scale: pixelScale, orientation: .up), to: name)
                if let crop = samples(cropped) { stats = cropStats(crop) }
            } else {
                failures.append("\(name): crop \(frame) outside the screenshot")
            }
            entries.append(
                """
                [[capture]]
                scene = \(quoted(scene.name))
                part = \(quoted(scene.part))
                appearance = \(quoted(scene.appearance.name))
                region = \(quoted(region.name))
                backdrop = \(quoted(region.backdrop))
                screen = \(quoted(name))
                page = \(quoted(scene.page))
                rect_pt = \(rectangle(region.rect()))
                rect_px = \(rectangle(frame))
                settled = \(settled)
                shots = \(settleShots + 1)
                pixels = \(stats)

                """)
        }
        if !screenWritten {
            screenWritten = true
            entries.insert(screenEntry(source: source), at: 0)
        }
        let next: () -> Void = { [weak self] in
            guard let self else { return }
            DispatchQueue.main.asyncAfter(deadline: .now() + 0.5) { [weak self] in
                guard let self else { return }
                if self.sceneIndex + 1 < self.scenes.count { self.show(self.sceneIndex + 1) } else { self.finish() }
            }
        }
        if let teardown = scene.teardown { teardown(self, next) } else { next() }
    }

    /// A crop's pixels: how many, how many have channels more than 2 levels
    /// apart (colour, which grayscale antialiasing of grey text never gives),
    /// and how many distinct colours.
    private func cropStats(_ bytes: [UInt8]) -> String {
        var chroma = 0
        var distinct = Set<UInt32>()
        for pixel in stride(from: 0, to: bytes.count - 3, by: 4) {
            let r = Int(bytes[pixel])
            let g = Int(bytes[pixel + 1])
            let b = Int(bytes[pixel + 2])
            if max(r, g, b) - min(r, g, b) > 2 { chroma += 1 }
            distinct.insert(UInt32(r) << 16 | UInt32(g) << 8 | UInt32(b))
        }
        return "{ count = \(bytes.count / 4), chroma_over_2 = \(chroma), distinct_colors = \(distinct.count) }"
    }

    private func screenEntry(source: CGImage) -> String {
        let screen = view.window?.windowScene?.screen
        let traits = view.traitCollection
        return """
            [screen]
            scale = \(number(screen?.scale ?? 0))
            bounds_pt = \(rectangle(screen?.bounds ?? .zero))
            native_bounds_px = \(rectangle(screen?.nativeBounds ?? .zero))
            screenshot_px = [\(source.width), \(source.height)]
            safe_area_top_pt = \(number(view.safeAreaInsets.top))
            safe_area_bottom_pt = \(number(view.safeAreaInsets.bottom))
            screenshot_color_space = \(quoted((source.colorSpace?.name as String?) ?? "none"))
            preferred_content_size_category = \(quoted(traits.preferredContentSizeCategory.rawValue))
            interface_idiom = \(traits.userInterfaceIdiom.rawValue)
            layout = "each scene's regions are its [[capture]] rect_pt; a grid's columns are the backdrops system (UIColor.systemBackground), black (#000000) and white (#ffffff), margin_pt and gap_pt apart"
            margin_pt = \(number(margin))
            gap_pt = \(number(gap))
            page_code = "\(codeBits) squares of \(number(codeSquare)) pt in a window above the alerts, under the top safe area, red #ff0000 for a set bit, blue #0000ff for a clear one, the scene's number from bit 0 at the left"

            """
    }

    private func write(_ image: UIImage, to name: String) {
        let url = out.appendingPathComponent(name)
        do {
            try FileManager.default.createDirectory(at: url.deletingLastPathComponent(), withIntermediateDirectories: true)
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
        func plistValue(_ key: String) -> String { quoted((info[key] as? String) ?? "absent") }
        return """
            [environment]
            mode = "measure"
            passes = [\(passes.map(quoted).joined(separator: ", "))]
            simulator_runtime_version = \(value("SIMULATOR_RUNTIME_VERSION"))
            simulator_runtime_build = \(value("SIMULATOR_RUNTIME_BUILD_VERSION"))
            simulator_device_name = \(value("SIMULATOR_DEVICE_NAME"))
            simulator_model_identifier = \(value("SIMULATOR_MODEL_IDENTIFIER"))
            system_version = \(quoted(UIDevice.current.systemVersion))
            operating_system_version = \(quoted(ProcessInfo.processInfo.operatingSystemVersionString))
            xcode = \(plistValue("DTXcode"))
            xcode_build = \(plistValue("DTXcodeBuild"))
            sdk = \(plistValue("DTSDKName"))
            sdk_build = \(plistValue("DTSDKBuild"))
            platform_version = \(plistValue("DTPlatformVersion"))
            platform_build = \(plistValue("DTPlatformBuild"))
            scenes = \(scenes.count)
            animations = "disabled (UIView.setAnimationsEnabled(false)); presentations are made with animated: false"
            reduce_motion = \(UIAccessibility.isReduceMotionEnabled)
            darker_system_colors = \(UIAccessibility.isDarkerSystemColorsEnabled)
            bold_text = \(UIAccessibility.isBoldTextEnabled)
            button_shapes = \(UIAccessibility.buttonShapesEnabled)

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
        exit(failures.isEmpty ? 0 : 1)
    }

    // MARK: view trees

    /// A view and its subviews, depth first, at most `limit` views and
    /// `depth` deep, as [[scene_read.view]] tables under `root`.
    func tree(_ root: UIView, label: String, limit: Int = 80, depth maxDepth: Int = 10) -> [String] {
        var tables: [String] = []
        var count = 0
        func visit(_ view: UIView, path: String, depth: Int) {
            guard count < limit, depth <= maxDepth else { return }
            count += 1
            let layer = view.layer
            let traits = view.traitCollection
            var fields = [
                "[[scene_read.view]]",
                "root = \(quoted(label))",
                "path = \(quoted(path))",
                "class = \(quoted(String(describing: type(of: view))))",
                "frame_pt = \(rectangle(view.convert(view.bounds, to: nil)))",
                "hidden = \(view.isHidden)",
                "alpha = \(number(view.alpha))",
                "background = \(colorValue(view.backgroundColor, traits))",
                "tint = \(colorValue(view.tintColor, traits))",
                "tint_adjustment_mode = \(view.tintAdjustmentMode.rawValue)",
                "layer_opacity = \(number(CGFloat(layer.opacity)))",
                "layer_background = \(colorValue(layer.backgroundColor))",
                "corner_radius_pt = \(number(layer.cornerRadius))",
                "corner_curve = \(quoted(layer.cornerCurve.rawValue))",
                "border_width_pt = \(number(layer.borderWidth))",
                "border_color = \(colorValue(layer.borderColor))",
                "shadow_opacity = \(number(CGFloat(layer.shadowOpacity)))",
                "shadow_radius_pt = \(number(layer.shadowRadius))",
                "shadow_offset_pt = \(pair(layer.shadowOffset))",
                "shadow_color = \(colorValue(layer.shadowColor))",
                "compositing_filter = \(layer.compositingFilter.map { described($0) } ?? absent)",
                "filters = [\((layer.filters ?? []).map { described($0) }.joined(separator: ", "))]",
            ]
            if let label = view as? UILabel {
                fields += [
                    "text = \(label.text.map(quoted) ?? absent)",
                    "font = \(fontValue(label.font))",
                    "text_color = \(colorValue(label.textColor, traits))",
                    "is_enabled = \(label.isEnabled)",
                    "number_of_lines = \(label.numberOfLines)",
                    "line_break_mode = \(quoted(lineBreakName(label.lineBreakMode)))",
                    "attributed_text = \(attributesValue(label.attributedText))",
                ]
            }
            if let imageView = view as? UIImageView {
                fields += [
                    "image = \(imageView.image.map { described($0) } ?? absent)",
                    "image_size_pt = \(imageView.image.map { pair($0.size) } ?? absent)",
                    "image_is_symbol = \(imageView.image?.isSymbolImage ?? false)",
                    "image_symbol_configuration = \(imageView.image?.symbolConfiguration.map { described($0) } ?? absent)",
                    "preferred_symbol_configuration = \(imageView.preferredSymbolConfiguration.map { described($0) } ?? absent)",
                    "image_rendering_mode = \(imageView.image?.renderingMode.rawValue ?? -1)",
                ]
            }
            if let effectView = view as? UIVisualEffectView {
                fields.append("effect = \(effectView.effect.map { described($0) } ?? absent)")
            }
            if let control = view as? UIControl {
                fields += [
                    "is_enabled_control = \(control.isEnabled)",
                    "is_selected = \(control.isSelected)",
                    "is_highlighted = \(control.isHighlighted)",
                ]
            }
            tables.append(fields.joined(separator: "\n") + "\n\n")
            for (index, child) in view.subviews.enumerated() {
                visit(child, path: "\(path).\(index)", depth: depth + 1)
            }
        }
        visit(root, path: "0", depth: 0)
        return tables
    }

    // MARK: presentations

    func adaptivePresentationStyle(for controller: UIPresentationController) -> UIModalPresentationStyle { .none }

    func adaptivePresentationStyle(
        for controller: UIPresentationController, traitCollection: UITraitCollection
    ) -> UIModalPresentationStyle { .none }

    func presentForScene(_ controller: UIViewController) {
        presentedForScene = controller
        present(controller, animated: false)
    }

    func dismissPresented(_ done: @escaping () -> Void) {
        guard presentedViewController != nil else {
            done()
            return
        }
        dismiss(animated: false) { done() }
    }

    // MARK: the `roles` pass

    private func readInactiveColors() {
        let activeAppearances: [(name: String, value: UIUserInterfaceActiveAppearance)] = [
            ("active", .active), ("inactive", .inactive),
        ]
        let colorClass = UIColor.self as AnyObject
        for name in ["tintColor"] + colorNames.filter({ $0 != "tintColor" }) {
            guard colorClass.responds(to: NSSelectorFromString(name)) == true,
                let color = colorClass.value(forKey: name) as? UIColor
            else { continue }
            for appearance in appearances {
                for active in activeAppearances {
                    let traits = UITraitCollection { traits in
                        traits.userInterfaceStyle = appearance.style
                        traits.activeAppearance = active.value
                    }
                    reads.append(
                        """
                        [[active_appearance_color]]
                        name = \(quoted(name))
                        appearance = \(quoted(appearance.name))
                        active_appearance = \(quoted(active.name))
                        value = \(colorValue(color, traits))

                        """)
                }
            }
        }
    }

    private func readListConfigurations() {
        let backgrounds: [(String, UIBackgroundConfiguration)] = [
            ("list_cell", .listCell()), ("list_plain_cell", .listPlainCell()),
            ("list_grouped_cell", .listGroupedCell()), ("list_sidebar_cell", .listSidebarCell()),
            ("list_accompanied_sidebar_cell", .listAccompaniedSidebarCell()),
        ]
        let contents: [(String, UIListContentConfiguration)] = [
            ("cell", .cell()), ("subtitle_cell", .subtitleCell()), ("value_cell", .valueCell()),
            ("sidebar_cell", .sidebarCell()), ("accompanied_sidebar_cell", .accompaniedSidebarCell()),
        ]
        let states: [(name: String, apply: (inout UICellConfigurationState) -> Void)] = [
            ("normal", { _ in }), ("selected", { $0.isSelected = true }), ("highlighted", { $0.isHighlighted = true }),
            ("selected_highlighted", {
                $0.isSelected = true
                $0.isHighlighted = true
            }),
            ("disabled", { $0.isDisabled = true }),
            ("selected_disabled", {
                $0.isSelected = true
                $0.isDisabled = true
            }),
            ("focused", { $0.isFocused = true }),
        ]
        for appearance in appearances {
            let traits = UITraitCollection(userInterfaceStyle: appearance.style)
            let tint = UIColor.tintColor.resolvedColor(with: traits)
            for (stateName, apply) in states {
                var state = UICellConfigurationState(traitCollection: traits)
                apply(&state)
                for (name, configuration) in backgrounds {
                    let updated = configuration.updated(for: state)
                    reads.append(
                        """
                        [[list_background]]
                        configuration = \(quoted(name))
                        appearance = \(quoted(appearance.name))
                        state = \(quoted(stateName))
                        background_color = \(colorValue(updated.backgroundColor, traits))
                        resolved_background_color = \(colorValue(updated.resolvedBackgroundColor(for: tint), traits))
                        stroke_width_pt = \(number(updated.strokeWidth))
                        resolved_stroke_color = \(colorValue(updated.resolvedStrokeColor(for: tint), traits))
                        corner_radius_pt = \(number(updated.cornerRadius))
                        background_insets_pt = \(insets(updated.backgroundInsets))
                        visual_effect = \(updated.visualEffect.map { described($0) } ?? absent)

                        """)
                }
                for (name, configuration) in contents {
                    var content = configuration
                    content.text = "Text"
                    content.secondaryText = "Secondary"
                    content.image = UIImage(systemName: "star")
                    let updated = content.updated(for: state)
                    reads.append(
                        """
                        [[list_content]]
                        configuration = \(quoted(name))
                        appearance = \(quoted(appearance.name))
                        state = \(quoted(stateName))
                        text_color = \(colorValue(updated.textProperties.resolvedColor(), traits))
                        text_font = \(fontValue(updated.textProperties.font))
                        secondary_text_color = \(colorValue(updated.secondaryTextProperties.resolvedColor(), traits))
                        secondary_text_font = \(fontValue(updated.secondaryTextProperties.font))
                        image_tint = \(colorValue(updated.imageProperties.resolvedTintColor(for: tint), traits))
                        image_preferred_symbol_configuration = \(updated.imageProperties.preferredSymbolConfiguration.map { described($0) } ?? absent)
                        image_reserved_layout_size_pt = \(pair(updated.imageProperties.reservedLayoutSize))
                        image_maximum_size_pt = \(pair(updated.imageProperties.maximumSize))
                        image_to_text_padding_pt = \(number(updated.imageToTextPadding))
                        text_to_secondary_text_vertical_padding_pt = \(number(updated.textToSecondaryTextVerticalPadding))
                        directional_layout_margins_pt = \(insets(updated.directionalLayoutMargins))

                        """)
                }
            }
        }
    }

    private func buttonRow(_ name: String) -> UIView {
        var plain = UIButton.Configuration.plain()
        plain.title = "Button"
        var filled = UIButton.Configuration.filled()
        filled.title = "Button"
        let toggle = UISwitch()
        toggle.isOn = true
        let stack = UIStackView(arrangedSubviews: [UIButton(configuration: plain), UIButton(configuration: filled), toggle])
        stack.axis = .horizontal
        stack.spacing = 4
        stack.alignment = .center
        return stack
    }

    private func buttonRowRead(_ read: SceneRead, key: String) {
        guard let stack = named[key] else { return }
        let traits = stack.traitCollection
        let prefix = key.replacingOccurrences(of: "/", with: "_")
        read.set("\(prefix)_tint", colorValue(stack.tintColor, traits))
        read.set("\(prefix)_tint_adjustment_mode", "\(stack.tintAdjustmentMode.rawValue)")
        read.set("\(prefix)_active_appearance", "\(traits.activeAppearance.rawValue)")
        read.views += tree(stack, label: key, limit: 40)
    }

    private func roleScenes() -> [MeasureScene] {
        var list: [MeasureScene] = []
        for appearance in appearances {
            // The tint, normal and dimmed.
            list.append(
                MeasureScene(
                    name: "tint_dimmed", part: "default", backdrop: "grid", appearance: appearance, settle: settleSeconds,
                    settles: true,
                    setup: { c in
                        c.grid(rows: ["normal", "dimmed"], height: 60) { row, _ in
                            let stack = c.buttonRow(row)
                            stack.tintAdjustmentMode = row == "dimmed" ? .dimmed : .normal
                            return stack
                        }
                    },
                    read: { c, read in
                        read.set("tint_color_class", colorValue(UIColor.tintColor, c.view.traitCollection))
                        for row in ["normal", "dimmed"] { c.buttonRowRead(read, key: "\(row)/system") }
                    }, teardown: nil))
            // The scene active and inactive.
            list.append(
                MeasureScene(
                    name: "window_inactive", part: "default", backdrop: "grid", appearance: appearance,
                    settle: settleSeconds, settles: true,
                    setup: { c in
                        c.grid(rows: ["active", "inactive"], height: 60) { row, _ in
                            let stack = c.buttonRow(row)
                            stack.traitOverrides.activeAppearance = row == "inactive" ? .inactive : .active
                            return stack
                        }
                    },
                    read: { c, read in
                        read.set("window_active_appearance", "\(c.view.traitCollection.activeAppearance.rawValue)")
                        read.set(
                            "scene_activation_state", "\(c.view.window?.windowScene?.activationState.rawValue ?? -1)")
                        for row in ["active", "inactive"] { c.buttonRowRead(read, key: "\(row)/system") }
                    }, teardown: nil))
            // Disabled controls.
            let controls = ["label", "text_field", "segmented", "switch", "slider", "stepper"]
            let rows = controls.flatMap { ["\($0)_normal", "\($0)_disabled"] }
            list.append(
                MeasureScene(
                    name: "disabled", part: "default", backdrop: "grid", appearance: appearance, settle: settleSeconds,
                    settles: true,
                    setup: { c in
                        c.grid(rows: rows, height: c.rowHeight(rows.count, limit: 52), spans: false) { row, _ in
                            let enabled = row.hasSuffix("_normal")
                            let made: UIView
                            if row.hasPrefix("label") {
                                let label = UILabel()
                                label.text = "Label"
                                label.font = .preferredFont(forTextStyle: .body)
                                label.isEnabled = enabled
                                made = label
                            } else if row.hasPrefix("text_field") {
                                let field = UITextField()
                                field.borderStyle = .roundedRect
                                field.text = "Text"
                                field.isEnabled = enabled
                                field.widthAnchor.constraint(equalToConstant: 90).isActive = true
                                made = field
                            } else if row.hasPrefix("segmented") {
                                let segmented = UISegmentedControl(items: ["One", "Two"])
                                segmented.selectedSegmentIndex = 0
                                segmented.isEnabled = enabled
                                made = segmented
                            } else if row.hasPrefix("switch") {
                                let toggle = UISwitch()
                                toggle.isOn = true
                                toggle.isEnabled = enabled
                                made = toggle
                            } else if row.hasPrefix("slider") {
                                let slider = UISlider()
                                slider.value = 0.5
                                slider.isEnabled = enabled
                                slider.widthAnchor.constraint(equalToConstant: 90).isActive = true
                                made = slider
                            } else {
                                let stepper = UIStepper()
                                stepper.value = 1
                                stepper.maximumValue = 2
                                stepper.isEnabled = enabled
                                made = stepper
                            }
                            return made
                        }
                    },
                    read: { c, read in
                        for row in rows {
                            guard let made = c.named["\(row)/system"] else { continue }
                            read.views += c.tree(made, label: row, limit: 30)
                        }
                    }, teardown: nil))
            // List selections, in each list appearance.
            let listAppearances: [(String, UICollectionLayoutListConfiguration.Appearance)] = [
                ("plain", .plain), ("grouped", .grouped), ("inset_grouped", .insetGrouped), ("sidebar", .sidebar),
                ("sidebar_plain", .sidebarPlain),
            ]
            for (listName, listAppearance) in listAppearances {
                list.append(
                    MeasureScene(
                        name: "selection", part: listName, backdrop: "system", appearance: appearance,
                        settle: settleSeconds, settles: true,
                        setup: { c in c.listScene(listAppearance) },
                        read: { c, read in c.listRead(read) }, teardown: nil))
            }
            // Selected text: the text view first responder over each backdrop,
            // then three that are not.
            for fill in measureFills.map({ $0.name }) {
                list.append(
                    MeasureScene(
                        name: "text_selection", part: "focused", backdrop: fill, appearance: appearance, settle: 3,
                        settles: true,
                        setup: { c in c.textScene(fill: fill, find: false) },
                        read: { c, read in c.textRead(read) },
                        teardown: { c, done in
                            c.view.endEditing(true)
                            DispatchQueue.main.asyncAfter(deadline: .now() + 1) { done() }
                        }))
            }
            list.append(
                MeasureScene(
                    name: "text_selection", part: "unfocused", backdrop: "grid", appearance: appearance,
                    settle: settleSeconds, settles: true,
                    setup: { c in
                        c.grid(rows: ["text"], height: 80, spans: true) { _, column in
                            let textView = c.selectionTextView()
                            textView.selectedRange = NSRange(location: 0, length: 8)
                            textView.heightAnchor.constraint(equalToConstant: 60).isActive = true
                            _ = column
                            return textView
                        }
                    },
                    read: { c, read in
                        if let textView = c.named["text/system"] as? UITextView {
                            read.set("is_first_responder", "\(textView.isFirstResponder)")
                            read.set("selected_range", "[\(textView.selectedRange.location), \(textView.selectedRange.length)]")
                            read.views += c.tree(textView, label: "text_view", limit: 60)
                        }
                    }, teardown: nil))
            // UIFindInteraction's highlights.
            for fill in measureFills.map({ $0.name }) {
                list.append(
                    MeasureScene(
                        name: "find", part: "default", backdrop: fill, appearance: appearance, settle: 4, settles: true,
                        setup: { c in c.textScene(fill: fill, find: true) },
                        read: { c, read in c.textRead(read) },
                        teardown: { c, done in
                            if let textView = c.named["text_view"] as? UITextView {
                                textView.findInteraction?.dismissFindNavigator()
                            }
                            c.view.endEditing(true)
                            DispatchQueue.main.asyncAfter(deadline: .now() + 1) { done() }
                        }))
            }
            // An alert, an action sheet, a popover and a sheet, over each backdrop.
            for kind in ["alert", "action_sheet", "popover", "sheet"] {
                for fill in measureFills.map({ $0.name }) {
                    list.append(
                        MeasureScene(
                            name: kind, part: "default", backdrop: fill, appearance: appearance, settle: 3, settles: true,
                            setup: { c in c.presentationScene(kind, fill: fill) },
                            read: { c, read in c.presentationRead(kind, read) },
                            teardown: { c, done in c.dismissPresented(done) }))
                }
            }
        }
        return list
    }

    // MARK: lists

    func listScene(_ appearance: UICollectionLayoutListConfiguration.Appearance) -> [MeasureRegion] {
        let items = ["normal", "selected", "highlighted", "disabled", "selected_disabled"]
        let configuration = UICollectionLayoutListConfiguration(appearance: appearance)
        let layout = UICollectionViewCompositionalLayout.list(using: configuration)
        let frame = CGRect(x: 0, y: stageTop, width: view.bounds.width, height: 420)
        let collection = UICollectionView(frame: frame, collectionViewLayout: layout)
        collection.allowsMultipleSelection = true
        let registration = UICollectionView.CellRegistration<MeasureListCell, String> { cell, _, item in
            var content = cell.defaultContentConfiguration()
            content.text = item
            content.secondaryText = "Secondary"
            content.image = UIImage(systemName: "star")
            cell.contentConfiguration = content
            cell.forcedDisabled = item.hasSuffix("disabled")
            cell.setNeedsUpdateConfiguration()
        }
        let source = UICollectionViewDiffableDataSource<Int, String>(collectionView: collection) {
            collectionView, indexPath, item in
            collectionView.dequeueConfiguredReusableCell(using: registration, for: indexPath, item: item)
        }
        var snapshot = NSDiffableDataSourceSnapshot<Int, String>()
        snapshot.appendSections([0])
        snapshot.appendItems(items)
        source.apply(snapshot, animatingDifferences: false)
        view.addSubview(collection)
        kept.append(source)
        collection.layoutIfNeeded()
        for (index, item) in items.enumerated() {
            let path = IndexPath(item: index, section: 0)
            if item.hasPrefix("selected") {
                collection.selectItem(at: path, animated: false, scrollPosition: [])
            }
            if item == "highlighted" { collection.cellForItem(at: path)?.isHighlighted = true }
        }
        named["collection"] = collection
        var regions = [
            MeasureRegion(name: "list", backdrop: "system", rect: { [weak collection] in
                guard let collection else { return .zero }
                return collection.convert(collection.bounds, to: nil)
            })
        ]
        for (index, item) in items.enumerated() {
            regions.append(
                MeasureRegion(name: item, backdrop: "system", rect: { [weak collection] in
                    guard let collection, let cell = collection.cellForItem(at: IndexPath(item: index, section: 0))
                    else { return .zero }
                    return cell.convert(cell.bounds, to: nil)
                }))
        }
        return regions
    }

    func listRead(_ read: SceneRead) {
        guard let collection = named["collection"] as? UICollectionView else { return }
        read.set("background", colorValue(collection.backgroundColor, collection.traitCollection))
        for index in 0..<collection.numberOfItems(inSection: 0) {
            guard let cell = collection.cellForItem(at: IndexPath(item: index, section: 0)) as? MeasureListCell
            else { continue }
            let traits = cell.traitCollection
            let state = cell.configurationState
            let key = (cell.contentConfiguration as? UIListContentConfiguration)?.text ?? "item_\(index)"
            read.set(
                "\(key)_state",
                "{ selected = \(state.isSelected), highlighted = \(state.isHighlighted), disabled = \(state.isDisabled), focused = \(state.isFocused) }"
            )
            read.set(
                "\(key)_background_configuration",
                cell.backgroundConfiguration.map { colorValue($0.resolvedBackgroundColor(for: cell.tintColor), traits) }
                    ?? absent)
            let fallback = cell.defaultBackgroundConfiguration().updated(for: state)
            read.set(
                "\(key)_default_background_resolved", colorValue(fallback.resolvedBackgroundColor(for: cell.tintColor), traits))
            if let content = cell.contentConfiguration as? UIListContentConfiguration {
                read.set("\(key)_text_color", colorValue(content.textProperties.resolvedColor(), traits))
                read.set("\(key)_secondary_text_color", colorValue(content.secondaryTextProperties.resolvedColor(), traits))
                read.set("\(key)_image_tint", colorValue(content.imageProperties.resolvedTintColor(for: cell.tintColor), traits))
            }
            read.views += tree(cell, label: key, limit: 30)
        }
    }

    // MARK: text

    func selectionTextView() -> UITextView {
        let textView = UITextView()
        textView.text = "Selected text and the rest of it"
        textView.font = .preferredFont(forTextStyle: .body)
        textView.backgroundColor = .clear
        textView.isScrollEnabled = false
        return textView
    }

    func textScene(fill: String, find: Bool) -> [MeasureRegion] {
        let backdrop = cell(x: margin, y: stageTop, width: stageWidth, height: 120, fill: fill)
        let textView = selectionTextView()
        if find {
            textView.text = "Find the word find, and find it again"
            textView.isEditable = false
            textView.isFindInteractionEnabled = true
        }
        textView.frame = backdrop.bounds.insetBy(dx: barInset, dy: barInset)
        backdrop.addSubview(textView)
        named["text_view"] = textView
        textView.becomeFirstResponder()
        if find {
            textView.findInteraction?.searchText = "find"
            textView.findInteraction?.presentFindNavigator(showingReplace: false)
            DispatchQueue.main.asyncAfter(deadline: .now() + 1.5) { [weak textView] in
                textView?.findInteraction?.findNext()
            }
        } else {
            textView.selectedRange = NSRange(location: 0, length: 8)
        }
        return [
            MeasureRegion(name: "text", backdrop: fill, rect: { [weak backdrop] in
                guard let backdrop else { return .zero }
                return backdrop.convert(backdrop.bounds, to: nil)
            })
        ]
    }

    func textRead(_ read: SceneRead) {
        guard let textView = named["text_view"] as? UITextView else { return }
        read.set("is_first_responder", "\(textView.isFirstResponder)")
        read.set("selected_range", "[\(textView.selectedRange.location), \(textView.selectedRange.length)]")
        read.set("tint", colorValue(textView.tintColor, textView.traitCollection))
        read.set("text_color", colorValue(textView.textColor, textView.traitCollection))
        if let interaction = textView.findInteraction {
            read.set("find_navigator_visible", "\(interaction.isFindNavigatorVisible)")
            read.set("find_search_text", interaction.searchText.map(quoted) ?? absent)
            if let session = interaction.activeFindSession {
                read.set("find_result_count", "\(session.resultCount)")
                read.set("find_highlighted_result_index", "\(session.highlightedResultIndex)")
            }
        }
        read.views += tree(textView, label: "text_view", limit: 80)
    }

    // MARK: presentations

    func presentationScene(_ kind: String, fill: String) -> [MeasureRegion] {
        let backdrop = cell(
            x: 0, y: 0, width: view.bounds.width, height: view.bounds.height, fill: fill)
        let row = buttonRow("behind")
        row.translatesAutoresizingMaskIntoConstraints = false
        backdrop.addSubview(row)
        NSLayoutConstraint.activate([
            row.centerXAnchor.constraint(equalTo: backdrop.centerXAnchor),
            row.topAnchor.constraint(equalTo: backdrop.topAnchor, constant: stageTop + 80),
        ])
        named["behind"] = row
        backdrop.layoutIfNeeded()
        let scrim = CGRect(x: margin, y: stageTop, width: stageWidth, height: 56)
        var regions = [
            MeasureRegion(name: "scrim", backdrop: fill, rect: { scrim }),
            MeasureRegion(name: "behind", backdrop: fill, rect: { [weak row] in
                guard let row else { return .zero }
                return row.convert(row.bounds, to: nil).insetBy(dx: -4, dy: -4)
            }),
        ]
        let presented: UIViewController
        switch kind {
        case "alert", "action_sheet":
            let alert = UIAlertController(
                title: "Title", message: "Message", preferredStyle: kind == "alert" ? .alert : .actionSheet)
            alert.addAction(UIAlertAction(title: "Default", style: .default))
            alert.addAction(UIAlertAction(title: "Destructive", style: .destructive))
            alert.addAction(UIAlertAction(title: "Cancel", style: .cancel))
            presented = alert
        case "popover":
            let content = UIViewController()
            let label = UILabel()
            label.text = "Popover"
            label.font = .preferredFont(forTextStyle: .body)
            label.translatesAutoresizingMaskIntoConstraints = false
            content.view.addSubview(label)
            NSLayoutConstraint.activate([
                label.centerXAnchor.constraint(equalTo: content.view.centerXAnchor),
                label.centerYAnchor.constraint(equalTo: content.view.centerYAnchor),
            ])
            content.preferredContentSize = CGSize(width: 220, height: 120)
            content.modalPresentationStyle = .popover
            if let popover = content.popoverPresentationController {
                popover.sourceView = row
                popover.sourceRect = row.bounds
                popover.permittedArrowDirections = .up
                popover.delegate = self
            }
            presented = content
        default:
            let content = UIViewController()
            content.view.backgroundColor = .systemBackground
            let label = UILabel()
            label.text = "Sheet"
            label.font = .preferredFont(forTextStyle: .body)
            label.translatesAutoresizingMaskIntoConstraints = false
            content.view.addSubview(label)
            NSLayoutConstraint.activate([
                label.centerXAnchor.constraint(equalTo: content.view.centerXAnchor),
                label.topAnchor.constraint(equalTo: content.view.topAnchor, constant: 40),
            ])
            content.modalPresentationStyle = .pageSheet
            content.sheetPresentationController?.detents = [.medium()]
            presented = content
        }
        presentForScene(presented)
        regions.append(
            MeasureRegion(name: kind, backdrop: fill, rect: { [weak presented] in
                guard let shown = presented?.view else { return .zero }
                return shown.convert(shown.bounds, to: nil).insetBy(dx: -24, dy: -24)
            }))
        return regions
    }

    func presentationRead(_ kind: String, _ read: SceneRead) {
        if let row = named["behind"] {
            read.set("behind_tint", colorValue(row.tintColor, row.traitCollection))
            read.set("behind_tint_adjustment_mode", "\(row.tintAdjustmentMode.rawValue)")
            read.views += tree(row, label: "behind", limit: 30)
        }
        guard let presented = presentedForScene else { return }
        read.set("presented", quoted(String(describing: type(of: presented))))
        read.set("presented_frame_pt", rectangle(presented.view.convert(presented.view.bounds, to: nil)))
        read.set("modal_presentation_style", "\(presented.modalPresentationStyle.rawValue)")
        if let container = presented.presentationController?.containerView {
            read.views += tree(container, label: "container", limit: 160, depth: 14)
        } else {
            read.views += tree(presented.view, label: "presented", limit: 160, depth: 14)
        }
    }

    // MARK: the `fonts` pass

    private func readFonts() {
        let large = UITraitCollection(preferredContentSizeCategory: .large)
        var styles = textStyles
        styles += [("extra_large_title", .extraLargeTitle), ("extra_large_title2", .extraLargeTitle2)]
        var featured = Set<String>()
        for (name, style) in styles {
            let descriptor = UIFontDescriptor.preferredFontDescriptor(withTextStyle: style, compatibleWith: large)
            let font = UIFont(descriptor: descriptor, size: 0)
            let features = featured.insert(font.fontName).inserted
            reads.append("[[font_style]]\nstyle = \(quoted(name))\ntext_style = \(quoted(style.rawValue))\n"
                + "category = \"large\"\nvariant = \"regular\"\n" + fontDetail(font, features: features) + "\n\n")
            let variants: [(String, UIFontDescriptor.SymbolicTraits)] = [
                ("tight", .traitTightLeading), ("loose", .traitLooseLeading), ("bold", .traitBold),
                ("italic", .traitItalic),
            ]
            for (variantName, trait) in variants {
                guard let changed = descriptor.withSymbolicTraits(descriptor.symbolicTraits.union(trait)) else {
                    reads.append(
                        "[[font_style]]\nstyle = \(quoted(name))\nvariant = \(quoted(variantName))\nexists = false\n\n")
                    continue
                }
                let variantFont = UIFont(descriptor: changed, size: 0)
                let variantFeatures = featured.insert(variantFont.fontName).inserted
                reads.append(
                    "[[font_style]]\nstyle = \(quoted(name))\ntext_style = \(quoted(style.rawValue))\n"
                        + "category = \"large\"\nvariant = \(quoted(variantName))\n"
                        + fontDetail(variantFont, features: variantFeatures) + "\n\n")
            }
        }
        var roles: [(String, UIFont?)] = [
            ("system_font_size", UIFont.systemFont(ofSize: UIFont.systemFontSize)),
            ("small_system_font_size", UIFont.systemFont(ofSize: UIFont.smallSystemFontSize)),
            ("label_font_size", UIFont.systemFont(ofSize: UIFont.labelFontSize)),
            ("button_font_size", UIFont.systemFont(ofSize: UIFont.buttonFontSize)),
            ("bold_system_17", UIFont.boldSystemFont(ofSize: 17)),
            ("italic_system_17", UIFont.italicSystemFont(ofSize: 17)),
            ("monospaced_system_17_regular", UIFont.monospacedSystemFont(ofSize: 17, weight: .regular)),
            ("monospaced_digit_system_17_regular", UIFont.monospacedDigitSystemFont(ofSize: 17, weight: .regular)),
            ("system_17_condensed", UIFont.systemFont(ofSize: 17, weight: .regular, width: .condensed)),
            ("system_17_compressed", UIFont.systemFont(ofSize: 17, weight: .regular, width: .compressed)),
            ("system_17_expanded", UIFont.systemFont(ofSize: 17, weight: .regular, width: .expanded)),
            ("label_default", UILabel().font),
            ("text_field_default", UITextField().font),
            ("text_view_default", UITextView().font),
            ("system_button_title", UIButton(type: .system).titleLabel?.font),
        ]
        let body = UIFontDescriptor.preferredFontDescriptor(withTextStyle: .body, compatibleWith: large)
        let designs: [(String, UIFontDescriptor.SystemDesign)] = [
            ("rounded", .rounded), ("serif", .serif), ("monospaced", .monospaced), ("default", .default),
        ]
        for (designName, design) in designs {
            roles.append(("body_design_\(designName)", body.withDesign(design).map { UIFont(descriptor: $0, size: 0) }))
        }
        reads.append(
            """
            [font_sizes]
            system_font_size_pt = \(number(UIFont.systemFontSize))
            small_system_font_size_pt = \(number(UIFont.smallSystemFontSize))
            label_font_size_pt = \(number(UIFont.labelFontSize))
            button_font_size_pt = \(number(UIFont.buttonFontSize))

            """)
        for (name, font) in roles {
            guard let font else {
                reads.append("[[font_role]]\nsource = \(quoted(name))\nexists = false\n\n")
                continue
            }
            let features = featured.insert(font.fontName).inserted
            reads.append("[[font_role]]\nsource = \(quoted(name))\n" + fontDetail(font, features: features) + "\n\n")
        }
        let bodyFont = UIFont(descriptor: body, size: 0) as CTFont
        let languages: [[String]] = [
            [], ["en"], ["ja"], ["zh-Hans"], ["zh-Hant"], ["zh-HK"], ["ko"], ["ar"], ["he"], ["th"], ["hi"], ["ru"],
            ["el"], ["vi"],
        ]
        for language in languages {
            let list = CTFontCopyDefaultCascadeListForLanguages(bodyFont, language as CFArray)
            let elements: [Any] = list.map { ($0 as NSArray) as [Any] } ?? []
            let names = elements.map { element -> String in
                guard let descriptor = element as? UIFontDescriptor,
                    let name = descriptor.object(forKey: .name) as? String
                else { return described(element) }
                return quoted(name)
            }
            reads.append(
                """
                [[cascade]]
                font = \(quoted(CTFontCopyPostScriptName(bodyFont) as String))
                languages = [\(language.map(quoted).joined(separator: ", "))]
                exists = \(list != nil)
                names = [\(names.joined(separator: ", "))]

                """)
        }
        let strings: [(String, String)] = [
            ("latin", "Text"), ("emoji", "\u{1F600}"), ("japanese", "日本語"), ("chinese", "中文"), ("korean", "한국어"),
            ("arabic", "العربية"), ("hebrew", "עברית"), ("thai", "ไทย"), ("devanagari", "हिन्दी"), ("greek", "Ελληνικά"),
            ("cyrillic", "Русский"), ("symbols", "\u{2318}\u{2325}\u{21E7}"),
        ]
        for (name, text) in strings {
            let length = (text as NSString).length
            let picked = CTFontCreateForString(bodyFont, text as CFString, CFRange(location: 0, length: length))
            reads.append(
                """
                [[font_for_string]]
                script = \(quoted(name))
                text = \(quoted(text))
                font = \(quoted(CTFontCopyPostScriptName(bodyFont) as String))
                picked_postscript_name = \(quoted(CTFontCopyPostScriptName(picked) as String))
                picked_family_name = \(quoted(CTFontCopyFamilyName(picked) as String))
                picked_size_pt = \(number(CTFontGetSize(picked)))

                """)
        }
    }

    private func fontScenes() -> [MeasureScene] {
        var list: [MeasureScene] = []
        for appearance in appearances {
            let rows = ["body", "caption1", "large_title"]
            list.append(
                MeasureScene(
                    name: "text_rendering", part: "default", backdrop: "grid", appearance: appearance,
                    settle: settleSeconds, settles: true,
                    setup: { c in
                        c.grid(rows: rows, height: 60) { row, _ in
                            let label = UILabel()
                            label.text = "Sphinx"
                            let style: UIFont.TextStyle =
                                row == "body" ? .body : row == "caption1" ? .caption1 : .largeTitle
                            label.font = UIFont.preferredFont(forTextStyle: style)
                            return label
                        }
                    },
                    read: { c, read in
                        for row in rows {
                            if let label = c.named["\(row)/system"] { read.views += c.tree(label, label: row, limit: 4) }
                        }
                    }, teardown: nil))
            let styles = configurationStyles().filter {
                ["plain", "gray", "tinted", "filled", "glass", "prominent_glass"].contains($0.name)
            }
            var titleRows = styles.map { $0.name }
            titleRows += ["filled_subtitle", "system", "filled_overlong_word", "filled_long_title", "filled_long_subtitle"]
            let titles = titleRows
            list.append(
                MeasureScene(
                    name: "button_titles", part: "default", backdrop: "grid", appearance: appearance,
                    settle: settleSeconds, settles: true,
                    setup: { c in
                        c.grid(rows: titles, height: c.rowHeight(titles.count, limit: 64)) { row, _ in
                            if row == "system" {
                                let button = UIButton(type: .system)
                                button.setTitle("Button", for: .normal)
                                return button
                            }
                            let styleName = styles.first { row == $0.name }?.name ?? "filled"
                            var configuration = (styles.first { $0.name == styleName }?.make() ?? .filled())
                            configuration.title = "Button"
                            configuration.buttonSize = .medium
                            switch row {
                            case "filled_subtitle": configuration.subtitle = "Subtitle"
                            case "filled_overlong_word": configuration.title = "Supercalifragilisticexpialidocious"
                            case "filled_long_title": configuration.title = "A long title of several words"
                            case "filled_long_subtitle":
                                configuration.subtitle = "A long subtitle of several words"
                            default: break
                            }
                            let button = UIButton(configuration: configuration)
                            if row.hasPrefix("filled_long") || row == "filled_overlong_word" {
                                button.widthAnchor.constraint(lessThanOrEqualToConstant: 100).isActive = true
                            }
                            return button
                        }
                    },
                    read: { c, read in
                        for row in titles {
                            guard let button = c.named["\(row)/system"] as? UIButton else { continue }
                            let label = button.titleLabel
                            read.set("\(row)_title_line_break_mode", label.map { quoted(lineBreakName($0.lineBreakMode)) } ?? absent)
                            read.set("\(row)_title_number_of_lines", label.map { "\($0.numberOfLines)" } ?? absent)
                            read.set("\(row)_title_adjusts_font_size", label.map { "\($0.adjustsFontSizeToFitWidth)" } ?? absent)
                            read.set("\(row)_title_attributes", attributesValue(label?.attributedText))
                            if let configuration = button.configuration?.updated(for: button) {
                                read.set(
                                    "\(row)_configuration_title_line_break_mode",
                                    quoted(lineBreakName(configuration.titleLineBreakMode)))
                                read.set(
                                    "\(row)_configuration_subtitle_line_break_mode",
                                    quoted(lineBreakName(configuration.subtitleLineBreakMode)))
                                read.set(
                                    "\(row)_configuration_attributed_title",
                                    configuration.attributedTitle.map { attributesValue(NSAttributedString($0)) } ?? absent)
                            }
                            read.set("\(row)_frame_pt", rectangle(button.convert(button.bounds, to: nil)))
                            read.views += c.tree(button, label: row, limit: 30)
                        }
                        if let font = (c.named["filled/system"] as? UIButton)?.titleLabel?.font {
                            read.set("filled_title_font_detail", quoted(fontDetail(font, features: false)))
                        }
                    }, teardown: nil))
        }
        return list
    }

    // MARK: the `symbols` pass

    private func readConfigurationDefaults() {
        for style in configurationStyles() {
            let configuration = style.make()
            reads.append(
                """
                [[configuration_default]]
                style = \(quoted(style.name))
                source = "a new configuration (UIButton.Configuration.<style>()), no property set"
                button_size = \(quoted(sizeName(configuration.buttonSize)))
                corner_style = \(quoted(cornerStyleName(configuration.cornerStyle)))
                content_insets_pt = \(insets(configuration.contentInsets))
                image_padding_pt = \(number(configuration.imagePadding))
                title_padding_pt = \(number(configuration.titlePadding))
                image_placement = \(quoted(edgeName(configuration.imagePlacement)))
                title_alignment = \(quoted(titleAlignmentName(configuration.titleAlignment)))
                indicator = \(quoted(indicatorName(configuration.indicator)))
                title_line_break_mode = \(quoted(lineBreakName(configuration.titleLineBreakMode)))
                subtitle_line_break_mode = \(quoted(lineBreakName(configuration.subtitleLineBreakMode)))
                automatically_update_for_selection = \(configuration.automaticallyUpdateForSelection)
                preferred_symbol_configuration = \(configuration.preferredSymbolConfigurationForImage.map { described($0) } ?? absent)

                """)
        }
        let button = UIButton(configuration: .plain())
        reads.append(
            """
            [[configuration_default]]
            style = "plain_in_button"
            source = "UIButton(configuration: .plain()).configuration, no property set"
            button_size = \(button.configuration.map { quoted(sizeName($0.buttonSize)) } ?? absent)

            """)
    }

    /// A button's image as UIKit resolved it, and the image views it draws.
    private func imageRead(_ button: UIButton) -> String {
        var fields = [
            "current_image = \(button.currentImage.map { described($0) } ?? absent)",
            "current_image_size_pt = \(button.currentImage.map { pair($0.size) } ?? absent)",
            "current_image_symbol_configuration = \(button.currentImage?.symbolConfiguration.map { described($0) } ?? absent)",
            "current_preferred_symbol_configuration = \(button.currentPreferredSymbolConfiguration.map { described($0) } ?? absent)",
        ]
        if let configuration = button.configuration?.updated(for: button) {
            fields.append(
                "configuration_preferred_symbol_configuration = \(configuration.preferredSymbolConfigurationForImage.map { described($0) } ?? absent)"
            )
            fields.append("configuration_image = \(configuration.image.map { described($0) } ?? absent)")
        }
        var views: [String] = []
        func visit(_ view: UIView) {
            if let imageView = view as? UIImageView, let image = imageView.image {
                views.append(
                    "{ frame_pt = \(rectangle(imageView.convert(imageView.bounds, to: button))), image = \(described(image)), "
                        + "size_pt = \(pair(image.size)), symbol_configuration = \(image.symbolConfiguration.map { described($0) } ?? absent), "
                        + "preferred_symbol_configuration = \(imageView.preferredSymbolConfiguration.map { described($0) } ?? absent), "
                        + "tint = \(colorValue(imageView.tintColor, imageView.traitCollection)), "
                        + "rendering_mode = \(image.renderingMode.rawValue) }")
            }
            for child in view.subviews { visit(child) }
        }
        visit(button)
        fields.append("image_views = [\(views.joined(separator: ", "))]")
        fields.append("frame_pt = \(rectangle(button.bounds))")
        return fields.joined(separator: "\n")
    }

    private func readButtonSymbols() {
        let holder = UIView(frame: CGRect(x: 0, y: 0, width: view.bounds.width, height: 200))
        holder.alpha = 0
        view.addSubview(holder)
        var samples: [(String, String, String, UIButton)] = []
        let menu = UIMenu(children: [UIAction(title: "Item") { _ in }])
        for style in configurationStyles() {
            for size in sizes {
                var configuration = style.make()
                configuration.title = "Button"
                configuration.image = UIImage(systemName: "star")
                configuration.buttonSize = size.size
                samples.append((style.name, size.name, "image", UIButton(configuration: configuration)))
                var iconOnly = style.make()
                iconOnly.image = UIImage(systemName: "star")
                iconOnly.buttonSize = size.size
                samples.append((style.name, size.name, "icon_only", UIButton(configuration: iconOnly)))
                var popup = style.make()
                popup.title = "Button"
                popup.buttonSize = size.size
                popup.indicator = .popup
                let popupButton = UIButton(configuration: popup)
                popupButton.menu = menu
                popupButton.showsMenuAsPrimaryAction = true
                samples.append((style.name, size.name, "indicator_popup", popupButton))
                let changes = UIButton(configuration: popup)
                changes.menu = UIMenu(children: [
                    UIAction(title: "One", state: .on) { _ in }, UIAction(title: "Two") { _ in },
                ])
                changes.showsMenuAsPrimaryAction = true
                changes.changesSelectionAsPrimaryAction = true
                samples.append((style.name, size.name, "pop_up", changes))
            }
        }
        let types: [(String, UIButton.ButtonType)] = [
            ("system", .system), ("info_light", .infoLight), ("info_dark", .infoDark),
            ("detail_disclosure", .detailDisclosure), ("contact_add", .contactAdd), ("close", .close),
        ]
        for (name, type) in types {
            let button = UIButton(type: type)
            if type == .system { button.setImage(UIImage(systemName: "star"), for: .normal) }
            samples.append((name, "default", type == .system ? "image" : "type", button))
        }
        for sample in samples {
            holder.addSubview(sample.3)
            sample.3.sizeToFit()
        }
        holder.layoutIfNeeded()
        for (style, size, variant, button) in samples {
            button.layoutIfNeeded()
            reads.append(
                "[[button_symbol]]\nstyle = \(quoted(style))\nsize = \(quoted(size))\nvariant = \(quoted(variant))\n"
                    + imageRead(button) + "\n\n")
        }
        holder.removeFromSuperview()
    }

    private func symbolScenes() -> [MeasureScene] {
        appearances.map { appearance in
            MeasureScene(
                name: "symbol_contexts", part: "default", backdrop: "system", appearance: appearance,
                settle: settleSeconds, settles: true,
                setup: { c in c.symbolContexts() },
                read: { c, read in
                    for name in ["toolbar", "navigation_bar", "tab_bar", "segmented", "search_bar", "list"] {
                        if let made = c.named[name] { read.views += c.tree(made, label: name, limit: 80, depth: 12) }
                    }
                }, teardown: nil)
        }
    }

    func symbolContexts() -> [MeasureRegion] {
        let star = UIImage(systemName: "star") ?? UIImage()
        var y = stageTop
        var regions: [MeasureRegion] = []
        func add(_ name: String, _ made: UIView, height: CGFloat) {
            made.frame = CGRect(x: 0, y: y, width: view.bounds.width, height: height)
            view.addSubview(made)
            named[name] = made
            regions.append(
                MeasureRegion(name: name, backdrop: "system", rect: { [weak made] in
                    guard let made else { return .zero }
                    return made.convert(made.bounds, to: nil)
                }))
            y += height + gap
        }
        let toolbar = UIToolbar()
        toolbar.items = [
            UIBarButtonItem(primaryAction: UIAction(image: star) { _ in }), .flexibleSpace(),
            UIBarButtonItem(systemItem: .action, primaryAction: UIAction { _ in }), .flexibleSpace(),
            UIBarButtonItem(systemItem: .add, primaryAction: UIAction { _ in }),
        ]
        add("toolbar", toolbar, height: 50)
        let bar = UINavigationBar()
        let previous = UINavigationItem(title: "Previous")
        let top = UINavigationItem(title: "Title")
        top.rightBarButtonItem = UIBarButtonItem(primaryAction: UIAction(image: star) { _ in })
        bar.setItems([previous, top], animated: false)
        add("navigation_bar", bar, height: 50)
        let tabBar = UITabBar()
        tabBar.items = [
            UITabBarItem(title: "Star", image: star, tag: 0),
            UITabBarItem(title: "Circle", image: UIImage(systemName: "circle"), tag: 1),
        ]
        tabBar.selectedItem = tabBar.items?.first
        add("tab_bar", tabBar, height: 60)
        let segmented = UISegmentedControl(items: [star as Any, "Text"])
        segmented.selectedSegmentIndex = 0
        let segmentedHolder = UIView()
        segmentedHolder.addSubview(segmented)
        segmented.frame = CGRect(x: margin, y: 4, width: 200, height: 32)
        add("segmented", segmentedHolder, height: 40)
        let search = UISearchBar()
        search.text = "Text"
        add("search_bar", search, height: 56)
        let configuration = UICollectionLayoutListConfiguration(appearance: .sidebar)
        let collection = UICollectionView(
            frame: .zero, collectionViewLayout: UICollectionViewCompositionalLayout.list(using: configuration))
        let registration = UICollectionView.CellRegistration<UICollectionViewListCell, String> { cell, _, item in
            var content = item == "sidebar" ? UIListContentConfiguration.sidebarCell() : cell.defaultContentConfiguration()
            content.text = item
            content.image = star
            cell.contentConfiguration = content
        }
        let source = UICollectionViewDiffableDataSource<Int, String>(collectionView: collection) {
            collectionView, indexPath, item in
            collectionView.dequeueConfiguredReusableCell(using: registration, for: indexPath, item: item)
        }
        var snapshot = NSDiffableDataSourceSnapshot<Int, String>()
        snapshot.appendSections([0])
        snapshot.appendItems(["cell", "sidebar"])
        source.apply(snapshot, animatingDifferences: false)
        kept.append(source)
        add("list", collection, height: 120)
        return regions
    }
}
