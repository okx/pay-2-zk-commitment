//
//  ZKSimpleAppTests.swift
//  ZKSimpleAppTests
//
//  Created by Quach Ha Chan Thanh on 25/6/24.
//

import XCTest
@testable import ZKSimpleApp

final class ZKSimpleAppTests: XCTestCase {
    
    func testTree_4() throws {
        // This is an example of a performance test case.
        let data = generateData(4)
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = setupCommitment(distribution: data)
        }
    }
    
    func testProof_4() throws {
        let data = generateData(4)
        var tree = setupCommitment(distribution: data)
        
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = try! generateProofOfClaim(amount: data.first!.amount, secret: data.first!.secret, index: 0, commitmentTree: tree)
        }
    }

    func testTree_64() throws {
        // This is an example of a performance test case.
        let data = generateData(64)
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = setupCommitment(distribution: data)
        }
    }
    
    func testProof_64() throws {
        let data = generateData(64)
        var tree = setupCommitment(distribution: data)
        
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = try! generateProofOfClaim(amount: data.first!.amount, secret: data.first!.secret, index: 0, commitmentTree: tree)
        }
    }
    
    func testTree_512() throws {
        // This is an example of a performance test case.
        let data = generateData(64)
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = setupCommitment(distribution: data)
        }
    }
    
    func testProof_512() {
        let data = generateData(512)
        var tree = setupCommitment(distribution: data)
        
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = try! generateProofOfClaim(amount: data.first!.amount, secret: data.first!.secret, index: 0, commitmentTree: tree)
        }
    }
    
    func testTree_4096() throws {
        // This is an example of a performance test case.
        let data = generateData(4096)
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = setupCommitment(distribution: data)
        }
    }
    
    func testProof_4096() {
        let data = generateData(4096)
        var tree = setupCommitment(distribution: data)
        
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = try! generateProofOfClaim(amount: data.first!.amount, secret: data.first!.secret, index: 0, commitmentTree: tree)
        }
    }
    
    func testTree_65536() throws {
        // This is an example of a performance test case.
        let data = generateData(65536)
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = setupCommitment(distribution: data)
        }
    }
    
    func testProof_65536() {
        let data = generateData(65536)
        var tree = setupCommitment(distribution: data)
        
        measure(metrics: [XCTMemoryMetric(), XCTCPUMetric(), XCTClockMetric()]) {
            // Put the code you want to measure the time of here.
            _ = try! generateProofOfClaim(amount: data.first!.amount, secret: data.first!.secret, index: 0, commitmentTree: tree)
        }
    }
    
    func generateData(_ count: Int) -> [AmountSecretPairing] {
        (1...count).map { _ in
            AmountSecretPairing(amount: 1, secret: UInt64.random(in: 999...99999999999))
        }
    }

}
