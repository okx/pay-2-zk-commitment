//
//  CreateDepositorView.swift
//  ZKSimpleApp
//
//  Created by Quach Ha Chan Thanh on 18/6/24.
//

import Foundation
import SwiftUI

struct CreateDepositorView: View {
    @StateObject var viewModel: CreateDepositorViewModel
    @State var amount: String = ""
    @State var numberCreation: Int = 1
    @Environment(\.presentationMode) var presentationMode
    @FocusState private var focusedField: String?
    
    var body: some View {
        VStack(alignment: .leading) {
            VStack(alignment: .leading) {
                Text("Create claimants")
                    .font(.title)
                
                TextField("Amount per claimant", text: $amount)
                    .frame(height: 48)
                    .frame(maxWidth: .infinity)
                    .padding(.horizontal, 6)
                    .keyboardType(.numberPad)
                    .overlay(
                        RoundedRectangle(cornerRadius: 5)
                            .stroke(lineWidth: 1.0)
                    )
                    .focused($focusedField, equals: "123")
                    .cornerRadius(5)
                
                HStack {
                    TextField("Repeating", text: Binding(get: { String(numberCreation) }, set: { numberCreation = Int($0) ?? 0 }) )
                        .frame(height: 48)
                        .frame(maxWidth: .infinity)
                        .padding(.horizontal, 6)
                        .keyboardType(.numberPad)
                        .overlay(
                            RoundedRectangle(cornerRadius: 5)
                                .stroke(lineWidth: 1.0)
                        )
                        .focused($focusedField, equals: "123")
                        .cornerRadius(5)
                    
                    Stepper(onIncrement: {
                        self.numberCreation += 1
                    }, onDecrement: {
                        self.numberCreation -= 1
                    }) {
                        Text("\(numberCreation)")
                    }
                }
                                
                Button(
                    action: {
                        if let amount = UInt64(amount) {
                            viewModel.addClaimant(amount: amount, repeating: numberCreation)
                            self.amount = ""
                        }
                    },
                    label: {
                        Text("Add claimant")
                            .frame(maxWidth: .infinity)
                            .padding(.vertical, 6)
                    })
                    .buttonStyle(.bordered)
                    .disabled(Int(amount) == nil)
                
                Button(
                    action: {
                        viewModel.createDeposit()
                    },
                    label: {
                        Text("Create")
                            .padding(.vertical, 6)
                            .frame(maxWidth: .infinity)
                    })
                .disabled(viewModel.claimants.count.isPowerOfTwo == false)
                .buttonStyle(.borderedProminent)
            }
            
            Text("Claimant List")
                .font(.title)
                .padding(.top, 30)
                .onTapGesture {
                    focusedField = nil
                }
            
            List(viewModel.claimants, id: \.index) { item in
                builItem(item)
                    .listRowInsets(EdgeInsets())
            }
            .listStyle(PlainListStyle())
            .frame(maxWidth: .infinity)
            .scrollContentBackground(.hidden)
            

            Spacer()
        }
        .padding()
        .navigationTitle("Create A Deposit")
        .onReceive(viewModel.$shouldPop) { shouldPop in
            if shouldPop {
                self.presentationMode.wrappedValue.dismiss()
            }
        }
    }
    
    func builItem(_ item: Claimant) -> some View {
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
        }
        .padding(.vertical, 10)
    }
}

protocol DepositCreatable: AnyObject {
    func createDeposit(claimants: [Claimant]) async throws
}

class CreateDepositorViewModel: ObservableObject {
    @Published var claimants: [Claimant] = []
    @Published var shouldPop: Bool = false
    weak var creator: DepositCreatable?
    
    init(creator: DepositCreatable?) {
        self.creator = creator
    }
    
    func addClaimant(amount: UInt64, repeating: Int) {
        for _ in 1...repeating {
            let secret = UInt64.random(in: 100...UInt64.max)
            claimants.insert(
                .init(
                    index: claimants.count,
                    amount: amount,
                    secret: secret
                )
            , at: 0)
        }
    }
    
    func createDeposit() {
        Task { @MainActor in
            do {
                try await creator?.createDeposit(claimants: claimants)
                shouldPop = true
            } catch {
                // Handler later
            }
        }
    }
}

extension Int {
    var isPowerOfTwo: Bool {
        guard self > 0 else {
            return false
        }
        return (self & (self - 1)) == 0
    }
}
