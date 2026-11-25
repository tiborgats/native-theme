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
// item leading-aligned and vertically centred in it: SwiftUI's
// `DisclosureGroup("Label", isExpanded:)` collapsed (bound to a constant, so
// a click does not expand it; the expander's header); the Toggle off, on, off
// `.disabled(true)` and on `.disabled(true)` (each bound to a constant, so a
// click does not turn it); for comparison, `Text("Label")` in the
// foreground styles primary, secondary, tertiary and quaternary and at the
// system font of 11 to 15 points, and AppKit's `NSTextField(labelWithString:)`
// enabled and disabled. Per appearance (aqua, darkAqua; the window's
// appearance set, then 1.5 s): the window captured at rest (`screencapture -x
// -o -l <windowNumber>`); each Toggle's switch (the NSSwitch its view tree
// holds) hovered by a posted mouse-moved CGEvent at its centre, captured 1 s
// later; and pressed by a posted left-mouse-down there, captured 0.6 s later
// while held, then released; the DisclosureGroup's chevron and its label
// likewise, each hovered and pressed at its centre as the rest capture shows
// it (disclosureParts, recorded as [[disclosure_parts]]); the cursor parked
// between on the middle of the content view's left margin (`inset / 2` from
// its left edge, half its height down, left of every item), where hitTest:
// gives the content view itself. Every event is posted only where
// `NSWindow.windowNumber(at:belowWindowWithWindowNumber: 0)` at its point,
// read just before, gives the host's window; else that target (or the park)
// is a failure and nothing more is posted. The hover and press section runs
// only in a key window of an active application. The window accepts
// mouse-moved events, and a local event monitor (it returns each event
// unchanged) records every mouse-moved, -entered, -exited, left-down and
// left-up event the application receives: its type, window number, point
// and the class of the view hitTest: finds there; each capture lists those
// received since the post before it (`events`), and each release those after
// the mouse-up ([[release]]).
// Writes <out-dir>/switch-label.toml (the environment, the accessibility
// settings, the system colours in sRGB per appearance, each item's frame and
// view tree in the window frame's points from its top left, each capture) and
// <out-dir>/<appearance>/<state>.png. Exits 1 where a step failed (each
// failure is listed in the TOML).

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
let itemHeight: CGFloat = 34
let itemGap: CGFloat = 4
let inset: CGFloat = 20

@MainActor
func hosted<Content: View>(_ content: Content) -> NSView {
    NSHostingView(rootView: content.fixedSize().frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .leading))
}

@MainActor
func makeItems() -> [Item] {
    var items: [Item] = [
        Item(
            name: "disclosure",
            view: hosted(DisclosureGroup("Label", isExpanded: .constant(false)) { Text("Content") })),
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
        } else if let button = view as? NSButton {
            line += ", bezel_style = \(button.bezelStyle.rawValue), button_state = \(button.state.rawValue)"
        }
        if let field = view as? NSTextField {
            line += ", font_pt = \(field.font.map { number($0.pointSize) } ?? quoted("absent"))"
            line += ", font = \(quoted(field.font?.fontName ?? "absent"))"
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
func firstButton(_ root: NSView) -> NSButton? {
    if let found = root as? NSButton, !(root is NSSwitch) { return found }
    for child in root.subviews {
        if let found = firstButton(child) { return found }
    }
    return nil
}

@MainActor
func firstSwitch(_ root: NSView) -> NSSwitch? {
    if let found = root as? NSSwitch { return found }
    for child in root.subviews {
        if let found = firstSwitch(child) { return found }
    }
    return nil
}

/// The AppKit screen point (y up) of a rect's centre in `window`.
@MainActor
func screenCentre(_ rect: CGRect, window: NSWindow) -> CGPoint {
    let screen = window.convertToScreen(rect)
    return CGPoint(x: screen.midX, y: screen.midY)
}

/// An AppKit screen point as a CG global point (the main display's top left,
/// y down); nil where there is no screen.
@MainActor
func quartz(_ point: CGPoint) -> CGPoint? {
    guard let height = NSScreen.screens.first?.frame.height else { return nil }
    return CGPoint(x: point.x, y: height - point.y)
}

func post(_ type: CGEventType, at point: CGPoint) -> Bool {
    guard let event = CGEvent(mouseEventSource: nil, mouseType: type, mouseCursorPosition: point, mouseButton: .left)
    else { return false }
    event.post(tap: .cghidEventTap)
    return true
}

func eventName(_ type: NSEvent.EventType) -> String {
    switch type {
    case .mouseMoved: return "mouseMoved"
    case .mouseEntered: return "mouseEntered"
    case .mouseExited: return "mouseExited"
    case .leftMouseDown: return "leftMouseDown"
    case .leftMouseUp: return "leftMouseUp"
    default: return "type \(type.rawValue)"
    }
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
    var monitor: Any?
    /// The mouse events received since the last `drain`, each an inline
    /// table; read and written on the main thread only.
    var events: [String] = []

    init(out: String) { self.out = out }

    /// Records `event` (the local monitor's): its type, window number and,
    /// in the host's window, its point in the window frame's points from its
    /// top left and the class of the view hitTest: finds there.
    @MainActor
    func record(_ event: NSEvent) {
        var line = "{ type = \(quoted(eventName(event.type))), window_number = \(event.windowNumber)"
        if let window, event.windowNumber == window.windowNumber {
            let at = event.locationInWindow
            let hit = window.contentView?.superview?.hitTest(at)
            line += ", location_pt = [\(number(at.x)), \(number(window.frame.height - at.y))]"
            line += ", hit = \(quoted(hit.map { String(describing: type(of: $0)) } ?? "absent"))"
        } else {
            line += ", location_pt = \(quoted("absent: not in the host's window")), hit = \(quoted("absent"))"
        }
        events.append(line + " }")
    }

    /// An AppKit screen point in the window frame's points from its top left.
    @MainActor
    func frameTopLeft(_ screen: CGPoint, window: NSWindow) -> CGPoint {
        let base = window.convertPoint(fromScreen: screen)
        return CGPoint(x: base.x, y: window.frame.height - base.y)
    }

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
        window.acceptsMouseMovedEvents = true
        self.window = window
        self.items = items
        monitor = NSEvent.addLocalMonitorForEvents(
            matching: [.mouseMoved, .mouseEntered, .mouseExited, .leftMouseDown, .leftMouseUp]
        ) { [weak self] event in
            MainActor.assumeIsolated { self?.record(event) }
            return event
        }
        NSApp.setActivationPolicy(.regular)
        NSApp.activate(ignoringOtherApps: true)
        window.makeKeyAndOrderFront(nil)
        Thread.detachNewThread { [self] in
            let failed = self.run()
            exit(failed ? 1 : 0)
        }
    }

    /// The DisclosureGroup's chevron and label in the window frame's points
    /// from its top left, found in the rest capture (`png`): in the item's
    /// rect, the columns holding a pixel more than 24 levels (summed over
    /// r, g, b) from the item's right-end pixel; the first run of such
    /// columns is the chevron, the columns after the first gap of 2 or more
    /// pixels the label; each part's rows are its inked rows.
    @MainActor
    func disclosureParts(_ png: String) -> (chevron: CGRect, label: CGRect)? {
        guard let window, let item = items.first(where: { $0.name == "disclosure" }),
            let source = CGImageSourceCreateWithURL(URL(fileURLWithPath: png) as CFURL, nil),
            let image = CGImageSourceCreateImageAtIndex(source, 0, nil),
            let space = CGColorSpace(name: CGColorSpace.sRGB),
            let context = CGContext(
                data: nil, width: image.width, height: image.height, bitsPerComponent: 8,
                bytesPerRow: image.width * 4, space: space, bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)
        else { return nil }
        context.draw(image, in: CGRect(x: 0, y: 0, width: image.width, height: image.height))
        guard let data = context.data else { return nil }
        let bytes = data.bindMemory(to: UInt8.self, capacity: image.width * image.height * 4)
        let scale = window.backingScaleFactor
        let rect = topLeft(item.view.convert(item.view.bounds, to: nil), window: window)
        let x0 = Int(rect.minX * scale), x1 = min(image.width, Int(rect.maxX * scale))
        let y0 = Int(rect.minY * scale), y1 = min(image.height, Int(rect.maxY * scale))
        guard x1 - 2 > x0, y1 > y0 else { return nil }
        func pixel(_ x: Int, _ y: Int) -> [Int] {
            let offset = (y * image.width + x) * 4
            return [Int(bytes[offset]), Int(bytes[offset + 1]), Int(bytes[offset + 2])]
        }
        let background = pixel(x1 - 2, (y0 + y1) / 2)
        func inked(_ x: Int, _ y: Int) -> Bool { zip(pixel(x, y), background).map { abs($0 - $1) }.reduce(0, +) > 24 }
        let columns = (x0..<x1).filter { x in (y0..<y1).contains { inked(x, $0) } }
        guard let first = columns.first else { return nil }
        var chevronEnd = first
        var labelStart: Int?
        for x in columns.dropFirst() {
            if labelStart == nil && x - chevronEnd <= 2 { chevronEnd = x } else if labelStart == nil { labelStart = x }
        }
        guard let labelStart, let labelEnd = columns.last else { return nil }
        func part(_ from: Int, _ to: Int) -> CGRect {
            let rows = (y0..<y1).filter { y in (from...to).contains { inked($0, y) } }
            let top = rows.first ?? y0
            let bottom = rows.last ?? y1
            return CGRect(
                x: CGFloat(from) / scale, y: CGFloat(top) / scale, width: CGFloat(to - from + 1) / scale,
                height: CGFloat(bottom - top + 1) / scale)
        }
        return (part(first, chevronEnd), part(labelStart, labelEnd))
    }

    /// Where `target` is hovered and pressed, in AppKit screen points: a
    /// Toggle's NSSwitch centre; the DisclosureGroup's chevron's or label's
    /// centre (disclosureParts of `restPNG`).
    @MainActor
    func targetPoint(_ target: String, restPNG: String) -> CGPoint? {
        guard let window else { return nil }
        if target.hasPrefix("disclosure") {
            guard let parts = disclosureParts(restPNG) else { return nil }
            let rect = target == "disclosure_chevron" ? parts.chevron : parts.label
            let inWindow = CGRect(x: rect.minX, y: window.frame.height - rect.maxY, width: rect.width, height: rect.height)
            return screenCentre(inWindow, window: window)
        }
        guard let item = items.first(where: { $0.name == target }), let toggle = firstSwitch(item.view) else {
            return nil
        }
        return screenCentre(toggle.convert(toggle.bounds, to: nil), window: window)
    }

    /// Where the cursor is parked, in AppKit screen points: the middle of the
    /// content view's left margin, `inset / 2` from its left edge and half
    /// its height down, where hitTest: gives the content view itself; the
    /// TOML lines of the reads, and the reason where it cannot be parked.
    @MainActor
    func parkPoint() -> (point: CGPoint?, lines: String, reason: String?) {
        guard let window else { return (nil, "", "the host has no window") }
        guard let content = window.contentView else { return (nil, "", "the window has no content view") }
        let local = CGPoint(x: inset / 2, y: content.bounds.height / 2)
        let inWindow = content.convert(local, to: nil)
        let hit = content.superview?.hitTest(inWindow)
        let hitName = hit.map { String(describing: type(of: $0)) } ?? "absent"
        let lines = "park_point_in_content_pt = [\(number(local.x)), \(number(local.y))]\n"
            + "view_at_park_point = \(quoted(hitName))\n"
        guard let hit, hit === content else {
            return (nil, lines, "hitTest: at the park point gives \(hitName), not the content view")
        }
        return (window.convertPoint(toScreen: inWindow), lines, nil)
    }

    /// The events recorded since the last call, which it clears.
    @MainActor
    func drain() -> [String] {
        let drained = events
        events = []
        return drained
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
        let setup: (windowNumber: Int, park: CGPoint?, parkReason: String?, text: String)? = onMain {
            guard let window = self.window else { return nil }
            let park = self.parkPoint()
            let info = ProcessInfo.processInfo
            let workspace = NSWorkspace.shared
            var accents: [String] = []
            for name in [NSAppearance.Name.aqua, .darkAqua] {
                NSAppearance(named: name)?.performAsCurrentDrawingAppearance {
                    accents.append("\(name.rawValue) = \(srgb(.controlAccentColor))")
                }
            }
            let defaults = UserDefaults.standard
            let environment = """
                [environment]
                os = \(quoted(info.operatingSystemVersionString))
                backing_scale = \(number(window.backingScaleFactor))
                window_frame_pt = \(rectText(window.frame))
                screen_frame_pt = \(NSScreen.screens.first.map { rectText($0.frame) } ?? quoted("absent"))
                active = \(NSApp.isActive)
                key = \(window.isKeyWindow)
                accepts_mouse_moved_events = \(window.acceptsMouseMovedEvents)
                item_size_pt = [\(number(itemWidth)), \(number(itemHeight))]
                \(park.lines)
                [accessibility]
                reduce_transparency = \(workspace.accessibilityDisplayShouldReduceTransparency)
                increase_contrast = \(workspace.accessibilityDisplayShouldIncreaseContrast)
                differentiate_without_color = \(workspace.accessibilityDisplayShouldDifferentiateWithoutColor)
                reduce_motion = \(workspace.accessibilityDisplayShouldReduceMotion)
                invert_colors = \(workspace.accessibilityDisplayShouldInvertColors)
                effective_appearance = \(quoted(NSApp.effectiveAppearance.name.rawValue))
                apple_interface_style = \(quoted(defaults.string(forKey: "AppleInterfaceStyle") ?? "absent"))
                apple_accent_color = \(quoted(defaults.object(forKey: "AppleAccentColor").map { "\($0)" } ?? "absent"))
                control_accent_color = { \(accents.joined(separator: ", ")) }

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
            return (window.windowNumber, park.point, park.reason, environment + colours + itemLines)
        }
        guard let setup else {
            failures.append("the host has no window")
            return finish("", captures: [], notes: notes, failures: failures)
        }
        let windowNumber = setup.windowNumber
        /// Posts `type` at `screen` (AppKit screen points) where the window
        /// there is the host's; else, or where it cannot be posted, the reason.
        func postOwn(_ type: CGEventType, at screen: CGPoint, what: String) -> String? {
            let (under, point): (Int, CGPoint?) = onMain {
                (NSWindow.windowNumber(at: screen, belowWindowWithWindowNumber: 0), quartz(screen))
            }
            guard under == windowNumber else {
                return "\(what): the window at the point is window \(under), not the host's \(windowNumber): not posted"
            }
            guard let point else { return "\(what): no screen to convert the point to: not posted" }
            return post(type, at: point) ? nil : "\(what): the event was not made"
        }
        var pointer = keyed
        if !keyed { notes.append("not hovered or pressed: the window is not key in an active application") }
        if let reason = setup.parkReason {
            failures.append("park: \(reason): nothing is hovered or pressed")
            pointer = false
        }
        if pointer, let park = setup.park, let reason = postOwn(.mouseMoved, at: park, what: "park") {
            failures.append("\(reason): nothing is hovered or pressed")
            pointer = false
        }
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
                let (active, key, events, at): (Bool, Bool, [String], String) = onMain {
                    let at: String
                    if let point, let window = self.window {
                        let local = self.frameTopLeft(point, window: window)
                        let global = quartz(point).map { "[\(number($0.x)), \(number($0.y))]" } ?? quoted("absent")
                        at = "target_pt = [\(number(local.x)), \(number(local.y))]\nmouse_global_pt = \(global)"
                    } else {
                        at = "target_pt = \(quoted("parked"))\nmouse_global_pt = \(quoted("parked"))"
                    }
                    return (NSApp.isActive, self.window?.isKeyWindow ?? false, self.drain(), at)
                }
                if let reason = capture(windowNumber: windowNumber, to: path) { failures.append("\(state): \(reason)") }
                captures.append(
                    """
                    [[capture]]
                    appearance = \(quoted(name.rawValue))
                    state = \(quoted(state))
                    target = \(quoted(target))
                    file = \(quoted("\(name.rawValue)/\(state).png"))
                    \(at)
                    button_held = \(held)
                    active = \(active)
                    key = \(key)
                    events = [\(events.joined(separator: ", "))]

                    """)
            }
            shoot("normal", target: "none", point: nil, held: false)
            let parts: String = onMain {
                guard let parts = self.disclosureParts("\(folder)/normal.png") else { return quoted("absent") }
                return "{ chevron_pt = \(rectText(parts.chevron)), label_pt = \(rectText(parts.label)) }"
            }
            captures.append(
                """
                [[disclosure_parts]]
                appearance = \(quoted(name.rawValue))
                found_in = "normal.png"
                parts = \(parts)

                """)
            for target in ["toggle_off", "toggle_on", "disclosure_chevron", "disclosure_label"] {
                guard pointer, let park = setup.park else { break }
                let centre: CGPoint? = onMain { self.targetPoint(target, restPNG: "\(folder)/normal.png") }
                guard let centre else {
                    notes.append("\(name.rawValue) \(target): no control in its view tree to aim at, not hovered or pressed")
                    continue
                }
                let what = "\(name.rawValue) \(target)"
                if let reason = postOwn(.mouseMoved, at: centre, what: "\(what) hover") {
                    failures.append("\(reason): nothing more is hovered or pressed")
                    pointer = false
                    break
                }
                Thread.sleep(forTimeInterval: 1.0)
                shoot("hovered_\(target)", target: target, point: centre, held: false)
                if let reason = postOwn(.leftMouseDown, at: centre, what: "\(what) press") {
                    failures.append("\(reason): nothing more is hovered or pressed")
                    pointer = false
                    break
                }
                Thread.sleep(forTimeInterval: 0.6)
                shoot("pressed_\(target)", target: target, point: centre, held: true)
                if let reason = postOwn(.leftMouseUp, at: centre, what: "\(what) release") {
                    failures.append("\(reason): nothing more is hovered or pressed")
                    pointer = false
                    break
                }
                Thread.sleep(forTimeInterval: 0.3)
                if let reason = postOwn(.mouseMoved, at: park, what: "\(what) park") {
                    failures.append("\(reason): nothing more is hovered or pressed")
                    pointer = false
                }
                Thread.sleep(forTimeInterval: 0.8)
                let events: [String] = onMain { self.drain() }
                captures.append(
                    """
                    [[release]]
                    appearance = \(quoted(name.rawValue))
                    target = \(quoted(target))
                    events = [\(events.joined(separator: ", "))]

                    """)
            }
        }
        return finish(setup.text, captures: captures, notes: notes, failures: failures)
    }

    /// Writes switch-label.toml from `notes` and `failures` (top-level keys,
    /// so first: a key after a table header belongs to that table), then
    /// `text` (the environment, the colours and the items) and `captures`;
    /// true where a step failed.
    nonisolated func finish(_ text: String, captures: [String], notes: [String], failures: [String]) -> Bool {
        var failures = failures
        let whole = "notes = [\(notes.map(quoted).joined(separator: ", "))]\n"
            + "failures = [\(failures.map(quoted).joined(separator: ", "))]\n\n"
            + text + captures.joined(separator: "\n")
        do {
            try whole.write(toFile: "\(out)/switch-label.toml", atomically: true, encoding: .utf8)
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
