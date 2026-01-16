#!/usr/bin/env swift
import Foundation
import Vision
import AppKit

guard CommandLine.arguments.count > 1 else {
    print("Usage: ocr_helper.swift <image_path>")
    exit(1)
}

let imagePath = CommandLine.arguments[1]

guard let nsImage = NSImage(contentsOfFile: imagePath) else {
    print("Error: Could not load image")
    exit(1)
}

guard let cgImage = nsImage.cgImage(forProposedRect: nil, context: nil, hints: nil) else {
    print("Error: Could not convert to CGImage")
    exit(1)
}

// Create the OCR request
let request = VNRecognizeTextRequest()
request.recognitionLevel = .accurate
request.usesLanguageCorrection = true

let handler = VNImageRequestHandler(cgImage: cgImage, options: [:])

do {
    // Perform the OCR
    try handler.perform([request])
    
    // Extract the results
    guard let observations = request.results else {
        exit(0)
    }
    
    // Get the text from each observation
    let recognizedText = observations.compactMap { observation in
        observation.topCandidates(1).first?.string
    }.joined(separator: "\n")
    
    // Print the text (this is what Rust will capture)
    print(recognizedText)
    
} catch {
    print("Error: \(error.localizedDescription)")
    exit(1)
}
