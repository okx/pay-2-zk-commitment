//
//  AppDelegate.swift
//  ZKSimpleApp
//
//  Created by Quach Ha Chan Thanh on 21/6/24.
//

import UIKit
import Foundation
import Web3Modal
import Web3
import CryptoSwift
import WalletConnectSigner
import Starscream
import Combine

class SocketConnectionManager: ObservableObject {
    @Published var socketConnected: Bool = false
}

class AppDelegate: NSObject, UIApplicationDelegate {
    private var disposeBag = Set<AnyCancellable>()
    private var socketConnectionManager = SocketConnectionManager()
    
    
    func application(_ application: UIApplication,
                     didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        // Call setup or configuration functions for third-party services here
        let metadata = AppMetadata(
            name: "Web3Modal Swift Dapp",
            description: "Web3Modal DApp sample",
            url: "www.web3modal.com",
            icons: ["https://avatars.githubusercontent.com/u/37784886"],
            redirect: try! .init(native: "zksimpleapp://", universal: "https://lab.web3modal.com/web3modal_example", linkMode: true)
        )
//        54ec6a69aa545565161408be85eaff18
//        let projectId =  "c6a69aa545565161408be85eaff18"
        let projectId = "54ec6a69aa545565161408be85eaff18"
        
        Networking.configure(
            groupIdentifier: "group.org.okx.ZKSimpleApp",
            projectId: projectId,
            socketFactory: DefaultSocketFactory()
        )
        
        Web3Modal.configure(
            projectId: projectId,
            metadata: metadata,
            crypto: DefaultCryptoProvider(),
            authRequestParams: nil) { error in
                print(error)
                self.setup()
            }
        
        return true
    }
    
    func setup() {
        Web3Modal.instance.socketConnectionStatusPublisher.receive(on: DispatchQueue.main).sink { [unowned self] status in
            print("Socket connection status: \(status)")
            self.socketConnectionManager.socketConnected = (status == .connected)
            
        }.store(in: &disposeBag)
        Web3Modal.instance.logger.setLogging(level: .debug)
        Sign.instance.setLogging(level: .debug)
        Networking.instance.setLogging(level: .debug)
        Relay.instance.setLogging(level: .debug)
        
        Web3Modal.instance.authResponsePublisher.sink { (id: RPCID, result: Result<(Session?, [Cacao]), AuthError>) in
            switch result {
            case .success((_, _)):
                //                       AlertPresenter.present(message: "User authenticated", type: .success)
                print("auth succ")
            case .failure(let error):
                //                       AlertPresenter.present(message: "User authentication error: \(error)", type: .error)
                print("auth failed")
                
            }
        }.store(in: &disposeBag)
        
        Web3Modal.instance.SIWEAuthenticationPublisher.sink { result in
            switch result {
            case .success((let message, let signature)):
                //                       AlertPresenter.present(message: "User authenticated", type: .success)
                print("siw succ")
            case .failure(let error):
                //                       AlertPresenter.present(message: "User authentication error: \(error)", type: .error)
                print("siw failed \(error)")
            }
        }.store(in: &disposeBag)
    }
}

extension WebSocket: WebSocketConnecting {
    
}

struct DefaultSocketFactory: WebSocketFactory {
    func create(with url: URL) -> WebSocketConnecting {
        let socket = WebSocket(request: .init(url: url))
        let queue = DispatchQueue(label: "com.walletconnect.sdk.sockets", attributes: .concurrent)
        socket.callbackQueue = queue
        return socket
    }
}

struct DefaultCryptoProvider: CryptoProvider {
    
    public func recoverPubKey(signature: EthereumSignature, message: Data) throws -> Data {
        let publicKey = try EthereumPublicKey(
            message: message.bytes,
            v: EthereumQuantity(quantity: BigUInt(signature.v)),
            r: EthereumQuantity(signature.r),
            s: EthereumQuantity(signature.s)
        )
        return Data(publicKey.rawPublicKey)
    }
    
    public func keccak256(_ data: Data) -> Data {
        let digest = SHA3(variant: .keccak256)
        let hash = digest.calculate(for: [UInt8](data))
        return Data(hash)
    }
    
}
