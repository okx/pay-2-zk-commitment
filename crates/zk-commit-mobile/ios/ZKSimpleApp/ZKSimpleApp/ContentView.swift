//
//  ContentView.swift
//  ZKSimpleApp
//
//  Created by Quach Ha Chan Thanh on 13/6/24.
//

import SwiftUI
import Web3Modal

struct ContentView: View {
    @StateObject var viewModel: DepositListViewModel
    @State var testPerfDuration: Double = 0
    
    var body: some View {
        if #available(iOS 16.0, *) {
            NavigationView {
                VStack {
                    
                    HStack {
                        Button(action: {
                            let date = Date().timeIntervalSince1970
                            testPerformance()
                            testPerfDuration = Date().timeIntervalSince1970 - date
                            Web3Modal.present()
                        }, label: {
                            Text("Trigger test perf")
                        })
                        
                        Spacer()
                        
                        if testPerfDuration != 0 {
                            Text("Taking \(testPerfDuration)")
                        }
                    }
                    
                    List(viewModel.depositList, id: \.id) { item in
                        buildDepositItem(item)
                    }
                    
                    Spacer()
                }
                .toolbar {
                    ToolbarItem(placement: .navigationBarTrailing) {
                        NavigationLink(destination: CreateDepositorView(viewModel: .init(creator: viewModel))) {
                            Image(systemName: "plus.circle")
                        }
                    }
                }
                .navigationTitle("Deposit")
            }
        } else {
            VStack {}
        }
    }
    
    func buildDepositItem(_ item: DepositItem) -> some View {
        NavigationLink(destination: ClaimListView(viewModel: .init(item: item))) {
            VStack(alignment: .leading) {
                HStack(alignment: .center) {
                    Text("ID:")
                    Text("\(item.id)")
                        .font(.caption)
                }
                
                HStack {
                    HStack {
                        Text("Duration")
                        Text("\(item.duration)")
                            .font(.caption)
                    }
                    
                    Spacer()
                    
                    HStack {
                        Text("Claimants")
                        Text("\(item.claimants.count)")
                            .font(.caption)
                    }
                    
                }
            }
        }
    }
}

struct Claimant {
    let index: Int
    let amount: UInt64
    let secret: UInt64
}

struct DepositItem {
    let id: Int
    let duration: String
    let commitment: CommitmentTree
    let claimants: [Claimant]
}

class DepositListViewModel: ObservableObject, DepositCreatable {
    @Published var depositList: [DepositItem] = []
    
    func createDeposit(claimants: [Claimant]) async throws {
        let duration = Date().timeIntervalSince1970
        let tree = setupCommitment(distribution: claimants.compactMap {
            return AmountSecretPairing(amount: $0.amount, secret: $0.secret)
        }.reversed())

        let timeTaken = Date().timeIntervalSince1970 - duration
        print(timeTaken)
        depositList.insert(
            .init(id: depositList.count + 1, duration: timeTaken.description, commitment: tree, claimants: claimants),
            at: 0)
        
    }
}
