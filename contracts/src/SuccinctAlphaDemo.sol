// SPDX-License-Identifier: MIT
pragma solidity ^0.8.16;

interface ISuccinctGateway {
    function requestCallback(
        bytes32 _functionId,
        bytes memory _input,
        bytes memory _context,
        bytes4 _callbackSelector,
        uint32 _callbackGasLimit
    ) external payable returns (bytes32);

    function requestCall(
        bytes32 _functionId,
        bytes memory _input,
        address _address,
        bytes memory _data,
        uint32 _gasLimit
    ) external payable;

    function verifiedCall(
        bytes32 _functionId,
        bytes memory _input
    ) external view returns (bytes memory);

    function isCallback() external view returns (bool);
}

contract SuccinctAlphaDemo {
    /// @notice The gateway contract address.
    address public gateway = 0x6e4f1e9eA315EBFd69d18C2DB974EEf6105FB803;

    /// @notice The owner of the contract.
    address public owner;

    constructor() {
        owner = msg.sender;
    }

    modifier onlyOwner() {
        require(msg.sender == owner, "Caller is not the owner");
        _;
    }

    modifier requiresMinimumEther() {
        require(msg.value >= 0.01 ether, "Minimum 0.05 ETH not sent");
        _;
    }

    event HandledRequestProofCallback(
        bytes32 indexed functionId,
        address indexed sender,
        bytes output
    );

    /// @notice Request a proof from the gateway contract for the demo.
    /// @param _functionId Your function id.
    /// @param _blockRoot The block root used as the root of trust for the oracle.
    function requestProof(
        bytes32 _functionId,
        bytes32 _blockRoot
    ) external payable requiresMinimumEther {
        ISuccinctGateway(gateway).requestCallback(
            _functionId,
            abi.encodePacked(_blockRoot),
            abi.encodePacked(_functionId, msg.sender),
            this.handleRequestProofCallback.selector,
            200000
        );
    }

    /// @notice Callback function for the gateway contract.
    /// @param _output The output of the callback.
    /// @param _context The context of the callback.
    function handleRequestProofCallback(
        bytes memory _output,
        bytes memory _context
    ) public {
        require(
            msg.sender == gateway && ISuccinctGateway(gateway).isCallback()
        );

        bytes32 functionId;
        address sender;
        assembly {
            functionId := mload(add(_context, 0x20))
            sender := shr(96, mload(add(add(_context, 0x20), 0x20)))
        }

        emit HandledRequestProofCallback(functionId, sender, _output);
    }

    function withdraw() external onlyOwner {
        payable(owner).transfer(address(this).balance);
    }
}
