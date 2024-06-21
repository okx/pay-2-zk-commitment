//
//  ZKSimpleAppApp.swift
//  ZKSimpleApp
//
//  Created by Quach Ha Chan Thanh on 13/6/24.
//

import SwiftUI

@main
struct ZKSimpleAppApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) var appDelegate
    
    var body: some Scene {
        WindowGroup {
            ContentView(viewModel: .init())
        }
    }
}
