import { ethers } from "hardhat";

async function main() {
    const Warehouse = await ethers.getContractFactory("Warehouse");
    const warehouse = await Warehouse.deploy();

    await warehouse.deployed();

    console.log(`Deployed Mattress Warehouse at: ${warehouse.address}`);
}

// We recommend this pattern to be able to use async/await everywhere
// and properly handle errors.
main().catch((error) => {
    console.error(error);
    process.exitCode = 1;
});
