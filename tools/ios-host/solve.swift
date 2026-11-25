// Solves the colour and alpha of what UIKit drew over a cell, pixel by pixel,
// from the two captures of one specimen and state over opaque black and opaque
// white, all else equal (tools/ios-host/IosHost/IosHost.swift), for
// tools/ios-host/run.sh:
//
//     xcrun swift tools/ios-host/solve.swift <run> <out-dir> <out-toml>
//
// Finds every screen/<appearance>/<specimen>/<size>/black/<state>.png of <run>
// with its white/<state>.png. Over black a layer of colour F and alpha a
// composites to B = a F, over white to W = a F + (1 - a) 255, per channel, in
// the screenshot's 8-bit sRGB encoding (the solve assumes UIKit composites in
// that encoding): a = 1 - (W - B) / 255, F = B / a. Each channel gives its own
// a; a single layer gives one a for all three, so the channels' spread is the
// check of that assumption (Liquid Glass, which adapts to what is behind it,
// fails it). Writes <out-dir>/<appearance>/<specimen>/<size>/<state>.png, the
// layer as un-premultiplied 8-bit sRGB RGBA (a the channels' mean), and one
// [[layer]] per pair to <out-toml>: the pixels that are opaque (W = B),
// transparent (B = 0, W = 255), partly covering, and inconsistent (some W < B,
// which no layer can give); the channels' largest alpha spread in levels; and
// the five commonest solved RGBA values among the covering pixels.

import CoreGraphics
import Foundation
import ImageIO

func samples(_ url: URL) -> (width: Int, height: Int, bytes: [UInt8])? {
    guard let source = CGImageSourceCreateWithURL(url as CFURL, nil),
        let image = CGImageSourceCreateImageAtIndex(source, 0, nil),
        let space = CGColorSpace(name: CGColorSpace.sRGB),
        let context = CGContext(
            data: nil, width: image.width, height: image.height, bitsPerComponent: 8,
            bytesPerRow: image.width * 4, space: space,
            bitmapInfo: CGImageAlphaInfo.premultipliedLast.rawValue)
    else { return nil }
    context.draw(image, in: CGRect(x: 0, y: 0, width: image.width, height: image.height))
    guard let data = context.makeImage()?.dataProvider?.data as Data? else { return nil }
    return (image.width, image.height, [UInt8](data))
}

func writePNG(_ bytes: [UInt8], width: Int, height: Int, to url: URL) -> Bool {
    guard let space = CGColorSpace(name: CGColorSpace.sRGB),
        let provider = CGDataProvider(data: Data(bytes) as CFData),
        let image = CGImage(
            width: width, height: height, bitsPerComponent: 8, bitsPerPixel: 32, bytesPerRow: width * 4,
            space: space, bitmapInfo: CGBitmapInfo(rawValue: CGImageAlphaInfo.last.rawValue), provider: provider,
            decode: nil, shouldInterpolate: false, intent: .defaultIntent),
        let destination = CGImageDestinationCreateWithURL(url as CFURL, "public.png" as CFString, 1, nil)
    else { return false }
    CGImageDestinationAddImage(destination, image, nil)
    return CGImageDestinationFinalize(destination)
}

func quoted(_ text: String) -> String {
    "\"" + text.replacingOccurrences(of: "\\", with: "\\\\").replacingOccurrences(of: "\"", with: "\\\"") + "\""
}

let arguments = CommandLine.arguments
guard arguments.count == 4 else {
    print("usage: solve.swift <run> <out-dir> <out-toml>")
    exit(2)
}
let run = URL(fileURLWithPath: arguments[1])
let outDirectory = URL(fileURLWithPath: arguments[2])
let outToml = URL(fileURLWithPath: arguments[3])
let screen = run.appendingPathComponent("screen")
let manager = FileManager.default
var blacks: [String] = []
if let enumerator = manager.enumerator(atPath: screen.path) {
    while let path = enumerator.nextObject() as? String {
        let parts = path.split(separator: "/")
        if parts.count == 5 && parts[3] == "black" && path.hasSuffix(".png") { blacks.append(path) }
    }
}
blacks.sort()
var toml = """
    # The layers solved from the captures over black and over white (tools/ios-host/solve.swift).
    run = \(quoted(run.lastPathComponent))
    model = "B = a F, W = a F + (1 - a) 255 per channel, 8-bit sRGB encoded; a = 1 - (W - B) / 255, F = B / a"


    """
var failed = false
for path in blacks {
    let parts = path.split(separator: "/").map(String.init)
    let whitePath = parts[0..<3].joined(separator: "/") + "/white/" + parts[4]
    let state = String(parts[4].dropLast(4))
    guard let black = samples(screen.appendingPathComponent(path)),
        let white = samples(screen.appendingPathComponent(whitePath)),
        black.width == white.width, black.height == white.height, black.bytes.count == white.bytes.count
    else {
        print("\(path): no comparable white capture at \(whitePath)")
        failed = true
        continue
    }
    var layer = [UInt8](repeating: 0, count: black.bytes.count)
    var opaque = 0
    var transparent = 0
    var partial = 0
    var inconsistent = 0
    var spread = 0
    var spreading = 0
    var counts: [UInt32: Int] = [:]
    for pixel in stride(from: 0, to: black.bytes.count - 3, by: 4) {
        var alphas: [Int] = []
        var bad = false
        for channel in 0..<3 {
            let b = Int(black.bytes[pixel + channel])
            let w = Int(white.bytes[pixel + channel])
            if w < b { bad = true }
            alphas.append(255 - max(0, w - b))
        }
        if bad { inconsistent += 1 }
        let pixelSpread = (alphas.max() ?? 0) - (alphas.min() ?? 0)
        spread = max(spread, pixelSpread)
        if pixelSpread > 2 { spreading += 1 }
        let alpha = Double(alphas.reduce(0, +)) / 3
        let alpha8 = Int(alpha.rounded())
        if alpha8 >= 255 {
            opaque += 1
        } else if alpha8 <= 0 {
            transparent += 1
        } else {
            partial += 1
        }
        guard alpha8 > 0 else { continue }
        var key = UInt32(alpha8)
        for channel in 0..<3 {
            let value = min(255, Int((Double(black.bytes[pixel + channel]) * 255 / alpha).rounded()))
            layer[pixel + channel] = UInt8(value)
            key = key << 8 | UInt32(value)
        }
        layer[pixel + 3] = UInt8(alpha8)
        counts[key, default: 0] += 1
    }
    let name = "\(parts[0])/\(parts[1])/\(parts[2])/\(state).png"
    let url = outDirectory.appendingPathComponent(name)
    do {
        try manager.createDirectory(at: url.deletingLastPathComponent(), withIntermediateDirectories: true)
    } catch {
        print("\(name): \(error)")
        failed = true
    }
    if !writePNG(layer, width: black.width, height: black.height, to: url) {
        print("\(name): the PNG does not write")
        failed = true
    }
    let common = counts.sorted { $0.value != $1.value ? $0.value > $1.value : $0.key < $1.key }.prefix(5).map {
        let key = $0.key
        let hex = String(format: "#%02x%02x%02x%02x", (key >> 16) & 0xff, (key >> 8) & 0xff, key & 0xff, key >> 24)
        return "{ rgba = \(quoted(hex)), pixels = \($0.value) }"
    }.joined(separator: ", ")
    toml += """
        [[layer]]
        appearance = \(quoted(parts[0]))
        specimen = \(quoted(parts[1]))
        size = \(quoted(parts[2]))
        state = \(quoted(state))
        black = \(quoted("screen/" + path))
        white = \(quoted("screen/" + whitePath))
        layer = \(quoted(name))
        pixels = \(black.width * black.height)
        opaque = \(opaque)
        transparent = \(transparent)
        partial = \(partial)
        inconsistent = \(inconsistent)
        alpha_spread_levels = \(spread)
        alpha_spread_over_2_pixels = \(spreading)
        common = [\(common)]


        """
}
do {
    try toml.write(to: outToml, atomically: true, encoding: .utf8)
} catch {
    print("\(outToml.path): \(error)")
    failed = true
}
print("\(blacks.count) pairs solved")
exit(failed ? 1 : 0)
