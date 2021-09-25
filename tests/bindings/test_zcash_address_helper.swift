import zcash_address

extension zcash_address.Address: Equatable {}

do {
    let testSaplingAddress = "ztestsapling1ysrf4uq52n5hhj0vzxpcfneszlk8flalh3ajdwsyucnpc0fjktp9afzcclnxdrnzfl7w7wyx3kz"
    let parsedAddress  = try zcash_address.parse(testSaplingAddress)
    // assert(parsedAddress.network == .test)
    assert(true)
} catch {
    fatalError("Invalid Address when it should have been valid")
}

