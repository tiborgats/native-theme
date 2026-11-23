// The iOS host's press pass (tools/ios-host/IosHost/IosHost.swift, `press`
// mode): launches the app with the run's name (IOS_HOST_RUN, which run.sh
// passes as TEST_RUNNER_IOS_HOST_RUN) and its specimen set (IOS_HOST_SET, as
// `set=<name>`) and presses each element the app arms, `press-0`, `press-1`,
// … (a button, a bar item, a list cell, a segmented control at its centre, a
// switch: any element type), by a real touch held for `holdSeconds`, until
// the app shows `press-done`.

import XCTest

final class PressTests: XCTestCase {
    let holdSeconds = 4.0
    let waitSeconds = 900.0

    func testPress() throws {
        let environment = ProcessInfo.processInfo.environment
        let app = XCUIApplication()
        var arguments = [environment["IOS_HOST_RUN"] ?? "press"]
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
            target.press(forDuration: holdSeconds)
            index += 1
        }
        app.terminate()
    }
}
