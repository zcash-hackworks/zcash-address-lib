import zcash_address


// extension Network: Equatable {
//     static func ==(lhs: Network, rhs: Network) -> Bool {
//         switch lhs {
//         case .main:
//             switch rhs {
//             case .main:
//                 return true
//             default:
//                 return false
//             }
//         case .test: 
//         switch rhs {
//             case .test:
//                 return true
//             default:
//                 return false
//             }
//         case .regtest:
//             switch rhs {
//             case .regtest:
//                 return true
//             default:
//                 return false
//             }    
//         }
//     }
// }

// let helper = ZcashAddressHelper()

// do {
//     let testSaplingAddress = "ztestsapling1ysrf4uq52n5hhj0vzxpcfneszlk8flalh3ajdwsyucnpc0fjktp9afzcclnxdrnzfl7w7wyx3kz"
//     let parsedAddress  = try helper.parse(testSaplingAddress)
//     // assert(parsedAddress.network == .test)
//     assert(true)
// } catch {
//     fatalError("Invalid Address when it should have been valid")
// }

