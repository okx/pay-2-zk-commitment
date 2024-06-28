//
//  ReceiverView.swift
//  ZKSimpleApp
//
//  Created by Quach Ha Chan Thanh on 18/6/24.
//

import Foundation
import SwiftUI

struct ClaimListView: View {
    @StateObject var viewModel: ClaimListViewModel
    
    var body: some View {
        if #available(iOS 16.0, *) {
            VStack {
                List(viewModel.claimants, id: \.item.index) { item in
                    builItem(item)
                        .listRowInsets(EdgeInsets())
                }
                .listStyle(PlainListStyle())
                .frame(maxWidth: .infinity)
                .scrollContentBackground(.hidden)
            }
            .padding()
            .navigationTitle("Claim list")
        } else {
            HStack { }
        }
        
    }
    
    func builItem(_ info: ClaimListViewModel.ClaimInfo) -> some View {
        HStack(alignment: .center) {
            let item = info.item
            
            VStack(alignment: .leading, spacing: 8) {
                HStack {
                    Text("ID:")
                    Text("\(item.index)")
                        .font(.caption)
                }
                
                HStack {
                    Text("Amount-secret:")
                    Text("\(item.amount) - \(item.secret)")
                        .font(.caption)
                }
                
                if case .claimed(let duration) = info.status {
                    HStack {
                        Text("Duration:")
                        Text(duration)
                            .font(.caption)
                    }
                }
            }
            
            Spacer()
            
            if case .unclaim = info.status {
                Button(action: {
                    viewModel.claim(item)
                }, label: {
                    Text(info.status.display)
                        .padding()
                        .background(Color.green)
                        .foregroundColor(.white)
                        .cornerRadius(8)

                })
            }
            
        }
        .padding(.vertical, 10)
    }
}

class ClaimManager {
    var claimed = [[Int]: String]()
    static let shared = ClaimManager()
    
    func getStatus(depositId: Int, claim: Claimant) -> ClaimListViewModel.ClaimStatus {
        let key = [depositId, claim.index]
        if let cl = claimed[key] {
            return .claimed(cl)
        }
        return .unclaim
    }
    
    func setClaimed(depositId: Int, claim: Claimant, time: String) {
        let key = [depositId, claim.index]
        claimed[key] = time
    }
}

class ClaimListViewModel: ObservableObject {
    enum ClaimStatus {
        case claimed(String)
        case unclaim
        
        var display: String {
            switch self {
            case .claimed:
                return "Claimed"
            case .unclaim:
                return "Click to claim"
            }
        }
        
        var duration: String? {
            switch self {
            case .claimed(let v):
                return v
            case .unclaim:
                return nil
            }
        }
    }
    
    struct ClaimInfo {
        let item: Claimant
        let status: ClaimStatus
    }
    
    let item: DepositItem
    @Published var claimants: [ClaimInfo] = []
    
    init(item: DepositItem) {
        self.item = item
        rebuild()
    }
    
    func claim(_ claimant: Claimant) {
        do {
            let d = Date().timeIntervalSince1970
            let result = try generateProofOfClaim(
                amount: claimant.amount,
                secret: claimant.secret,
                index: Int32(claimant.index),
                commitmentTree: item.commitment)
            let takingTime = Date().timeIntervalSince1970 - d
            print("Taking \(takingTime)")
            
            ClaimManager.shared.setClaimed(depositId: item.id, claim: claimant, time: "\(takingTime)s")
            rebuild()
            print(result)
        } catch {
            print(error)
        }
    }
    
    func rebuild() {
        claimants = item.claimants.map {
            .init(item: $0, status: ClaimManager.shared.getStatus(depositId: item.id, claim: $0))
        }
    }
}
