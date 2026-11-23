// The labelled switch on macOS (the 0.8.0 plan, M2.FS4): SwiftUI's
// `Toggle("Label", isOn:)` with `.toggleStyle(.switch)`, which AppKit has no
// counterpart of (NSSwitch has no title) and the Rust probe cannot make
// (SwiftUI has no Objective-C API). Built and run by os-probe.yml's
// switch-label job:
//
//     swiftc -parse-as-library -O tools/swiftui-host/SwitchLabel.swift -o switch-label
//     ./switch-label <out-dir>
//
// One titled window (empty title), key, the application active, its content
// a flipped view holding one NSHostingView per item at a known frame, each
// item leading-aligned and vertically centred in it: the Toggle off, on, off
// `.disabled(true)` and on `.disabled(true)` (each bound to a constant, so a
// click does not turn it); for comparison, `Text("Label")` in the
// foreground styles primary, secondary, tertiary and quaternary and at the
// system font of 11 to 15 points, and AppKit's `NSTextField(labelWithString:)`
// enabled and disabled. Per appearance (aqua, darkAqua; the window's
// appearance set, then 1.5 s): the window captured at rest (`screencapture -x
// -o -l <windowNumber>`); each Toggle's switch (the NSSwitch its view tree
// holds) hovered by a posted mouse-moved CGEvent at its centre, captured 1 s
// later; and pressed by a posted left-mouse-down there, captured 0.6 s later
// while held, then released; the cursor parked right of the window between.
// Writes <out-dir>/switch-label.toml (the environment, the system colours in
// sRGB per appearance, each item's frame and view tree in the window frame's
// points from its top left, each capture) and <out-dir>/<appearance>/<state>.png.
// Exits 1 where a step failed (each failure is listed in the TOML).

import AppKit
import SwiftUI

final class Flipped: NSView {
    override var isFlipped: Bool { true }
}

struct Item {
    let name: String
    let view: NSView
}

func quoted(_ text: String) -> String {
    "\"" + text.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "\"", with: "\\\"") + "\""
}

func number(_ value: CGFloat) -> String { String(format: "%.3f", Double(value)) }

func rectText(_ rect: CGRect) -> String {
    "[\(number(rect.minX)), \(number(rect.minY)), \(number(rect.width)), \(number(rect.height))]"
}

func srgb(_ color: NSColor) -> String {
    guard let converted = color.usingColorSpace(.sRGB) else { return quoted("unconvertible") }
    let parts = [converted.redComponent, converted.greenComponent, converted.blueComponent, converted.alphaComponent]
    return quoted("#" + parts.map { String(format: "%02x", Int((min(max($0, 0), 1) * 255).rounded())) }.joined())
}

let itemWidth: CGFloat = 280
let itemHeight: CGFloat = 40
let itemGap: CGFloat = 4
let inset: CGFloat = 20

@MainActor
func hosted<Content: View>(_ content: Content) -> NSView {
    NSHostingView(rootView: content.fixedSize().frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .leading))
}

@MainActor
func makeItems() -> [Item] {
    var items: [Item] = [
        Item(name: "toggle_off", view: hosted(Toggle("Label", isOn: .constant(false)).toggleStyle(.switch))),
        Item(name: "toggle_on", view: hosted(Toggle("Label", isOn: .constant(true)).toggleStyle(.switch))),
        Item(
            name: "toggle_off_disabled",
            view: hosted(Toggle("Label", isOn: .constant(false)).toggleStyle(.switch).disabled(true))),
        Item(
            name: "toggle_on_disabled",
            view: hosted(Toggle("Label", isOn: .constant(true)).toggleStyle(.switch).disabled(true))),
        Item(name: "text_primary", view: hosted(Text("Label").foregroundStyle(.primary))),
        Item(name: "text_secondary", view: hosted(Text("Label").foregroundStyle(.secondary))),
        Item(name: "text_tertiary", view: hosted(Text("Label").foregroundStyle(.tertiary))),
        Item(name: "text_quaternary", view: hosted(Text("Label").foregroundStyle(.quaternary))),
    ]
    for size in 11...15 {
        items.append(Item(name: "text_size_\(size)", view: hosted(Text("Label").font(.system(size: CGFloat(size))))))
    }
    let enabled = NSTextField(labelWithString: "Label")
    let disabled = NSTextField(labelWithString: "Label")
    disabled.isEnabled = false
    items.append(Item(name: "label_field", view: enabled))
    items.append(Item(name: "label_field_disabled", view: disabled))
    return items
}

/// The views of `root`'s subtree, depth first, at most 40 and 10 deep, in
/// the window frame's points from its top left.
@MainActor
func tree(_ root: NSView, window: NSWindow) -> [String] {
    var lines: [String] = []
    func visit(_ view: NSView, path: String, depth: Int) {
        guard lines.count < 40, depth <= 10 else { return }
        let frame = topLeft(view.convert(view.bounds, to: nil), window: window)
        var line = "{ path = \(quoted(path)), class = \(quoted(String(describing: type(of: view)))), frame_pt = \(rectText(frame))"
        if let toggle = view as? NSSwitch {
            line += ", switch_state = \(toggle.state.rawValue), enabled = \(toggle.isEnabled)"
        }
        if let field = view as? NSTextField {
            line += ", font_pt = \(number(field.font?.pointSize ?? 0)), font = \(quoted(field.font?.fontName ?? "absent"))"
            line += ", text_color = \(field.textColor.map(srgb) ?? quoted("absent"))"
        }
        lines.append(line + " }")
        for (index, child) in view.subviews.enumerated() {
            visit(child, path: "\(path).\(index)", depth: depth + 1)
        }
    }
    visit(root, path: "0", depth: 0)
    return lines
}

/// A rect in window coordinates as the window frame's points from its top
/// left (the capture's coordinates, the title bar included).
@MainActor
func topLeft(_ rect: CGRect, window: NSWindow) -> CGRect {
    CGRect(x: rect.minX, y: window.frame.height - rect.maxY, width: rect.width, height: rect.height)
}

@MainActor
func firstSwitch(_ root: NSView) -> NSSwitch? {
    if let found = root as? NSSwitch { return found }
    for child in root.subviews {
        if let found = firstSwitch(child) { return found }
    }
    return nil
}

/// The CG global point (the main display's top left, y down) of a rect's
/// centre in `window`.
@MainActor
func globalCentre(_ rect: CGRect, window: NSWindow) -> CGPoint {
    let screen = window.convertToScreen(rect)
    let height = NSScreen.screens.first?.frame.height ?? 0
    return CGPoint(x: screen.midX, y: height - screen.midY)
}

func post(_ type: CGEventType, at point: CGPoint) -> Bool {
    guard let event = CGEvent(mouseEventSource: nil, mouseType: type, mouseCursorPosition: point, mouseButton: .left)
    else { return false }
    event.post(tap: .cghidEventTap)
    return true
}

func capture(windowNumber: Int, to path: String) -> String? {
    let process = Process()
    process.executableURL = URL(fileURLWithPath: "/usr/sbin/screencapture")
    process.arguments = ["-x", "-o", "-l", String(windowNumber), path]
    do {
        try process.run()
    } catch {
        return "screencapture: \(error)"
    }
    process.waitUntilExit()
    if process.terminationStatus != 0 { return "screencapture exit \(process.terminationStatus)" }
    if !FileManager.default.fileExists(atPath: path) { return "screencapture wrote no \(path)" }
    return nil
}

func onMain<T: Sendable>(_ work: @MainActor () -> T) -> T {
    DispatchQueue.main.sync { MainActor.assumeIsolated { work() } }
}

final class Host: NSObject, NSApplicationDelegate {
    let out: String
    var window: NSWindow?
    var items: [Item] = []

    init(out: String) { self.out = out }

    func applicationDidFinishLaunching(_ notification: Notification) {
        let items = makeItems()
        let height = inset * 2 + CGFloat(items.count) * (itemHeight + itemGap)
        let window = NSWindow(
            contentRect: CGRect(x: 200, y: 120, width: itemWidth + 2 * inset, height: height),
            styleMask: [.titled, .closable, .miniaturizable, .resizable], backing: .buffered, defer: false)
        window.title = ""
        window.isReleasedWhenClosed = false
        let content = Flipped(frame: CGRect(x: 0, y: 0, width: itemWidth + 2 * inset, height: height))
        for (index, item) in items.enumerated() {
            item.view.frame = CGRect(
                x: inset, y: inset + CGFloat(index) * (itemHeight + itemGap), width: itemWidth, height: itemHeight)
            if let field = item.view as? NSTextField {
                field.sizeToFit()
                field.frame.origin.y = inset + CGFloat(index) * (itemHeight + itemGap) + (itemHeight - field.frame.height) / 2
            }
            content.addSubview(item.view)
        }
        window.contentView = content
        self.window = window
        self.items = items
        NSApp.setActivationPolicy(.regular)
        NSApp.activate(ignoringOtherApps: true)
        window.makeKeyAndOrderFront(nil)
        Thread.detachNewThread { [self] in
            let failed = self.run()
            exit(failed ? 1 : 0)
        }
    }

    /// The pass, off the main thread (the main thread keeps running the
    /// application, AppKit's mouse tracking included); true where a step failed.
    nonisolated func run() -> Bool {
        var failures: [String] = []
        var notes: [String] = []
        var captures: [String] = []
        var keyed = false
        for _ in 0..<50 {
            keyed = onMain {
                if !NSApp.isActive { NSApp.activate(ignoringOtherApps: true) }
                self.window?.makeKeyAndOrderFront(nil)
                return NSApp.isActive && (self.window?.isKeyWindow ?? false)
            }
            if keyed { break }
            Thread.sleep(forTimeInterval: 0.2)
        }
        if !keyed { failures.append("the window did not become key in an active application within 10 s") }
        let manager = FileManager.default
        let (windowNumber, park, environment, colours, itemLines) = onMain {
            () -> (Int, CGPoint, String, String, String) in
            guard let window = self.window else { return (0, .zero, "", "", "") }
            let parkRect = CGRect(x: window.frame.width + 150, y: window.frame.height / 2, width: 1, height: 1)
            let info = ProcessInfo.processInfo
            let environment = """
                [environment]
                os = \(quoted(info.operatingSystemVersionString))
                backing_scale = \(number(window.backingScaleFactor))
                window_frame_pt = \(rectText(window.frame))
                screen_frame_pt = \(rectText(NSScreen.screens.first?.frame ?? .zero))
                active = \(NSApp.isActive)
                key = \(window.isKeyWindow)
                item_size_pt = [\(number(itemWidth)), \(number(itemHeight))]

                """
            var colours = ""
            for name in [NSAppearance.Name.aqua, .darkAqua] {
                guard let appearance = NSAppearance(named: name) else { continue }
                appearance.performAsCurrentDrawingAppearance {
                    colours += """
                        [[colours]]
                        appearance = \(quoted(name.rawValue))
                        label = \(srgb(.labelColor))
                        secondary_label = \(srgb(.secondaryLabelColor))
                        tertiary_label = \(srgb(.tertiaryLabelColor))
                        quaternary_label = \(srgb(.quaternaryLabelColor))
                        control_text = \(srgb(.controlTextColor))
                        disabled_control_text = \(srgb(.disabledControlTextColor))
                        window_background = \(srgb(.windowBackgroundColor))

                        """
                }
            }
            var itemLines = ""
            for item in self.items {
                let frame = topLeft(item.view.convert(item.view.bounds, to: nil), window: window)
                let toggle = firstSwitch(item.view)
                let switchFrame = toggle.map { topLeft($0.convert($0.bounds, to: nil), window: window) }
                itemLines += """
                    [[item]]
                    name = \(quoted(item.name))
                    frame_pt = \(rectText(frame))
                    fitting_size_pt = [\(number(item.view.fittingSize.width)), \(number(item.view.fittingSize.height))]
                    switch_frame_pt = \(switchFrame.map(rectText) ?? quoted("absent"))
                    views = [
                    \(tree(item.view, window: window).joined(separator: ",\n"))
                    ]

                    """
            }
            return (window.windowNumber, globalCentre(parkRect, window: window), environment, colours, itemLines)
        }
        _ = post(.mouseMoved, at: park)
        for name in [NSAppearance.Name.aqua, .darkAqua] {
            let folder = "\(out)/\(name.rawValue)"
            do {
                try manager.createDirectory(atPath: folder, withIntermediateDirectories: true)
            } catch {
                failures.append("create \(folder): \(error)")
            }
            onMain {
                self.window?.appearance = NSAppearance(named: name)
                self.window?.display()
            }
            Thread.sleep(forTimeInterval: 1.5)
            func shoot(_ state: String, target: String, point: CGPoint?, held: Bool) {
                let path = "\(folder)/\(state).png"
                let (active, key) = onMain { (NSApp.isActive, self.window?.isKeyWindow ?? false) }
                if let reason = capture(windowNumber: windowNumber, to: path) { failures.append("\(state): \(reason)") }
                let at = point.map { "[\(number($0.x)), \(number($0.y))]" } ?? quoted("parked")
                captures.append(
                    """
                    [[capture]]
                    appearance = \(quoted(name.rawValue))
                    state = \(quoted(state))
                    target = \(quoted(target))
                    file = \(quoted("\(name.rawValue)/\(state).png"))
                    mouse_global_pt = \(at)
                    button_held = \(held)
                    active = \(active)
                    key = \(key)

                    """)
            }
            shoot("normal", target: "none", point: nil, held: false)
            for target in ["toggle_off", "toggle_on"] {
                let centre: CGPoint? = onMain {
                    guard let window = self.window, let item = self.items.first(where: { $0.name == target }),
                        let toggle = firstSwitch(item.view)
                    else { return nil }
                    return globalCentre(toggle.convert(toggle.bounds, to: nil), window: window)
                }
                guard let centre else {
                    notes.append("\(name.rawValue) \(target): no NSSwitch in its view tree, not hovered or pressed")
                    continue
                }
                if !post(.mouseMoved, at: centre) { failures.append("\(target): mouse-moved event not made") }
                Thread.sleep(forTimeInterval: 1.0)
                shoot("hovered_\(target)", target: target, point: centre, held: false)
                if !post(.leftMouseDown, at: centre) { failures.append("\(target): mouse-down event not made") }
                Thread.sleep(forTimeInterval: 0.6)
                shoot("pressed_\(target)", target: target, point: centre, held: true)
                if !post(.leftMouseUp, at: centre) { failures.append("\(target): mouse-up event not made") }
                Thread.sleep(forTimeInterval: 0.3)
                _ = post(.mouseMoved, at: park)
                Thread.sleep(forTimeInterval: 0.8)
            }
        }
        let text = environment + colours + itemLines + captures.joined(separator: "\n")
            + "notes = [\(notes.map(quoted).joined(separator: ", "))]\n"
            + "failures = [\(failures.map(quoted).joined(separator: ", "))]\n"
        do {
            try text.write(toFile: "\(out)/switch-label.toml", atomically: true, encoding: .utf8)
        } catch {
            failures.append("write switch-label.toml: \(error)")
        }
        for failure in failures { print("failure: \(failure)") }
        return !failures.isEmpty
    }
}

@main
struct SwitchLabel {
    static func main() {
        let arguments = CommandLine.arguments
        guard arguments.count == 2 else {
            print("usage: switch-label <out-dir>")
            exit(2)
        }
        do {
            try FileManager.default.createDirectory(atPath: arguments[1], withIntermediateDirectories: true)
        } catch {
            print("create \(arguments[1]): \(error)")
            exit(1)
        }
        let app = NSApplication.shared
        let host = Host(out: arguments[1])
        app.delegate = host
        app.run()
    }
}
