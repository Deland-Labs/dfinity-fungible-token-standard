import "../setup"
import {canister} from "../utils";
import {ReInstallOptions} from "~/utils/canister";
import {DFTInitOptions} from "../../tasks";
import {parseToCommon} from "~/utils/uint";
import BigNumber from "bignumber.js";
import logger from "node-color-log";

const build = () => {
    canister.build("dft_tx_storage");
}

const reinstall_by_dfx = async () => {
    const default_dft_id = "rrkah-fqaaa-aaaaa-aaaaq-cai"
    await canister.reinstall("dft_tx_storage", `'(principal "${default_dft_id}",1:nat)'`);
}

export const reinstall = async (options?: ReInstallOptions) => {
    if (options?.build) {
        build();
    }
    await reinstall_by_dfx();
}