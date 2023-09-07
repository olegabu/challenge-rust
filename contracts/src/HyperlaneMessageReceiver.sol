// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.13;

import "hyperlane-quickstart/HyperlaneMessageReceiver.sol";

contract Receiver is HyperlaneMessageReceiver {
    constructor(address _mailbox) HyperlaneMessageReceiver(_mailbox) {}
}
