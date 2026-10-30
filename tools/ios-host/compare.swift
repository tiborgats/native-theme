// Compares the files of two runs of the iOS host that are not byte-identical,
// pixel by pixel (both decoded into 8-bit sRGB RGBA), for tools/ios-host/run.sh:
//
//     xcrun swift tools/ios-host/compare.swift [--list-only] <run1> <run2> <list>
//
// <list> holds one path per line, relative to both runs. Prints each PNG's
// count of differing pixels and their largest difference in levels, and each
// text file's first differing lines. Exits 1 when a measurement file (anything
// outside draw/) is a text file that differs, or is not a PNG of the same size,
// or differs by more than `tolerance`: iOS 26's Liquid Glass edges in the
// simulator's screenshot vary by a level between runs in a few pixels (spike
// I; `tolerance`), and Liquid Glass over black by up to two levels in a few
// dozen pixels (iOS 26.5, `glassTolerance`, a path holding `glass`);
// drawHierarchy's copies (draw/) are listed and fail nothing, as is every
// file under --list-only and every capture of the activity indicator, which
// turns without end (a path holding `+activity_indicator_`).

import CoreGraphics
import Foundation
import ImageIO

let tolerance = 1
let glassTolerance = 2

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

var arguments = CommandLine.arguments
let listOnly = arguments.contains("--list-only")
arguments.removeAll { $0 == "--list-only" }
guard arguments.count == 4 else {
    print("usage: compare.swift [--list-only] <run1> <run2> <list>")
    exit(2)
}
let first = URL(fileURLWithPath: arguments[1])
let second = URL(fileURLWithPath: arguments[2])
let list = (try? String(contentsOfFile: arguments[3], encoding: .utf8)) ?? ""
var failed = false
for path in list.split(separator: "\n").map(String.init) {
    let measured = !listOnly && !path.hasPrefix("./draw/") && !path.contains("+activity_indicator_")
    if !path.hasSuffix(".png") {
        let a = (try? String(contentsOf: first.appendingPathComponent(path), encoding: .utf8)) ?? ""
        let b = (try? String(contentsOf: second.appendingPathComponent(path), encoding: .utf8)) ?? ""
        let linesA = a.split(separator: "\n", omittingEmptySubsequences: false)
        let linesB = b.split(separator: "\n", omittingEmptySubsequences: false)
        var shown = 0
        for index in 0..<max(linesA.count, linesB.count) where shown < 20 {
            let lineA = index < linesA.count ? String(linesA[index]) : "(none)"
            let lineB = index < linesB.count ? String(linesB[index]) : "(none)"
            if lineA != lineB {
                print("\(path):\(index + 1): \(lineA) | \(lineB)")
                shown += 1
            }
        }
        print("\(path): differs (\(linesA.count) and \(linesB.count) lines)")
        if measured { failed = true }
        continue
    }
    guard
        let a = samples(first.appendingPathComponent(path)),
        let b = samples(second.appendingPathComponent(path)),
        a.width == b.width, a.height == b.height, a.bytes.count == b.bytes.count
    else {
        print("\(path): not comparable as images")
        if measured { failed = true }
        continue
    }
    var pixels = 0
    var largest = 0
    for pixel in stride(from: 0, to: a.bytes.count - 3, by: 4) {
        var difference = 0
        for sample in pixel..<(pixel + 4) {
            difference = max(difference, abs(Int(a.bytes[sample]) - Int(b.bytes[sample])))
        }
        if difference > 0 { pixels += 1 }
        largest = max(largest, difference)
    }
    print("\(path): \(pixels) pixels differ, by at most \(largest) levels")
    if measured && largest > (path.contains("glass") ? glassTolerance : tolerance) { failed = true }
}
exit(failed ? 1 : 0)
