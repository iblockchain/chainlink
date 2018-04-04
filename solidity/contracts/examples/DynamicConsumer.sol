pragma solidity ^0.4.18;

import "../Chainlinked.sol";

contract DynamicConsumer is Chainlinked {
  uint256 private requestId;
  bytes32 public currentPrice;

  function DynamicConsumer(address _oracle) public {
    setOracle(_oracle);
  }

  function requestEthereumPrice(bytes32 _currency) public {
    ChainlinkLib.Run memory run = newRun("someJobId", this, "fulfill(uint256,bytes32)");
    run.add("url", "https://etherprice.com/api");
    bytes32[] memory path = new bytes32[](2);
    path[0] = "recent";
    path[1] = _currency;
    run.addBytes32Array("path", path);
    requestId = chainlinkRequest(run);
  }

  function fulfill(uint256 _requestId, bytes32 _data)
    public
    onlyOracle
    checkRequestId(_requestId)
  {
    currentPrice = _data;
  }

  modifier checkRequestId(uint256 _requestId) {
    require(requestId == _requestId);
    _;
  }

}