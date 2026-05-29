// swift-tools-version:5.9
import PackageDescription

let package = Package(
    name: "IOHIDManagerBridge",
    platforms: [
        .macOS(.v10_15)
    ],
    products: [
        .library(
            name: "IOHIDManagerBridge",
            type: .static,
            targets: ["IOHIDManagerBridge"])
    ],
    targets: [
        .target(
            name: "IOHIDManagerBridge",
            path: "Sources/IOHIDManagerBridge")
    ]
)
