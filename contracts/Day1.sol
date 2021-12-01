pragma solidity ^0.8.0;

contract Day1 {
    uint32[] list;
    function addInput(uint32 item) public {
        list.push(item);
    }

    function getDay1_1Res() public view returns (uint32) {
        uint32 goodIFeel = 0;
        uint count = 0;

        for (uint i = 1; i < list.length; ++i) {
            uint prev = list[i - 1];
            uint curr = list[i];

            if (prev < curr) {
                count++;
            }
        }

        return uint32(count);
    }

    function getDay1_2Res() public view returns (uint32) {
        uint count = 0;

        uint prev = list[0];
        for (uint i = 3; i < list.length; ++i) {
            if (prev < list[i]) {
                count++;
            }
            prev = list[i - 2];
        }

        return uint32(count);
    }
}
