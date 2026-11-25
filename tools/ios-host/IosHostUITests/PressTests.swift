// The iOS host's press pass (tools/ios-host/IosHost/IosHost.swift, `press`
// mode): launches the app with the run's name (IOS_HOST_RUN, which run.sh
// passes as TEST_RUNNER_IOS_HOST_RUN) and its specimen set (IOS_HOST_SET, as
// `set=<name>`) and presses each element the app arms, `press-0`, `press-1`,
// … (a button, a bar item, or the list set's marker over a list cell, a
// segmented control, a tab bar item, a switch: any element type), by a real
// touch at its centre held for `holdSeconds`, until the app shows
// `press-done`. Where the run's name starts with `hover` (the app's hover
// mode, on an iPad), each element is hovered instead: the pointer moved to
// the window's corner, then to the element's centre (`XCUICoordinate.hover()`,
// iOS 15+), again until the app arms the next one.

import XCTest

final class PressTests: XCTestCase {
    // 4 s let the touch end before the screenshot arrived in 11 and 40
    // presses of the list set's two press runs on iOS 18.6 (run 45512794102),
    // past the host's five retakes.
    let holdSeconds = 8.0
    let waitSeconds = 900.0
    // A hover try (the corner's and the target's) took about 11 s in run
    // 45622616324's log, so the app's 60 s wait (hoverWaitSeconds) gives
    // about 5; the cap only bounds the loop should the app not move on.
    let hoverTries = 10

    func testPress() throws {
        let environment = ProcessInfo.processInfo.environment
        let app = XCUIApplication()
        let run = environment["IOS_HOST_RUN"] ?? "press"
        let hover = run.hasPrefix("hover")
        var arguments = [run]
        if environment["IOS_HOST_QUICK"] == "1" { arguments.append("quick") }
        if let set = environment["IOS_HOST_SET"], !set.isEmpty { arguments.append("set=\(set)") }
        app.launchArguments = arguments
        app.launch()
        let done = app.staticTexts["press-done"]
        var index = 0
        while true {
            let target = app.descendants(matching: .any).matching(identifier: "press-\(index)").firstMatch
            let start = Date()
            var found = false
            while Date().timeIntervalSince(start) < waitSeconds {
                if target.exists {
                    found = true
                    break
                }
                if done.exists { break }
                Thread.sleep(forTimeInterval: 0.2)
            }
            if !found {
                XCTAssertTrue(done.exists, "neither press-\(index) nor press-done within \(waitSeconds) s")
                XCTAssertEqual(done.label, "press-done", "the app reported failures")
                break
            }
            // At the element's centre as a coordinate: the list set's
            // identifier is on a marker that takes no touch, over the view
            // pressed (IosHost.swift, armNext).
            let centre = target.coordinate(withNormalizedOffset: CGVector(dx: 0.5, dy: 0.5))
            if hover {
                guard #available(iOS 15.0, *) else {
                    XCTFail("XCUICoordinate.hover() needs iOS 15")
                    break
                }
                // From the page's margin, so the pointer moves onto the
                // target even where it already lies there (a retake); again
                // while the target stays armed, as the pointer leaves on its
                // own (run 45622616324) and the app reads the target only
                // once the pointer has stayed on it. The target's centre is
                // taken once, relative to the app: the app takes the
                // identifier away once it has read the target, which can be
                // during a try, and a coordinate of the element then fails
                // the test (run 45632490596).
                let origin = app.coordinate(withNormalizedOffset: CGVector(dx: 0, dy: 0))
                let frame = target.frame
                let appFrame = app.frame
                let point = origin.withOffset(
                    CGVector(dx: frame.midX - appFrame.minX, dy: frame.midY - appFrame.minY))
                var tries = 0
                repeat {
                    origin.withOffset(CGVector(dx: 4, dy: 4)).hover()
                    point.hover()
                    tries += 1
                } while tries < hoverTries && target.exists && !done.exists
            } else {
                centre.press(forDuration: holdSeconds)
            }
            index += 1
        }
        app.terminate()
    }
}
